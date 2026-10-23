//! Simplified L2/L3 simulation for education labs (deterministic, bounded).

#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
#![forbid(unsafe_code)]

mod l2;

use networkscraft_net_model::{DeviceKind, Ipv4Addr, NodeId, Topology};
use serde::{Deserialize, Serialize};

pub use l2::deliver_ping;

/// Maximum BFS steps when walking a topology (hostile graph protection).
pub const MAX_GRAPH_VISITS: usize = 4096;

/// Maximum events one operation may emit.
pub const MAX_EVENTS: usize = 256;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimEvent {
    pub seq: u32,
    pub summary: String,
    pub link_id: Option<u64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PingOutcome {
    Success { rtt_ms: u32 },
    Unreachable,
    NoRoute,
    NotHost,
    UnknownTarget,
    DifferentSubnet,
}

impl PingOutcome {
    pub fn label(self) -> &'static str {
        match self {
            PingOutcome::Success { .. } => "success",
            PingOutcome::Unreachable => "unreachable",
            PingOutcome::NoRoute => "no route",
            PingOutcome::NotHost => "not a host",
            PingOutcome::UnknownTarget => "unknown host",
            PingOutcome::DifferentSubnet => "different L2 domain",
        }
    }
}

/// ICMP echo between two hosts on the same L2 domain (switches extend the domain; routers do not).
pub fn ping(topology: &Topology, source: NodeId, dest: Ipv4Addr) -> (PingOutcome, Vec<SimEvent>) {
    let source_node = topology.node(source);
    match source_node {
        Some(n) if n.kind == DeviceKind::Host => {}
        _ => return (PingOutcome::NotHost, Vec::new()),
    }

    let dest_host = topology.host_by_ipv4(&dest);
    let dest_id = match dest_host {
        Some(n) => n.id,
        None => return (PingOutcome::UnknownTarget, Vec::new()),
    };

    if dest_id == source {
        let mut events = Vec::new();
        push_event(&mut events, "ICMP echo request (loopback)", None);
        push_event(&mut events, "ICMP echo reply", None);
        return (PingOutcome::Success { rtt_ms: 1 }, events);
    }

    if !same_l2_domain(topology, source, dest_id) {
        let mut events = Vec::new();
        push_event(
            &mut events,
            "no path: hosts are not in the same L2 domain (routing not modeled yet)",
            None,
        );
        return (PingOutcome::DifferentSubnet, events);
    }

    if !hosts_reachable(topology, source, dest_id) {
        let mut events = Vec::new();
        push_event(&mut events, "no L2 path between hosts", None);
        return (PingOutcome::Unreachable, events);
    }

    let events = deliver_ping(topology, source, dest_id, dest);
    let hops = topology
        .ports(source)
        .len()
        .saturating_add(topology.ports(dest_id).len()) as u32;
    let rtt = 2 + hops;
    (PingOutcome::Success { rtt_ms: rtt }, events)
}

fn push_event(events: &mut Vec<SimEvent>, summary: &str, link_id: Option<u64>) {
    if events.len() >= MAX_EVENTS {
        return;
    }
    events.push(SimEvent {
        seq: events.len() as u32 + 1,
        summary: summary.to_string(),
        link_id,
    });
}

/// Switches forward at L2; routers block a single broadcast domain for now.
fn same_l2_domain(topology: &Topology, a: NodeId, b: NodeId) -> bool {
    if a == b {
        return true;
    }
    let mut queue = vec![a];
    let mut seen = vec![a];
    let mut visits = 0usize;
    while let Some(cur) = queue.pop() {
        visits += 1;
        if visits > MAX_GRAPH_VISITS {
            return false;
        }
        if cur == b {
            return true;
        }
        for next in topology.neighbours(cur) {
            if seen.contains(&next) {
                continue;
            }
            let kind = topology.node(next).map(|n| n.kind);
            if kind == Some(DeviceKind::Router) {
                continue;
            }
            seen.push(next);
            queue.push(next);
        }
    }
    false
}

fn hosts_reachable(topology: &Topology, from: NodeId, to: NodeId) -> bool {
    same_l2_domain(topology, from, to)
}

#[cfg(test)]
mod tests {
    use super::*;
    use networkscraft_net_model::DeviceKind;

    #[test]
    fn two_hosts_on_one_switch_ping() {
        let mut t = Topology::new("lab");
        let sw = t.add_node(DeviceKind::Switch, "SW1", 0.0, 0.0);
        let pc1 = t.add_node(DeviceKind::Host, "PC1", 0.0, 0.0);
        let pc2 = t.add_node(DeviceKind::Host, "PC2", 0.0, 0.0);
        let _ = t.add_link(pc1, sw);
        let _ = t.add_link(pc2, sw);
        let _ = t.set_ipv4(pc1, "10.0.0.1");
        let _ = t.set_ipv4(pc2, "10.0.0.2");
        let dest = Ipv4Addr::parse("10.0.0.2").unwrap_or(Ipv4Addr::new(10, 0, 0, 2));
        let (outcome, events) = ping(&t, pc1, dest);
        assert!(matches!(outcome, PingOutcome::Success { .. }));
        assert!(events.iter().any(|e| e.summary.contains("learned")));
    }

    #[test]
    fn router_between_hosts_fails_l2() {
        let mut t = Topology::new("lab");
        let r = t.add_node(DeviceKind::Router, "R1", 0.0, 0.0);
        let pc1 = t.add_node(DeviceKind::Host, "PC1", 0.0, 0.0);
        let pc2 = t.add_node(DeviceKind::Host, "PC2", 0.0, 0.0);
        let _ = t.add_link(pc1, r);
        let _ = t.add_link(r, pc2);
        let _ = t.set_ipv4(pc1, "10.0.0.1");
        let _ = t.set_ipv4(pc2, "10.0.0.2");
        let dest = Ipv4Addr::parse("10.0.0.2").unwrap_or(Ipv4Addr::new(10, 0, 0, 2));
        let (outcome, _) = ping(&t, pc1, dest);
        assert_eq!(outcome, PingOutcome::DifferentSubnet);
    }
}
