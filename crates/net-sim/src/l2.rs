//! L2 switches: MAC learning, flood, and unicast forward.

use networkscraft_net_model::{DeviceKind, Ipv4Addr, LinkId, MacAddr, NodeId, Topology, node_mac};

use crate::{MAX_EVENTS, SimEvent};

/// MAC entries learned per switch.
pub const MAX_MAC_PER_SWITCH: usize = 512;

/// Frame processing steps per ping (hostile cap).
pub const MAX_L2_STEPS: usize = 2048;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Payload {
    ArpWhoHas { target: Ipv4Addr, sender_ip: Ipv4Addr },
    ArpReply { target_ip: Ipv4Addr, sender_ip: Ipv4Addr },
    IcmpEchoRequest,
    IcmpEchoReply,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Frame {
    dst: MacAddr,
    src: MacAddr,
    payload: Payload,
}

struct SwitchTable {
    /// Destination MAC -> egress link on this switch.
    learned: Vec<(MacAddr, LinkId)>,
}

struct L2State {
    switches: Vec<(NodeId, SwitchTable)>,
    events: Vec<SimEvent>,
    steps: usize,
}

impl L2State {
    fn new(topology: &Topology) -> Self {
        let switches = topology
            .nodes
            .iter()
            .filter(|n| n.kind == DeviceKind::Switch)
            .map(|n| (n.id, SwitchTable { learned: Vec::new() }))
            .collect();
        L2State { switches, events: Vec::new(), steps: 0 }
    }

    fn push(&mut self, summary: String, link: Option<LinkId>) {
        if self.events.len() >= MAX_EVENTS {
            return;
        }
        self.events.push(SimEvent {
            seq: self.events.len() as u32 + 1,
            summary,
            link_id: link.map(|l| l.0),
        });
    }

    fn table_mut(&mut self, switch: NodeId) -> Option<&mut SwitchTable> {
        self.switches.iter_mut().find(|(id, _)| *id == switch).map(|(_, t)| t)
    }

    fn learn(&mut self, switch: NodeId, mac: MacAddr, egress_link: LinkId) {
        let Some(table) = self.table_mut(switch) else { return };
        if table.learned.iter().any(|(m, _)| *m == mac) {
            return;
        }
        if table.learned.len() >= MAX_MAC_PER_SWITCH {
            return;
        }
        table.learned.push((mac, egress_link));
        self.push(format!("SW{} learned {} on link {}", switch.0, mac, egress_link.0), Some(egress_link));
    }

    fn egress_for(&self, switch: NodeId, mac: MacAddr) -> Option<LinkId> {
        let (_, table) = self.switches.iter().find(|(id, _)| *id == switch)?;
        table.learned.iter().find(|(m, _)| *m == mac).map(|(_, link)| *link)
    }
}

/// Deliver ICMP echo using L2 switching (ARP + unicast).
pub fn deliver_ping(topology: &Topology, source: NodeId, dest: NodeId, dest_ip: Ipv4Addr) -> Vec<SimEvent> {
    let mut state = L2State::new(topology);
    let source_ip = topology.node(source).and_then(|n| n.ipv4.as_ref()).and_then(|s| Ipv4Addr::parse(s));
    let Some(source_ip) = source_ip else {
        state.push("ping aborted: source host has no IPv4".into(), None);
        return state.events;
    };

    let source_mac = node_mac(source);
    let dest_mac = node_mac(dest);

    let arp_req = Frame {
        dst: MacAddr::BROADCAST,
        src: source_mac,
        payload: Payload::ArpWhoHas { target: dest_ip, sender_ip: source_ip },
    };
    flood_from_host(topology, &mut state, source, arp_req);

    let arp_rep = Frame {
        dst: source_mac,
        src: dest_mac,
        payload: Payload::ArpReply { target_ip: source_ip, sender_ip: dest_ip },
    };
    unicast_from_host(topology, &mut state, dest, arp_rep);

    let echo_req = Frame {
        dst: dest_mac,
        src: source_mac,
        payload: Payload::IcmpEchoRequest,
    };
    unicast_from_host(topology, &mut state, source, echo_req);

    let echo_rep = Frame {
        dst: source_mac,
        src: dest_mac,
        payload: Payload::IcmpEchoReply,
    };
    unicast_from_host(topology, &mut state, dest, echo_rep);

    state.events
}

fn flood_from_host(topology: &Topology, state: &mut L2State, host: NodeId, frame: Frame) {
    for port in topology.ports(host) {
        state.push(format!("host{} broadcast {} -> {} on link {}", host.0, frame.src, frame.dst, port.link_id.0), Some(port.link_id));
        receive_on_peer(topology, state, port.peer, port.link_id, host, frame);
    }
}

fn unicast_from_host(topology: &Topology, state: &mut L2State, host: NodeId, frame: Frame) {
    let ports = topology.ports(host);
    if ports.len() != 1 {
        state.push(format!("host{} cannot unicast: need exactly one link", host.0), None);
        return;
    }
    let port = ports[0];
    state.push(
        format!("host{} unicast {} -> {} on link {}", host.0, frame.src, frame.dst, port.link_id.0),
        Some(port.link_id),
    );
    receive_on_peer(topology, state, port.peer, port.link_id, host, frame);
}

fn receive_on_peer(topology: &Topology, state: &mut L2State, device: NodeId, via_link: LinkId, from_peer: NodeId, frame: Frame) {
    if state.steps >= MAX_L2_STEPS {
        return;
    }
    state.steps += 1;

    let kind = topology.node(device).map(|n| n.kind);
    match kind {
        Some(DeviceKind::Switch) => forward_switch(topology, state, device, via_link, from_peer, frame),
        Some(DeviceKind::Host) => deliver_host(state, device, via_link, frame),
        Some(DeviceKind::Router) => state.push(format!("router{} dropped L2 frame", device.0), Some(via_link)),
        None => {}
    }
}

fn forward_switch(topology: &Topology, state: &mut L2State, switch: NodeId, ingress_link: LinkId, _ingress_peer: NodeId, frame: Frame) {
    state.learn(switch, frame.src, ingress_link);

    let known = state.egress_for(switch, frame.dst);
    let flood = frame.dst.is_broadcast() || known.is_none();
    if flood {
        state.push(format!("SW{} floods {} -> {}", switch.0, frame.src, frame.dst), Some(ingress_link));
        for port in topology.ports(switch) {
            if port.link_id == ingress_link {
                continue;
            }
            receive_on_peer(topology, state, port.peer, port.link_id, switch, frame);
        }
        return;
    }

    let Some(egress) = known else {
        return;
    };
    state.push(format!("SW{} forwards unicast to link {}", switch.0, egress.0), Some(egress));
    let Some(port) = topology.ports(switch).into_iter().find(|p| p.link_id == egress) else {
        return;
    };
    receive_on_peer(topology, state, port.peer, port.link_id, switch, frame);
}

fn deliver_host(state: &mut L2State, host: NodeId, via_link: LinkId, frame: Frame) {
    let summary = match frame.payload {
        Payload::ArpWhoHas { target, .. } => format!("host{} ARP who-has {}", host.0, target),
        Payload::ArpReply { sender_ip, .. } => format!("host{} ARP reply from {}", host.0, sender_ip),
        Payload::IcmpEchoRequest => format!("host{} ICMP echo request", host.0),
        Payload::IcmpEchoReply => format!("host{} ICMP echo reply", host.0),
    };
    state.push(summary, Some(via_link));
}

#[cfg(test)]
mod tests {
    use super::*;
    use networkscraft_net_model::DeviceKind;

    #[test]
    fn switch_learns_mac_on_ping() {
        let mut t = Topology::new("lab");
        let sw = t.add_node(DeviceKind::Switch, "SW1", 0.0, 0.0);
        let pc1 = t.add_node(DeviceKind::Host, "PC1", 0.0, 0.0);
        let pc2 = t.add_node(DeviceKind::Host, "PC2", 0.0, 0.0);
        let _ = t.add_link(pc1, sw);
        let _ = t.add_link(pc2, sw);
        let _ = t.set_ipv4(pc1, "10.0.0.1");
        let _ = t.set_ipv4(pc2, "10.0.0.2");
        let dest = Ipv4Addr::parse("10.0.0.2").unwrap_or(Ipv4Addr::new(10, 0, 0, 2));
        let events = deliver_ping(&t, pc1, pc2, dest);
        assert!(events.iter().any(|e| e.summary.contains("learned")));
        assert!(events.iter().any(|e| e.summary.contains("ICMP echo reply")));
    }
}
