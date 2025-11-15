use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type of payload to inject
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PayloadType {
    /// Regular context/information
    Context,
    /// Warning message
    Warning,
    /// Blocker/error that needs attention
    Block,
    /// Completion notification
    Completion,
    /// Progress update
    Progress,
    /// User prompt (simulated user input)
    UserPrompt,
}

/// Payload to inject into Claude session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InjectionPayload {
    pub payload_type: PayloadType,
    pub content: String,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl InjectionPayload {
    /// Create a context payload
    pub fn context(content: impl Into<String>) -> Self {
        Self {
            payload_type: PayloadType::Context,
            content: content.into(),
            metadata: None,
        }
    }

    /// Create a warning payload
    pub fn warning(content: impl Into<String>) -> Self {
        Self {
            payload_type: PayloadType::Warning,
            content: content.into(),
            metadata: None,
        }
    }

    /// Create a blocker payload
    pub fn block(content: impl Into<String>) -> Self {
        Self {
            payload_type: PayloadType::Block,
            content: content.into(),
            metadata: None,
        }
    }

    /// Create a completion notification payload
    pub fn completion(summary: impl Into<String>, metadata: HashMap<String, serde_json::Value>) -> Self {
        Self {
            payload_type: PayloadType::Completion,
            content: summary.into(),
            metadata: Some(metadata),
        }
    }

    /// Create a progress update payload
    pub fn progress(percentage: u8, message: impl Into<String>) -> Self {
        let mut metadata = HashMap::new();
        metadata.insert(
            "progress_percentage".to_string(),
            serde_json::Value::from(percentage),
        );

        Self {
            payload_type: PayloadType::Progress,
            content: message.into(),
            metadata: Some(metadata),
        }
    }

    /// Create a user prompt payload (simulates user input)
    pub fn user_prompt(prompt: impl Into<String>) -> Self {
        Self {
            payload_type: PayloadType::UserPrompt,
            content: prompt.into(),
            metadata: None,
        }
    }

    /// Add metadata to payload
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Serialize) -> Self {
        let metadata = self.metadata.get_or_insert_with(HashMap::new);
        metadata.insert(
            key.into(),
            serde_json::to_value(value).unwrap_or(serde_json::Value::Null),
        );
        self
    }

    /// Convert payload to string suitable for injection
    pub fn to_injection_string(&self) -> String {
        match self.payload_type {
            PayloadType::Context => format!("\n\n📋 REAL-TIME CONTEXT UPDATE:\n{}\n", self.content),

            PayloadType::Warning => format!("\n\n⚠️ WARNING:\n{}\n", self.content),

            PayloadType::Block => format!(
                "\n\n🚨 BLOCKER - ATTENTION NEEDED:\n{}\n\nPlease review this blocker and adjust your approach.\n",
                self.content
            ),

            PayloadType::Completion => {
                let metadata_str = if let Some(ref metadata) = self.metadata {
                    format!(
                        "\n\nDetails:\n{}",
                        serde_json::to_string_pretty(metadata).unwrap_or_default()
                    )
                } else {
                    String::new()
                };

                format!(
                    "\n\n✅ COMPLETION NOTIFICATION:\n{}{}\n",
                    self.content, metadata_str
                )
            }

            PayloadType::Progress => {
                let percentage = self
                    .metadata
                    .as_ref()
                    .and_then(|m| m.get("progress_percentage"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0);

                format!(
                    "\n\n📊 PROGRESS UPDATE [{} %]:\n{}\n",
                    percentage, self.content
                )
            }

            PayloadType::UserPrompt => {
                // For user prompts, just send the content directly
                // Claude will interpret this as if the user typed it
                format!("{}", self.content)
            }
        }
    }

    /// Convert payload to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

/// Builder for creating complex injection payloads
pub struct PayloadBuilder {
    payload_type: PayloadType,
    content: String,
    metadata: HashMap<String, serde_json::Value>,
}

impl PayloadBuilder {
    pub fn new(payload_type: PayloadType) -> Self {
        Self {
            payload_type,
            content: String::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    pub fn metadata(mut self, key: impl Into<String>, value: impl Serialize) -> Self {
        self.metadata.insert(
            key.into(),
            serde_json::to_value(value).unwrap_or(serde_json::Value::Null),
        );
        self
    }

    pub fn build(self) -> InjectionPayload {
        InjectionPayload {
            payload_type: self.payload_type,
            content: self.content,
            metadata: if self.metadata.is_empty() {
                None
            } else {
                Some(self.metadata)
            },
        }
    }
}

/// Preset payloads for common scenarios
pub mod presets {
    use super::*;

    /// Dependency completed notification
    pub fn dependency_completed(
        task_name: &str,
        summary: &str,
        insights: Vec<String>,
    ) -> InjectionPayload {
        PayloadBuilder::new(PayloadType::Completion)
            .content(format!(
                "Upstream dependency '{}' has completed.\n\nSummary: {}\n\nYou can now proceed with your task using this context.",
                task_name, summary
            ))
            .metadata("upstream_task", task_name)
            .metadata("summary", summary)
            .metadata("insights", insights)
            .build()
    }

    /// Task ready to start
    pub fn task_ready(task_name: &str, context: &str) -> InjectionPayload {
        PayloadBuilder::new(PayloadType::Context)
            .content(format!(
                "Task '{}' is ready to start.\n\nContext: {}",
                task_name, context
            ))
            .metadata("task", task_name)
            .build()
    }

    /// Test failure notification
    pub fn test_failed(test_name: &str, error: &str) -> InjectionPayload {
        PayloadBuilder::new(PayloadType::Block)
            .content(format!(
                "Test '{}' failed with error:\n\n{}\n\nPlease fix the failing test before proceeding.",
                test_name, error
            ))
            .metadata("test", test_name)
            .metadata("error", error)
            .build()
    }

    /// Security audit warning
    pub fn security_warning(issue: &str, severity: &str) -> InjectionPayload {
        PayloadBuilder::new(PayloadType::Warning)
            .content(format!(
                "Security audit found {} severity issue:\n\n{}\n\nPlease address this security concern.",
                severity, issue
            ))
            .metadata("severity", severity)
            .build()
    }

    /// Code review feedback
    pub fn code_review_feedback(file: &str, feedback: &str) -> InjectionPayload {
        PayloadBuilder::new(PayloadType::Context)
            .content(format!(
                "Code review feedback for {}:\n\n{}",
                file, feedback
            ))
            .metadata("file", file)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payload_creation() {
        let payload = InjectionPayload::context("Test context message");
        println!("{}", payload.to_injection_string());

        let payload = InjectionPayload::warning("Test warning");
        println!("{}", payload.to_injection_string());

        let payload = InjectionPayload::block("Test blocker");
        println!("{}", payload.to_injection_string());

        let payload = InjectionPayload::progress(50, "Halfway done");
        println!("{}", payload.to_injection_string());
    }

    #[test]
    fn test_presets() {
        let payload = presets::dependency_completed(
            "Design schema",
            "Created 5 tables with proper indexes",
            vec![
                "Use UUID for IDs".to_string(),
                "Add created_at/updated_at".to_string(),
            ],
        );
        println!("{}", payload.to_injection_string());

        let payload = presets::test_failed("test_jwt_expiry", "Token expiry calculation incorrect");
        println!("{}", payload.to_injection_string());
    }
}
