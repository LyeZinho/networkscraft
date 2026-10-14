# NetworksCraft — skeleton and migration map

This repository (`networkscraft`) started from **CADCraft**. Legacy CAD code remains until it is
removed in phases. The target product is an **open-source, multi-vendor network lab simulator**
(in the spirit of Packet Tracer, without vendor lock-in). The new skeleton lives in
`networkscraft-*` crates and apps.

See [ROADMAP.md](../ROADMAP.md) and [docs/roadmap.md](roadmap.md) for the full product plan.

## What exists today (NetworksCraft)

| Path | Package | Role |
|------|---------|------|
| `crates/net-model/` | `networkscraft-net-model` | Topology: nodes, links, addresses (L0) |
| `crates/net-engine/` | `networkscraft-net-engine` | Session, commands, command line (L4) |
| `crates/net-ui-egui/` | `networkscraft-net-ui-egui` | egui UI + `craft-ui` docking (L5) |
| `apps/networkscraft/` | `networkscraft` | Minimal desktop host (layer exempt) |

Run the skeleton:

```sh
cargo run --release -p networkscraft
```

## Panels (`Panel`) and default layout

Defined in `crates/net-ui-egui/src/docking.rs`:

| `Panel` | JSON id | Purpose |
|---------|---------|---------|
| `Topology` | `topology` | Topology canvas (protected: cannot close or float) |
| `DevicePalette` | `devicePalette` | Routers, switches, hosts, cables |
| `Terminal` | `terminal` | CLI for the selected device |
| `PacketLog` | `packetLog` | Simulated events / packets |
| `Properties` | `properties` | Selected node or link properties |

Default layout:

```text
+------------------+----------------------------------------+------------------+
|                  |                                        | Properties       |
| Device palette   |           Topology (main)              +------------------+
|                  |                                        | Packet log       |
+------------------+----------------------------------------+------------------+
|                         Terminal (full width)                                |
+------------------------------------------------------------------------------+
```

## File-by-file map — CADCraft → remove / keep / replace

Legend: **R** remove when CAD leaves the repo · **M** keep (shared infra) · **S** replace with
NetworksCraft equivalent · **—** defer (CAD packaging until a Net release)

### Apps

| Path | Action | Notes |
|------|--------|-------|
| `apps/cadcraft/` | R | Replaced by `apps/networkscraft/` (thin eframe host) |
| `apps/cadcraft-cli/` | S | Future `apps/networkscraft-cli/` (headless labs, MCP) |
| `apps/cadcraft-web/` | S | Future WASM host with `NetApp` |
| `apps/networkscraft/` | **new** | Desktop entry for the skeleton |

### CAD crates (remove after full migration)

| Path | Action |
|------|--------|
| `crates/geom/` … `crates/engine/` | R |
| `crates/ui-egui/` | R (port useful patterns into `net-ui-egui`) |
| `crates/mcp/` | S → `networkscraft-mcp` |
| `crates/testkit/` | M or R while CAD tests exist |

### NetworksCraft crates (grow)

| Path | Action | Next content |
|------|--------|--------------|
| `crates/net-model/` | expand | Interfaces, VLANs, sim clock |
| `crates/net-engine/` | expand | `CommandSpec`, sim tick, device CLI |
| `crates/net-ui-egui/` | expand | Topology editing, coded icons, menus |
| `crates/net-sim/` | **create** (L2) | Forwarding, ARP, event queues |
| `crates/net-io/` | **create** (L3) | Lab YAML/JSON, PCAP export |

### Root and tooling

| Path | Action |
|------|--------|
| `Cargo.toml` | M → add `networkscraft-*`; eventually drop `cadcraft-*` |
| `xtask/src/layers.rs` | M → `net-model`, `net-engine`, `net-ui-egui`, `networkscraft` |
| `README.md` | S → NetworksCraft |
| `ROADMAP.md`, `docs/roadmap.md`, `docs/gaps.md` | S → NetworksCraft product docs |
| `examples/*.dxf` | R |
| `.github/workflows/` | M → CI focused on Net crates (`default-members`) |
| `AGENTS.md`, `CLAUDE.md` | S → NetworksCraft agent instructions |

### UI patterns to port from `cadcraft-ui-egui`

| Source | NetworksCraft target |
|--------|----------------------|
| `CadApp`, `run` / `start` / `cmdline` | `NetApp` in `net-ui-egui` |
| `docking.rs` | Already adapted in `net-ui-egui` |
| `theme.rs` | Own tokens (no Autodesk/Cisco artwork) |
| `chrome.rs` | Lab New/Open/Save, Run/Pause simulation |
| `control.rs` | JSON control channel for agents |
| `menus.rs`, `UI_COMMANDS` | Panel toggles, theme, language |
| `icons.rs` | Code-drawn router/switch/PC icons |
| `apps/cadcraft/src/workspace.rs` | Persist docking in `apps/networkscraft` |

### Suggested removal order (avoid breaking CI overnight)

1. Land Net skeleton (`net-*`, `apps/networkscraft`) — **done**.
2. Set `default-members` to NetworksCraft; CI runs `-p networkscraft-*` first.
3. Drop CAD apps from CI matrix; delete CAD crates bottom-up.
4. Rename repo branding; update `repository` in `Cargo.toml`.

## Conventions (ArtCraft / craftrules)

- **Language:** All product documentation, UI strings (English catalog first), code comments,
  commit messages, and agent instructions for this repo are **English**. Other languages use
  the i18n TSV pattern when localization starts (M2+).
- Thin UI: panels read `net-engine::Session`; mutations only through commands.
- Never crash in production: `#![deny(clippy::unwrap_used, …)]`, hostile input caps.
- Clean-room: observe Packet Tracer (and similar) in the UI only; no Cisco IOS binaries,
  icons, or copied help text.
- Layering: `net-model` (L0) → `net-sim` (L2) → `net-io` (L3) → `net-engine` (L4) →
  `net-ui-egui` (L5) → apps.
