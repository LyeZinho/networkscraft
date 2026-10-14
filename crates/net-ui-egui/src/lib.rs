//! NetworksCraft egui front end — thin shell over [`networkscraft_net_engine::Session`].

#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
#![forbid(unsafe_code)]

pub mod docking;
mod panels;
pub mod theme;

use networkscraft_net_engine::Session;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const PREFS_KEY: &str = "networkscraft.prefs";

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct UiState {
    pub docking: docking::Workspace,
    pub theme: theme::ThemePref,
    pub show_toolbar: bool,
    pub show_status_bar: bool,
}

impl Default for UiState {
    fn default() -> Self {
        UiState { docking: docking::Workspace::default(), theme: theme::ThemePref::default(), show_toolbar: true, show_status_bar: true }
    }
}

pub struct NetApp {
    pub session: Session,
    pub ui: UiState,
    pub cmd_input: String,
    pub status: Option<String>,
    shown_theme: Option<egui::Theme>,
}

impl NetApp {
    pub fn with_sample() -> Self {
        NetApp {
            session: Session::with_sample_lab(),
            ui: UiState::default(),
            cmd_input: String::new(),
            status: None,
            shown_theme: None,
        }
    }

    pub fn shown_theme(&self) -> egui::Theme {
        self.shown_theme.unwrap_or_else(|| self.ui.theme.resolve())
    }

    pub fn run(&mut self, id: &str, params: Value) -> Result<Value, String> {
        self.session.execute(id, &params).map_err(|e| e.to_string())
    }

    pub fn submit_cmdline(&mut self) {
        let line = std::mem::take(&mut self.cmd_input);
        if line.trim().is_empty() {
            return;
        }
        if let Err(e) = self.session.cmdline(&line) {
            self.session.echo(e.to_string());
        }
    }

    pub fn logic(&mut self, ctx: &egui::Context) {
        let theme = self.ui.theme.resolve();
        if self.shown_theme != Some(theme) {
            theme::apply(ctx, theme);
            self.shown_theme = Some(theme);
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        let t = theme::Tokens::for_theme(self.shown_theme());
        if self.ui.show_toolbar {
            egui::TopBottomPanel::top("nc_toolbar").frame(egui::Frame::NONE.fill(t.chrome)).show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("NetworksCraft");
                    ui.separator();
                    if ui.button("New lab").clicked() {
                        let _ = self.run("newlab", Value::Null);
                    }
                    if ui.button("Sample lab").clicked() {
                        self.session = Session::with_sample_lab();
                    }
                    if ui.button("Reset panels").clicked() {
                        self.ui.docking = docking::Workspace::default();
                    }
                });
            });
        }
        if self.ui.show_status_bar {
            egui::TopBottomPanel::bottom("nc_status").frame(egui::Frame::NONE.fill(t.chrome)).show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let msg = self.status.as_deref().unwrap_or("Ready");
                    ui.label(msg);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(format!("{} nodes", self.session.topology.nodes.len()));
                    });
                });
            });
        }
        egui::CentralPanel::default().frame(egui::Frame::NONE.fill(t.panel)).show(ctx, |ui| {
            docking::show(self, ui);
        });
    }
}
