# NetworksCraft — gaps and work list

**Status:** early skeleton · **Roadmap:** [roadmap.md](roadmap.md) · **Summary:** [ROADMAP.md](../ROADMAP.md)

Ranked work items. Agents and contributors should pick from the **top** of this file unless
[roadmap.md](roadmap.md) marks a milestone gate as blocking.

## Platform and repo

| Priority | Gap | Milestone |
|----------|-----|-----------|
| P0 | English-only product docs and agent instructions | M0 |
| P0 | CI builds `networkscraft` + Net crates on every PR | M0 |
| P1 | `default-members` limited to NetworksCraft workspace | M0 |
| P1 | README describes NetworksCraft (not CADCraft) | M0 |
| P2 | Remove legacy CADCraft crates after Net CI is standalone | Phase D |

## Simulation

| Priority | Gap | Milestone |
|----------|-----|-----------|
| P0 | `net-sim` crate with event queue and tests | M1 — **ping slice landed** |
| P0 | L2 MAC learning and forwarding | M1 — **initial switch model** |
| P0 | Host IPv4 + ARP + ICMP echo | M1 — **event trace + same-L2 ping** |
| P1 | Router static routes | M1 |
| P1 | Simulation Run/Pause and time scale | M1 |
| P2 | OSPF single area | M2 |
| P2 | VLANs and inter-VLAN routing | M2 |
| P2 | ACLs | M2 |

## Model and I/O

| Priority | Gap | Milestone |
|----------|-----|-----------|
| P0 | Lab file format v1 (JSON schema + roundtrip tests) | M1 |
| P0 | New / Open / Save in UI and engine commands | M1 |
| P1 | Interfaces per device (ports, speed, up/down) | M1 |
| P2 | PCAP or structured packet export | M2 |

## UI/UX

| Priority | Gap | Milestone |
|----------|-----|-----------|
| P0 | Persist docking layout (`workspace.json`) | M0 |
| P0 | Code-drawn device icons (router, switch, host) | M0 |
| P1 | Drag nodes; link tool; delete | M2 |
| P1 | Packet log bound to sim events | M1 |
| P1 | Packet highlight on topology links | M2 |
| P2 | Menu bar + keyboard shortcuts | M2 |
| P2 | Device terminal tied to selected node CLI | M2 |

## Engine and CLI

| Priority | Gap | Milestone |
|----------|-----|-----------|
| P1 | `CommandSpec` registry like CADCraft | M1 |
| P1 | Interactive command prompts (cable, IP) | M1 |
| P2 | Generic CLI dialect parser | M2 |
| P2 | `networkscraft-cli` headless runner | M1 |

## Automation

| Priority | Gap | Milestone |
|----------|-----|-----------|
| P2 | JSON control channel | M2 |
| P2 | MCP server | M2 |
| P2 | `inspect_topology` + `capture_packets` tools | M2 |

## Stability

| Priority | Gap | Milestone |
|----------|-----|-----------|
| P0 | `#![deny]` panics on all Net production crates | M0 |
| P1 | `hostile_lab_never_panic` regression tests | M1 |
| P2 | Autosave and recovery | M2 |

## Revision history

| Date | Change |
|------|--------|
| 2026-10-14 | Initial gaps list for NetworksCraft |
| 2026-10-23 | Mark `net-sim` ping slice and ARP/ICMP events in progress on M1 |
