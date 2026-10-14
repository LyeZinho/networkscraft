//! Interface colours for NetworksCraft (original palette, not copied from Cisco/Autodesk).

use egui::{Color32, Theme, Visuals};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ThemePref {
    #[default]
    Dark,
    Light,
}

impl ThemePref {
    pub fn resolve(self) -> Theme {
        match self {
            ThemePref::Dark => Theme::Dark,
            ThemePref::Light => Theme::Light,
        }
    }
}

pub struct Tokens {
    pub chrome: Color32,
    pub panel: Color32,
    pub tab_active: Color32,
    pub border: Color32,
    pub canvas: Color32,
    pub text: Color32,
    pub text_dim: Color32,
    pub accent: Color32,
    pub link: Color32,
    pub node_router: Color32,
    pub node_switch: Color32,
    pub node_host: Color32,
}

impl Tokens {
    pub fn for_theme(theme: Theme) -> Self {
        match theme {
            Theme::Dark => Tokens {
                chrome: Color32::from_rgb(38, 42, 48),
                panel: Color32::from_rgb(32, 36, 42),
                tab_active: Color32::from_rgb(48, 54, 62),
                border: Color32::from_rgb(58, 64, 72),
                canvas: Color32::from_rgb(22, 26, 32),
                text: Color32::from_rgb(230, 232, 235),
                text_dim: Color32::from_rgb(160, 168, 178),
                accent: Color32::from_rgb(56, 132, 202),
                link: Color32::from_rgb(90, 160, 90),
                node_router: Color32::from_rgb(220, 140, 60),
                node_switch: Color32::from_rgb(80, 170, 220),
                node_host: Color32::from_rgb(180, 200, 120),
            },
            Theme::Light => Tokens {
                chrome: Color32::from_rgb(240, 242, 245),
                panel: Color32::from_rgb(248, 249, 251),
                tab_active: Color32::from_rgb(255, 255, 255),
                border: Color32::from_rgb(200, 204, 210),
                canvas: Color32::from_rgb(252, 252, 253),
                text: Color32::from_rgb(30, 34, 40),
                text_dim: Color32::from_rgb(100, 108, 118),
                accent: Color32::from_rgb(30, 100, 180),
                link: Color32::from_rgb(40, 120, 60),
                node_router: Color32::from_rgb(180, 100, 30),
                node_switch: Color32::from_rgb(40, 130, 190),
                node_host: Color32::from_rgb(100, 140, 50),
            },
        }
    }
}

pub fn apply(ctx: &egui::Context, theme: Theme) {
    let mut visuals = Visuals::from_theme(theme);
    let t = Tokens::for_theme(theme);
    visuals.panel_fill = t.panel;
    visuals.window_fill = t.panel;
    visuals.extreme_bg_color = t.chrome;
    ctx.set_visuals(visuals);
}
