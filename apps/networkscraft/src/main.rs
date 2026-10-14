//! NetworksCraft desktop entry point.

#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

use eframe::egui;
use networkscraft_net_ui_egui::NetApp;

struct App {
    net: NetApp,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.net.logic(ctx);
        self.net.ui(ctx);
    }
}

fn main() -> eframe::Result {
    let net = NetApp::with_sample();
    let options = eframe::NativeOptions { viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 800.0]), ..Default::default() };
    eframe::run_native("NetworksCraft", options, Box::new(|_cc| Ok(Box::new(App { net }))))
}
