# NetworksCraft roadmap

**Stage: pre-alpha → M1 (simulation started)** · **Target:** open-source multi-vendor network lab
simulator (education-first, agent-drivable, native + WASM)

> **Last reviewed:** 2026-10-23 · **Last updated:** 2026-10-23 · **Change:** `net-sim` crate,
> L2-domain ICMP ping, packet log wired to engine

NetworksCraft aims to replace closed, single-vendor lab tools with a **fast, free, Rust-native**
simulator: drag-and-drop topology, device CLIs, visible packet flow, and the same **command +
MCP + CLI** automation model as the other Crafting Apps.

**Reference behaviour (black-box only):** Cisco Packet Tracer (education), GNS3/EVE-NG (realism
for advanced users). **Not a goal:** binary IOS emulation or proprietary asset reuse.

Deep milestones, gates, and hour estimates: [docs/roadmap.md](docs/roadmap.md). Day-to-day work
list: [docs/gaps.md](docs/gaps.md). Migration from legacy CAD code:
[docs/networkscraft-skeleton.md](docs/networkscraft-skeleton.md).

## Headline numbers (estimated)

| Number | Value | Kind |
|--------|-------|------|
| Stage | **Pre-alpha / early M1** | Skeleton UI + first sim slice |
| Ready for classroom labs | **≈ 8%** | Estimated |
| Ready for CCNA-style static labs | **≈ 2%** | L3 routing not started |
| Remaining to **alpha** (first usable labs) | **≈ 100–180 h** | Estimated |
| Remaining to **beta** (shareable curriculum) | **≈ 380–620 h** | Estimated |
| Remaining to **1.0** (broad education parity) | **≈ 880–1,350 h** | Estimated |

Hours are agent wall-clock order-of-magnitude. Simulation depth (routing, VLANs, ACLs) dominates
the calendar after the first ping slice.

## Stage summary

| Stage | User-visible bar | Status |
|-------|------------------|--------|
| **M0 — Skeleton** | Window, docking panels, sample topology, commands | **Mostly done** |
| **M1 — Alpha** | Static IP, ping on L2, save/load lab, switch MAC table | **In progress** |
| **M2 — Beta** | VLANs, OSPF (single area), ACLs, packet animation, CLI dialect | Not started |
| **M3 — 1.0** | Curriculum pack, WASM, MCP/CLI, stable file format, i18n core | Not started |
| **M4 — Beyond** | BGP labs, wireless/AP mode, container bridge, import from other tools | Future |

## What works today

| Area | Status |
|------|--------|
| Desktop app (`networkscraft`) | Minimal eframe host |
| Docking UI (Devices, Topology, Terminal, Packet log, Properties) | Default layout + drag splits |
| Topology model | Nodes, links, IPv4 on hosts, neighbours graph |
| **`net-sim`** | L2 switches: MAC learn, flood, unicast forward; ARP/ICMP delivery between hosts |
| Commands | `newlab`, `adddevice`, `select`, `setipv4`, `ping` (from selected host) |
| Sample lab | R1–SW1–PC1–PC2 with `10.0.0.1` / `10.0.0.2` — ping PC1→PC2 works |
| Packet log panel | Shows last ping’s `SimEvent` list |
| Lab files | Not implemented |
| L2 MAC learning / switch forward | **Basic model** (`net-sim/l2.rs`; one link = one port) |
| L3 routing | Not implemented |
| CLI / MCP / web | Not implemented |
| Legacy CADCraft tree | Still in repo; not part of NetworksCraft product |

### Try it

1. `cargo run --release -p networkscraft`
2. Click **PC1** on the topology (node id **2** in the default sample — check Properties).
3. Terminal: `ping 10.0.0.2`
4. Open **Packet log** for ARP/ICMP lines.

## M1 alpha — current focus (simulation)

| Work item | Status |
|-----------|--------|
| `net-sim` crate + tests | **Done (ping slice)** |
| Wire `ping` to engine + packet log | **Done** |
| IPv4 parse + `setipv4` | **Done** |
| Switch MAC table + frame forward | **Done (initial)** |
| Run/Pause sim clock | Not started |
| Lab file v1 save/load | Not started |
| `networkscraft-cli` headless ping | Not started |

**Alpha gate (unchanged):** two PCs on a switch, assign IPs, ping succeeds; save/open lab; headless
CLI; no panics on hostile input.

## Upcoming (ranked)

1. **Lab I/O:** JSON schema, New/Open/Save, sample labs in repo or corpus.
3. **Sim controls:** Run/Pause, reset; engine `sim.step` for future events.
4. **Topology editing:** drag nodes, link tool, delete, undo.
5. **Router static routes:** ping across subnets through R1.
6. **Device CLI** (generic dialect) tied to selected node.
7. **Control channel + CLI + MCP** for automation.
8. **CI / README** focused on NetworksCraft; retire CADCraft from default build when ready.

## Progress log

| Date | Entry |
|------|--------|
| 2026-10-23 | L2 switch MAC learning + flood/forward in `net-sim/l2.rs`; topology `ports()` / `MacAddr`. |
| 2026-10-23 | `net-sim`: same-L2 ping with ARP/ICMP events; sample lab PC1/PC2; packet log panel; `setipv4`; roadmap updated for M1. |
| 2026-10-14 | NetworksCraft skeleton: `net-model`, `net-engine`, `net-ui-egui`, `apps/networkscraft`; Panel layout; migration doc; initial roadmap. |

## Revision history

| Date | Change | Summary |
|------|--------|---------|
| 2026-10-23 | minor | M1 simulation started; headline numbers and “what works” refreshed |
| 2026-10-14 | major | Initial NetworksCraft ROADMAP |
