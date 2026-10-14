//! Panel layout via shared [`craft_ui::docking`].

use craft_ui::docking::{Action, DockArea, DockStyle, Layout, Location, Node, PanelLimits, Permissions, Placement, Zone};
use craft_ui::layout::{SplitAxis, SplitSize};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Panel {
    /// Main topology canvas (protected).
    Topology,
    DevicePalette,
    Terminal,
    PacketLog,
    Properties,
}

impl Panel {
    pub const ALL: &[Self] = &[Self::Topology, Self::DevicePalette, Self::Terminal, Self::PacketLog, Self::Properties];

    pub fn id(self) -> &'static str {
        match self {
            Self::Topology => "topology",
            Self::DevicePalette => "devicePalette",
            Self::Terminal => "terminal",
            Self::PacketLog => "packetLog",
            Self::Properties => "properties",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Topology => "Topology",
            Self::DevicePalette => "Devices",
            Self::Terminal => "Terminal",
            Self::PacketLog => "Packet log",
            Self::Properties => "Properties",
        }
    }

    fn default_side(self) -> Zone {
        match self {
            Self::Topology => Zone::Center,
            Self::DevicePalette => Zone::Left,
            Self::Terminal => Zone::Bottom,
            Self::PacketLog | Self::Properties => Zone::Right,
        }
    }

    fn parse(id: &str) -> Result<Self, String> {
        Self::ALL.iter().copied().find(|p| p.id() == id).ok_or_else(|| format!("Unknown panel: {id}"))
    }
}

fn permissions(panel: &Panel) -> Permissions {
    if *panel == Panel::Topology {
        Permissions::PROTECTED
    } else {
        Permissions::default()
    }
}

fn tabs(panel: Panel) -> Node<Panel> {
    Node::Tabs { panels: vec![panel], active: 0 }
}

