# NetworksCraft — detailed roadmap (beginning to end)

This document is the **full journey** from the current skeleton to a **1.0 education-ready**
product and beyond. It follows the Crafting Apps pattern: thin UI, everything is a command,
automation-first, clean-room, never crash.

**One-page summary:** [ROADMAP.md](../ROADMAP.md)  
**Migration from legacy CAD:** [networkscraft-skeleton.md](networkscraft-skeleton.md)

---

## Vision and success criteria

**Vision:** Anyone can build a network lab, run it locally or in the browser, see packets move,
configure devices in a familiar CLI **without** proprietary lock-in, and let agents/scripts drive
the same UI a human uses.

**1.0 success (education):**

- CCNA-intro scope: topology, IPv4, static routing, VLANs, inter-VLAN routing (L3 switch or
  router-on-a-stick), OSPF single-area, basic ACLs, DHCP/DNS stub or simple server nodes.
- Save/share labs as open files; no cloud account required.
- Desktop (Linux, macOS, Windows) + WASM demo.
- MCP + headless CLI for grading and agent labs.

**Explicit non-goals for 1.0:**

- Full IOS/XE/JunOS CLI parity or binary images.
- Production-scale sim (millions of pps, hardware-in-the-loop).
- Copying Cisco/Autodesk icons, fonts, or documentation text.

---

## Architecture trajectory

```text
Phase 0 (now)     net-model ── net-engine ── net-ui-egui ── apps/networkscraft
Phase 1           + net-sim (event-driven forward plane + basic control plane)
Phase 2           + net-io (lab files, PCAP export, optional Git-friendly diff)
Phase 3           + networkscraft-cli, networkscraft-mcp, networkscraft-web
Phase 4           + net-cli-dialect (personalities: generic, cisco-like, linux-like)
Optional          + net-bridge (export to containers / real NICs) — post-1.0
```

Layering (enforced like CADCraft):

| Layer | Crates | Depends on |
|-------|--------|------------|
| L0 | `net-model` | serde only |
| L2 | `net-sim` | `net-model` |
| L3 | `net-io` | `net-model`, `net-sim` (read-only replay) |
| L4 | `net-engine` | model + sim + io hooks |
| L5 | `net-ui-egui`, `networkscraft-mcp` | engine, egui, craft-ui |
| Apps | `networkscraft`, `networkscraft-cli`, `networkscraft-web` | L5 / engine |

---

## Milestones (M0 → M4)

### M0 — Skeleton (current)

**Goal:** Prove UI shell and command routing; no real simulation.

| Deliverable | Done? |
|-------------|-------|
| `NetApp` + `Session` split | Yes |
| Panel enum + default docking layout | Yes |
| Sample topology (R1–SW1–PC1) | Yes |
| Stub commands (`adddevice`, `select`, `ping`) | Yes |
| English product docs + roadmap | Yes |
| Workspace persist, menus, themed chrome | No |
| Remove or isolate CADCraft from default build | No |

**Exit gate:** `cargo run -p networkscraft` shows all panels; user can add nodes and type commands;
CI builds Net crates on every PR.

**Estimate remaining:** 20–40 h (workspace, icons, lab New/Open placeholders, CI focus).

---

### M1 — Alpha (“first ping”)

**Goal:** Two hosts ping across a switch; lab saves to disk; simulation is deterministic and tested
without UI.

| Work | Details |
|------|---------|
| `net-sim` crate | Discrete-event or fixed time-step; bounded queues |
| L2 switch | MAC learn, flood unknown, unicast forward |
| Host stack | IPv4, ARP, ICMP echo request/reply |
| Cables | Link up/down; one subnet per broadcast domain initially |
| Engine | `sim.step`, `sim.run`, `sim.pause`; wire ping to sim |
| I/O | Lab file v1 (JSON): nodes, links, positions, addresses |
| UI | Run/Pause toolbar; packet log from sim events |
| Tests | Hostile caps; regression “two PC ping” never panics |

**Alpha gate (all must pass):**

1. User builds two PCs + switch, assigns IPs, ping succeeds in UI.
2. Save lab, restart app, open lab, ping still works.
3. Headless: `networkscraft-cli run lab.json --script 'ping …'` returns exit code 0.
4. No `unwrap` in production crates; `cargo xtask ci` green for Net workspace.

**Estimate:** 120–200 h cumulative from M0 exit.

---

### M2 — Beta (classroom-ready)

**Goal:** Instructor can ship a VLAN + OSPF lab; students see packets; CLI feels credible for labs.

