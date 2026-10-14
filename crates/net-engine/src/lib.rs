//! Engine shell: one lab [`Session`], JSON commands and a simple command line.

#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
#![forbid(unsafe_code)]

mod error;

pub use error::Error;
use networkscraft_net_model::{DeviceKind, NodeId, Topology};
use serde_json::{Value, json};

#[derive(Clone, Debug, Default)]
pub struct Session {
    pub topology: Topology,
    log: Vec<String>,
    selected: Option<NodeId>,
}

impl Session {
    pub fn with_sample_lab() -> Self {
        let mut topology = Topology::new("Sample lab");
        let r = topology.add_node(DeviceKind::Router, "R1", 120.0, 80.0);
        let s = topology.add_node(DeviceKind::Switch, "SW1", 280.0, 80.0);
        let h = topology.add_node(DeviceKind::Host, "PC1", 440.0, 80.0);
        let _ = topology.add_link(r, s);
        let _ = topology.add_link(s, h);
        Session { topology, ..Session::default() }
    }

    pub fn echo(&mut self, line: impl Into<String>) {
        self.log.push(line.into());
    }

    pub fn log_tail(&self, n: usize) -> &[String] {
        let start = self.log.len().saturating_sub(n);
        &self.log[start..]
    }

    pub fn selected(&self) -> Option<NodeId> {
        self.selected
    }

    pub fn set_selected(&mut self, id: Option<NodeId>) {
        self.selected = id;
    }

    pub fn execute(&mut self, command: &str, params: &Value) -> Result<Value, Error> {
        match command {
            "newlab" => {
                let title = params.get("title").and_then(Value::as_str).unwrap_or("Untitled lab");
                self.topology = Topology::new(title);
                self.selected = None;
                Ok(json!({ "title": self.topology.title }))
            }
            "adddevice" => {
                let kind = params
                    .get("kind")
                    .and_then(Value::as_str)
                    .and_then(parse_kind)
                    .ok_or_else(|| Error::msg("kind must be router, switch or host"))?;
                let name = params.get("name").and_then(Value::as_str).unwrap_or(kind.label());
                let x = params.get("x").and_then(Value::as_f64).unwrap_or(0.0) as f32;
                let y = params.get("y").and_then(Value::as_f64).unwrap_or(0.0) as f32;
                let id = self.topology.add_node(kind, name, x, y);
                Ok(json!({ "nodeId": id.0 }))
            }
            "select" => {
                let id = params.get("nodeId").and_then(Value::as_u64).map(NodeId);
                if let Some(nid) = id
                    && self.topology.node(nid).is_none()
                {
                    return Err(Error::msg("unknown nodeId"));
                }
                self.selected = id;
                Ok(json!({ "selected": self.selected.map(|n| n.0) }))
            }
            "ping" => {
                let target = params.get("target").and_then(Value::as_str).unwrap_or("?");
                self.echo(format!("ping {target}: not available yet (simulation pending)"));
                Ok(json!({ "message": "simulation not available yet" }))
            }
            other => Err(Error::msg(format!("unknown command: {other}"))),
        }
    }

    /// Minimal command line: `ping …`, `select <id>`, `adddevice router R2 10 20`.
    pub fn cmdline(&mut self, text: &str) -> Result<(), Error> {
        let text = text.trim();
        if text.is_empty() {
            return Ok(());
        }
        self.echo(format!("> {text}"));
        let mut parts = text.split_whitespace();
        let Some(first) = parts.next() else { return Ok(()) };
        let cmd = first.to_ascii_lowercase();
        match cmd.as_str() {
            "ping" => {
                let target = parts.next().unwrap_or("?");
                self.execute("ping", &json!({ "target": target }))?;
            }
            "select" => {
                let id = parts.next().ok_or_else(|| Error::msg("usage: select <nodeId>"))?;
                let id = id.parse::<u64>().map_err(|_| Error::msg("nodeId must be a number"))?;
                self.execute("select", &json!({ "nodeId": id }))?;
            }
            "adddevice" => {
                let kind = parts.next().ok_or_else(|| Error::msg("usage: adddevice <kind> [name] [x y]"))?;
                let name = parts.next();
                let x = parts.next().and_then(|s| s.parse().ok());
                let y = parts.next().and_then(|s| s.parse().ok());
                let mut p = json!({ "kind": kind });
                if let Some(n) = name.filter(|n| !n.chars().all(|c| c.is_ascii_digit() || c == '.' || c == '-')) {
                    p["name"] = json!(n);
                }
                if let (Some(x), Some(y)) = (x, y) {
                    p["x"] = json!(x);
                    p["y"] = json!(y);
                }
                self.execute("adddevice", &p)?;
            }
            _ => return Err(Error::msg(format!("unknown command: {cmd}"))),
        }
        Ok(())
    }
}

fn parse_kind(s: &str) -> Option<DeviceKind> {
    match s.to_ascii_lowercase().as_str() {
        "router" | "r" => Some(DeviceKind::Router),
        "switch" | "sw" => Some(DeviceKind::Switch),
        "host" | "pc" => Some(DeviceKind::Host),
        _ => None,
    }
}
