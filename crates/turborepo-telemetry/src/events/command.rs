use serde::{Deserialize, Serialize};
use turborepo_vercel_api::{TelemetryCommandEvent, TelemetryEvent};
use uuid::Uuid;

use super::{Event, EventBuilder, EventType, PubEventBuilder};
use crate::{config::TelemetryConfig, telem};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandEventBuilder {
    id: String,
    command: String,
    parent: Option<String>,
}

impl EventBuilder<CommandEventBuilder> for CommandEventBuilder {
    fn get_id(&self) -> &String {
        &self.id
    }

    fn with_parent(mut self, parent_event: &CommandEventBuilder) -> Self {
        self.parent = Some(parent_event.get_id().clone());
        self
    }
}

impl PubEventBuilder for CommandEventBuilder {
    fn track(&self, event: Event) {
        let val = match event.is_sensitive {
            EventType::Sensitive => TelemetryConfig::one_way_hash(&event.value),
            EventType::NonSensitive => event.value.to_string(),
        };

        telem(TelemetryEvent::Command(TelemetryCommandEvent {
            id: self.id.clone(),
            command: self.command.clone(),
            parent: self.parent.clone(),
            key: event.key,
            value: val,
        }));
    }

    fn child(&self) -> Self {
        Self::new(&self.command).with_parent(self)
    }
}

// events

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CodePath {
    Go,
    Rust,
}

impl CommandEventBuilder {
    pub fn new(command: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            command: command.to_string(),
            parent: None,
        }
    }

    pub fn track_call(&self) -> &Self {
        self.track(Event {
            key: "command".to_string(),
            value: "called".to_string(),
            is_sensitive: EventType::NonSensitive,
        });
        self
    }

    pub fn track_run_code_path(&self, path: CodePath) -> &Self {
        self.track(Event {
            key: "binary".to_string(),
            value: match path {
                CodePath::Go => "go".to_string(),
                CodePath::Rust => "rust".to_string(),
            },
            is_sensitive: EventType::NonSensitive,
        });
        self
    }

    pub fn track_telemetry_config(&self, enabled: bool) -> &Self {
        self.track(Event {
            key: "action".to_string(),
            value: if enabled { "enabled" } else { "disabled" }.to_string(),
            is_sensitive: EventType::NonSensitive,
        });
        self
    }

    pub fn track_generator_option(&self, option: &str) -> &Self {
        self.track(Event {
            key: "option".to_string(),
            value: option.to_string(),
            is_sensitive: EventType::NonSensitive,
        });
        self
    }

    pub fn track_generator_tag(&self, tag: &str) -> &Self {
        self.track(Event {
            key: "tag".to_string(),
            value: tag.to_string(),
            is_sensitive: EventType::NonSensitive,
        });
        self
    }
}