| Work | Details |
|------|---------|
| Router model | Static routes, connected routes, OSPF (single area) |
| VLANs | 802.1Q on trunk; SVI or ROAS |
| ACLs | Extended ACL numbered, inbound on interface |
| DHCP / DNS | Simple server node or server role on host |
| CLI | `show ip interface brief`, `show ip route`, `show vlan`, config mode subset |
| Topology UX | Drag-move, link creation tool, snap grid, delete with undo |
| Packet visual | Step/highlight on link; filter in packet log |
| i18n | English UI complete; hooks for pt-BR etc. |
| WASM | Read-only or full sim (decision: full sim if perf allows) |

**Beta gate:**

1. Published “CCNA intro” sample pack (3–5 labs) in separate corpus repo.
2. Agent can complete lab checklist via MCP (place devices, cable, configure, verify ping).
3. 30-minute sim session without memory growth over cap (leak tests).

**Estimate:** +280–450 h from alpha.

---

### M3 — 1.0 (general availability)

**Goal:** Trustworthy tool for schools and self-learners; parity with **core** Packet Tracer
**education** workflows (not every game mode or IoT toy).

| Work | Details |
|------|---------|
| Protocol polish | STP (RSTP minimal), ICMP errors, traceroute |
| Wireless (optional slice) | Simple AP + SSID + one VLAN — if scope confirmed |
| Assessment | Activity wizard or script rubrics via CLI |
| Packaging | Linux AppImage/Flatpak, macOS, Windows installers |
| Docs | User guide, lab authoring guide, command reference |
| Security | Fuzz lab files; cap file size and node count |
| Accessibility | Keyboard navigation in topology; reduced motion |
| Legal | ATTRIBUTION.md for any third-party assets |

**1.0 gate:**

1. External pilot: one course runs full term on NetworksCraft labs.
2. File format frozen v1 with migration story for v2.
3. Zero known crashers on corrupted lab upload (report error, keep app alive).

**Estimate:** +200–350 h from beta.

---

### M4 — Beyond 1.0

Ordered by typical demand; not committed dates.

| Theme | Examples |
|-------|----------|
| Advanced routing | BGP lab mode, policy maps (simplified) |
| Multi-vendor CLI | Personality packs (syntax only, clean-room) |
| Integration | Export config to real devices; optional container attach |
| Performance | SIMD batch forward; larger topologies (100+ nodes cap raised) |
| Collaboration | Live share view-only session (optional server) |
| Mobile | View-only topology (stretch) |

---

## Cross-cutting tracks (whole program)

These run in parallel across milestones.

### Commands and automation

Every user action → `CommandSpec` in `net-engine` with JSON `run` and tests. UI commands in
`UI_COMMANDS`. MCP tools mirror CADCraft: `execute`, `cmdline.input`, `inspect_topology`,
`capture_packets`, `render` (topology PNG).

### Never crash

Session and sim stepping under panic guard; hostile lab files; bounded graph size; regression tests
`hostile_lab_never_panic`.

### Clean-room parity

Maintain `docs/target-app-parity.md` (Packet Tracer education scope) and `docs/gaps.md` work list;
optional black-box screenshots under local `plan/` (gitignored), never committed.

### Quality gates

`cargo xtask ci`: fmt, clippy, tests, layers, assets (when icons exist), wasm build for web app.

---

## Repository cleanup phases (legacy CADCraft)

| Phase | Action |
|-------|--------|
| A | Net crates are primary; document legacy |
| B | `default-members = ["crates/net-model", …, "apps/networkscraft"]` |
| C | CI stops building `cadcraft` unless `CI_LEGACY_CAD=1` |
| D | Delete CAD crates and apps |
| E | Rename root README, icons, packaging to NetworksCraft |

---

## Rough timeline scenarios

Assuming 1–2 focused contributors or agents:

| Target | Calendar (indicative) |
|--------|------------------------|
| M0 complete | +2–4 weeks |
| M1 alpha | +2–4 months |
| M2 beta | +6–9 months from start |
| M3 1.0 | +12–18 months from start |

Calendar varies with scope cuts (e.g. defer wireless, defer OSPF to post-beta).

---

## How to use this doc

- Pick work from the **current milestone** exit gate rows first.
- When a gate passes, update [ROADMAP.md](../ROADMAP.md) stage and progress log.
- Split large rows in [gaps.md](gaps.md) (create when M1 starts) for day-to-day tasks.

---

## Revision history

| Date | Change |
|------|--------|
| 2026-10-14 | Initial end-to-end roadmap for NetworksCraft |