fn split(axis: SplitAxis, size: SplitSize, first: Node<Panel>, second: Node<Panel>) -> Node<Panel> {
    Node::Split { axis, size, first: Box::new(first), second: Box::new(second) }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Workspace {
    pub layout: Layout<Panel>,
    pub hidden: Vec<(Panel, Location<Panel>)>,
}

impl Default for Workspace {
    fn default() -> Self {
        // Left palette | (topology + right stack) over terminal.
        let center_column = split(
            SplitAxis::Horizontal,
            SplitSize::FixedSecond(300.0),
            tabs(Panel::Topology),
            Node::Stack {
                entries: vec![
                    craft_ui::docking::StackEntry { panel: Panel::Properties, open: true, height: Some(220.0) },
                    craft_ui::docking::StackEntry { panel: Panel::PacketLog, open: true, height: None },
                ],
            },
        );
        let main = split(
            SplitAxis::Vertical,
            SplitSize::FixedSecond(180.0),
            split(
                SplitAxis::Horizontal,
                SplitSize::FixedFirst(240.0),
                tabs(Panel::DevicePalette),
                center_column,
            ),
            tabs(Panel::Terminal),
        );
        Self { layout: Layout { root: Some(main), floating: Vec::new() }, hidden: Vec::new() }
    }
}

impl Workspace {
    pub fn validate(&self) -> Result<(), String> {
        self.layout.validate().map_err(|e| e.to_string())?;
        if !self.layout.contains(&Panel::Topology) {
            return Err("The layout must include the topology view".into());
        }
        if self.layout.floating.iter().any(|g| g.panels.contains(&Panel::Topology)) {
            return Err("The topology view cannot float".into());
        }
        Ok(())
    }

    pub fn apply(&mut self, action: Action<Panel>) -> Result<(), String> {
        self.validate()?;
        let mut next = self.clone();
        if let Action::Close { panel } = &action {
            let location = next.layout.location(panel).map_err(|e| e.to_string())?;
            next.hidden.retain(|(id, _)| id != panel);
            next.hidden.push((*panel, location));
        } else if let Action::Open { panel, .. } | Action::OpenAt { panel, .. } = &action {
            next.hidden.retain(|(id, _)| id != panel);
        }
        next.layout.apply_with_permissions(action, permissions).map_err(|e| e.to_string())?;
        next.validate()?;
        *self = next;
        Ok(())
    }

    pub fn reveal_panel(&mut self, panel: Panel) -> Result<(), String> {
        if !self.layout.contains(&panel) {
            if let Some((_, loc)) = self.hidden.iter().find(|(id, _)| *id == panel).map(|(_, l)| l.clone()) {
                self.layout.restore(panel, loc).map_err(|e| e.to_string())?;
            } else {
                self.layout
                    .apply_with_permissions(
                        Action::OpenAt {
                            panel,
                            anchor: Panel::Topology,
                            placement: Placement::Split(panel.default_side()),
                        },
                        permissions,
                    )
                    .map_err(|e| e.to_string())?;
            }
        }
        self.apply(Action::Activate { panel })
    }
}

pub fn command(app: &mut crate::NetApp, reset: bool, params: &Value) -> Result<Value, String> {
    let mut workspace = app.ui.docking.clone();
    if reset {
        workspace = Workspace::default();
    } else if let Some(action) = params.get("action") {
        let action: Action<Panel> = serde_json::from_value(action.clone()).map_err(|e| e.to_string())?;
        workspace.apply(action)?;
    } else {
        let operation = params.get("operation").and_then(Value::as_str).ok_or("operation is required")?;
        let panel = Panel::parse(params.get("panel").and_then(Value::as_str).ok_or("panel is required")?)?;
        match operation {
            "open" => workspace.reveal_panel(panel)?,
            "close" => workspace.apply(Action::Close { panel })?,
            "activate" => workspace.apply(Action::Activate { panel })?,
            _ => return Err("operation must be open, close or activate".into()),
        }
    }
    app.ui.docking = workspace;
    Ok(json!({ "docking": app.ui.docking }))
}

pub fn show(app: &mut crate::NetApp, ui: &mut egui::Ui) {
    let theme = app.shown_theme();
    let t = crate::theme::Tokens::for_theme(theme);
    let mut workspace = app.ui.docking.clone();
    if workspace.validate().is_err() {
        workspace = Workspace::default();
    }
    app.ui.docking = workspace.clone();
    let mut style = DockStyle::from_ui(ui);
    style.min_pane = 96.0;
    style.background = t.panel;
    style.tab_background = t.chrome;
    style.active_background = t.tab_active;
    style.text = t.text;
    style.inactive_text = t.text_dim;
    style.border = egui::Stroke::new(1.0, t.border);
    style.accent = t.accent;
    let output = DockArea::new(egui::Id::new("networkscraft-docking")).show_with_limits(
        ui,
        &workspace.layout,
        &style,
        |panel| panel.label().into(),
        permissions,
        panel_limits,
        |ui, panel| match panel {
            Panel::Topology => crate::panels::topology(app, ui),
            Panel::DevicePalette => crate::panels::device_palette(app, ui),
            Panel::Terminal => crate::panels::terminal(app, ui),
            Panel::PacketLog => crate::panels::packet_log(app, ui),
            Panel::Properties => crate::panels::properties(app, ui),
        },
    );
    for action in output.actions {
        if let Err(e) = serde_json::to_value(action).and_then(|a| command(app, false, &json!({ "action": a }))) {
            app.status = Some(format!("Panel change failed: {e}"));
        }
    }
}

fn panel_limits(panel: &Panel) -> PanelLimits {
    let (min, max_w) = match panel {
        Panel::Topology => (egui::vec2(320.0, 200.0), 1_000_000.0),
        Panel::DevicePalette => (egui::vec2(200.0, 120.0), 480.0),
        Panel::Terminal => (egui::vec2(200.0, 100.0), 1_000_000.0),
        Panel::PacketLog | Panel::Properties => (egui::vec2(220.0, 80.0), 720.0),
    };
    PanelLimits { min, max: egui::vec2(max_w, 1_000_000.0) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_layout_is_valid() {
        let ws = Workspace::default();
        ws.validate().expect("default workspace");
        assert!(ws.layout.contains(&Panel::Topology));
        assert!(ws.layout.contains(&Panel::Terminal));
    }
}
