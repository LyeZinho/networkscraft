# NetworksCraft roadmap

**Stage: pre-alpha (skeleton)** · **Target:** open-source multi-vendor network lab simulator
(education-first, agent-drivable, native + WASM)

> **Last reviewed:** 2026-10-14 · **Last updated:** 2026-10-14 · **Change:** initial NetworksCraft
> roadmap (repo transitioning from CADCraft legacy)

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
| Stage | **Pre-alpha** | Skeleton UI + empty sim |
| Ready for classroom labs | **≈ 5%** | Estimated |
| Ready for CCNA-style static labs | **≈ 0%** | Needs L2/L3 sim |
| Remaining to **alpha** (first usable labs) | **≈ 120–200 h** | Estimated |
| Remaining to **beta** (shareable curriculum) | **≈ 400–650 h** | Estimated |
| Remaining to **1.0** (broad education parity) | **≈ 900–1,400 h** | Estimated |

Hours are agent wall-clock order-of-magnitude, assuming Craft-app conventions (commands, tests,
no panics). Parallel agents reduce calendar time; simulation depth does not parallelise as well
as UI menus.

## Stage summary

| Stage | User-visible bar | Status |
|-------|------------------|--------|
| **M0 — Skeleton** | Window, docking panels, sample topology, stub commands | **In progress** |
| **M1 — Alpha** | Static IP, ping/traceroute, L2 switch MAC table, save/load lab | Not started |
| **M2 — Beta** | VLANs, OSPF (single area), ACLs, packet animation, CLI dialect | Not started |
| **M3 — 1.0** | Curriculum pack, WASM, MCP/CLI, stable file format, i18n core | Not started |
| **M4 — Beyond** | BGP labs, wireless/AP mode, container bridge, import from other tools | Future |

## What works today

| Area | Status |
|------|--------|
| Desktop app (`networkscraft`) | Minimal eframe host |
| Docking UI (Devices, Topology, Terminal, Packet log, Properties) | Default layout + drag splits |
| Topology model | Nodes (router/switch/host), links, selection |
| Commands | `newlab`, `adddevice`, `select`, `ping` (stub message) |
| Simulation | Not implemented |
| Lab files | Not implemented |
| CLI / MCP / web | Not implemented |
| Legacy CADCraft tree | Still in repo; not part of NetworksCraft product |

## Upcoming (ranked)

1. **Repo focus:** English docs, README for NetworksCraft, CI `default-members` for Net crates.
2. **`net-sim` vertical slice:** ARP + L2 forward + host ping between two PCs on one switch.
3. **Lab I/O:** JSON/YAML schema, New/Open/Save, autosave.
4. **Topology editing:** Drag nodes, link tool, delete, undo.
5. **Device CLI:** Generic dialect (`show ip`, `configure terminal`, hostname, IP on interface).
6. **Packet log:** Timestamped events tied to links; optional simple animation on canvas.
7. **Control channel + CLI + MCP** (parity with CADCraft automation story).
8. **Remove CADCraft crates** once Net CI is green without them.

## Progress log

| Date | Entry |
|------|--------|
| 2026-10-14 | NetworksCraft skeleton: `net-model`, `net-engine`, `net-ui-egui`, `apps/networkscraft`; Panel layout; migration doc; this roadmap. |

## Revision history

| Date | Change | Summary |
|------|--------|---------|
| 2026-10-14 | major | Initial NetworksCraft ROADMAP (replaces CADCraft summary for this repo direction) |
