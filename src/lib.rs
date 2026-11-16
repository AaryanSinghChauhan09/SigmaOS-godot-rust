// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-godot-rust: godot rust component model.
//!
//! Real implementation: rusts, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rust {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GodotRustError {
    EmptyRustName,
    DuplicateRust(String),
    EmptyEventSubject,
}

impl fmt::Display for GodotRustError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyRustName => write!(f, "rust name must not be empty"),
            Self::DuplicateRust(id) => write!(f, "duplicate rust {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for GodotRustError {}

pub struct GodotRust {
    pub name: String,
    rusts: BTreeMap<String, Rust>,
    events: BTreeMap<String, Event>,
}

impl GodotRust {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            rusts: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_rust(&mut self, item: Rust) -> Result<(), GodotRustError> {
        if item.name.trim().is_empty() {
            return Err(GodotRustError::EmptyRustName);
        }
        let id = item.id.clone();
        if self.rusts.contains_key(&id) {
            return Err(GodotRustError::DuplicateRust(id.clone()));
        }
        self.rusts.insert(id, item);
        Ok(())
    }

    pub fn rust_count(&self) -> usize {
        self.rusts.len()
    }

    pub fn get_rust(&self, id: &str) -> Option<&Rust> {
        self.rusts.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), GodotRustError> {
        if event.subject.trim().is_empty() {
            return Err(GodotRustError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_rust(id: &str, name: &str) -> Rust {
        Rust {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = GodotRust::new("GodotRust");
        assert_eq!(m.name, "GodotRust");
        assert_eq!(m.rust_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_rust() {
        let mut m = GodotRust::new("GodotRust");
        m.add_rust(sample_rust("a1", "Alpha")).unwrap();
        assert_eq!(m.rust_count(), 1);
        assert!(m.get_rust("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = GodotRust::new("GodotRust");
        assert!(m.add_rust(sample_rust("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = GodotRust::new("GodotRust");
        m.add_rust(sample_rust("x", "One")).unwrap();
        assert!(m.add_rust(sample_rust("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = GodotRust::new("GodotRust");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
