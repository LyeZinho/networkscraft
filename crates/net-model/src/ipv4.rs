//! IPv4 addresses for lab devices (parsing and comparison only).

use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Ipv4Addr(pub [u8; 4]);

impl Ipv4Addr {
    pub const fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        Ipv4Addr([a, b, c, d])
    }

    pub fn parse(s: &str) -> Option<Ipv4Addr> {
        let s = s.trim();
        if s.is_empty() || s.len() > 15 {
            return None;
        }
        let mut parts = [0u8; 4];
        let mut idx = 0usize;
        for segment in s.split('.') {
            if idx >= 4 {
                return None;
            }
            if segment.is_empty() || segment.len() > 3 {
                return None;
            }
            let mut value = 0u16;
            for ch in segment.chars() {
                if !ch.is_ascii_digit() {
                    return None;
                }
                value = value * 10 + u16::from(ch as u8 - b'0');
                if value > 255 {
                    return None;
                }
            }
            parts[idx] = value as u8;
            idx += 1;
        }
        if idx != 4 {
            return None;
        }
        Some(Ipv4Addr(parts))
    }

    pub fn octets(self) -> [u8; 4] {
        self.0
    }
}

impl fmt::Display for Ipv4Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}.{}", self.0[0], self.0[1], self.0[2], self.0[3])
    }
}
