//! Panel bodies (stubs until simulation and topology editing land).

use egui::{Color32, Sense, Stroke, pos2, vec2};
use networkscraft_net_model::DeviceKind;
use serde_json::json;

use crate::NetApp;
use crate::theme::Tokens;

pub fn device_palette(app: &mut NetApp, ui: &mut egui::Ui) {
    let t = Tokens::for_theme(app.shown_theme());
    ui.label(egui::RichText::new("Drag devices onto the topology (coming soon).").color(t.text_dim));
    ui.add_space(8.0);
    for kind in DeviceKind::ALL {
        ui.horizontal(|ui| {
            let color = node_color(t, kind);
            ui.painter().circle_filled(pos2(ui.cursor().left() + 8.0, ui.cursor().top() + 10.0), 6.0, color);
            ui.add_space(20.0);
            if ui.button(format!("Add {}", kind.label())).clicked() {
                let name = format!("{}_{}", kind.label(), app.session.topology.nodes.len() + 1);
                let x = 80.0 + (app.session.topology.nodes.len() as f32 * 40.0);
                let _ = app.run("adddevice", json!({ "kind": kind_label(kind), "name": name, "x": x, "y": 120.0 }));
            }
        });
    }
}

pub fn topology(app: &mut NetApp, ui: &mut egui::Ui) {
    let t = Tokens::for_theme(app.shown_theme());
    ui.painter().rect_filled(ui.max_rect(), 0.0, t.canvas);
    let rect = ui.max_rect().shrink(12.0);
    for link in &app.session.topology.links {
        let Some(a) = app.session.topology.node(link.a) else { continue };
        let Some(b) = app.session.topology.node(link.b) else { continue };
        let pa = node_center(rect, a.x, a.y);
        let pb = node_center(rect, b.x, b.y);
        ui.painter().line_segment([pa, pb], Stroke::new(2.0, t.link));
    }
    for node in &app.session.topology.nodes {
        let center = node_center(rect, node.x, node.y);
        let r = 22.0;
        let selected = app.session.selected() == Some(node.id);
        let fill = node_color(t, node.kind);
        ui.painter().circle_filled(center, r, fill);
        if selected {
            ui.painter().circle_stroke(center, r + 3.0, Stroke::new(2.0, t.accent));
        }
        ui.painter().text(center + vec2(0.0, r + 12.0), egui::Align2::CENTER_TOP, &node.name, egui::FontId::proportional(12.0), t.text);
        let hit = egui::Rect::from_center_size(center, vec2(r * 2.0, r * 2.0 + 20.0));
        let resp = ui.interact(hit, ui.id().with(("node", node.id.0)), Sense::click());
        if resp.clicked() {
            let _ = app.run("select", json!({ "nodeId": node.id.0 }));
        }
    }
    ui.allocate_space(ui.available_size());
}

pub fn terminal(app: &mut NetApp, ui: &mut egui::Ui) {
    let t = Tokens::for_theme(app.shown_theme());
    egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
        for line in app.session.log_tail(200) {
            ui.label(egui::RichText::new(line).monospace().color(t.text_dim));
        }
    });
    ui.separator();
    ui.horizontal(|ui| {
        ui.label(">");
        let resp = ui.add(
            egui::TextEdit::singleline(&mut app.cmd_input)
                .desired_width(f32::INFINITY)
                .font(egui::TextStyle::Monospace)
                .id_salt("terminal_input"),
        );
        if resp.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            app.submit_cmdline();
        }
    });
}

pub fn packet_log(app: &mut NetApp, ui: &mut egui::Ui) {
    let t = Tokens::for_theme(app.shown_theme());
    ui.label(egui::RichText::new("Simulated packets will appear here.").color(t.text_dim));
    ui.separator();
    for line in app.session.log_tail(50).iter().filter(|l| l.contains("ping") || l.starts_with('>')) {
        ui.label(egui::RichText::new(line.as_str()).small().monospace());
    }
}

pub fn properties(app: &mut NetApp, ui: &mut egui::Ui) {
    let t = Tokens::for_theme(app.shown_theme());
    ui.heading(&app.session.topology.title);
    ui.label(format!("Nodes: {}", app.session.topology.nodes.len()));
    ui.label(format!("Links: {}", app.session.topology.links.len()));
    ui.separator();
    match app.session.selected().and_then(|id| app.session.topology.node(id)) {
        Some(n) => {
            ui.label(egui::RichText::new("Selected device").strong());
            ui.label(format!("Name: {}", n.name));
            ui.label(format!("Kind: {}", n.kind.label()));
            ui.label(format!("Id: {}", n.id.0));
            ui.label(format!("Position: {:.0}, {:.0}", n.x, n.y));
        }
        None => ui.label(egui::RichText::new("Click a device on the topology.").color(t.text_dim)),
    }
}

fn node_center(rect: egui::Rect, x: f32, y: f32) -> egui::Pos2 {
    pos2(rect.left() + x.min(rect.width()), rect.top() + y.min(rect.height()))
}

fn node_color(t: Tokens, kind: DeviceKind) -> Color32 {
    match kind {
        DeviceKind::Router => t.node_router,
        DeviceKind::Switch => t.node_switch,
        DeviceKind::Host => t.node_host,
    }
}

fn kind_label(kind: DeviceKind) -> &'static str {
    match kind {
        DeviceKind::Router => "router",
        DeviceKind::Switch => "switch",
        DeviceKind::Host => "host",
    }
}
