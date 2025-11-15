use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Worker metadata for orchestration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerInfo {
    pub name: String,
    pub agent_type: String,
    pub task_id: Option<String>,
    pub tmux_session: String,
    pub working_dir: String,
    pub spawned_at: u64,
    pub status: WorkerStatus,
    pub messages_sent: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkerStatus {
    Starting,
    Ready,
    Working,
    Idle,
    Error,
    Stopped,
}

impl std::fmt::Display for WorkerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WorkerStatus::Starting => write!(f, "starting"),
            WorkerStatus::Ready => write!(f, "ready"),
            WorkerStatus::Working => write!(f, "working"),
            WorkerStatus::Idle => write!(f, "idle"),
            WorkerStatus::Error => write!(f, "error"),
            WorkerStatus::Stopped => write!(f, "stopped"),
        }
    }
}

/// Worker registry for tracking active sessions
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkerRegistry {
    workers: HashMap<String, WorkerInfo>,
}

impl WorkerRegistry {
    /// Create new empty registry
    pub fn new() -> Self {
        Self {
            workers: HashMap::new(),
        }
    }

    /// Load registry from file
    pub fn load() -> Result<Self> {
        let path = Self::get_registry_path();
        if !path.exists() {
            return Ok(Self::new());
        }

        let content = fs::read_to_string(&path)?;
        let registry: WorkerRegistry = serde_json::from_str(&content)?;
        Ok(registry)
    }

    /// Save registry to file
    pub fn save(&self) -> Result<()> {
        let path = Self::get_registry_path();
        let content = serde_json::to_string_pretty(&self)?;
        fs::write(&path, content)?;
        Ok(())
    }

    /// Get registry file path
    fn get_registry_path() -> PathBuf {
        let home = dirs::home_dir().expect("Cannot find home directory");
        home.join(".claude-worker-registry.json")
    }

    /// Register a new worker
    pub fn register(&mut self, worker: WorkerInfo) -> Result<()> {
        self.workers.insert(worker.name.clone(), worker);
        self.save()?;
        Ok(())
    }

    /// Unregister a worker
    pub fn unregister(&mut self, name: &str) -> Result<()> {
        self.workers.remove(name);
        self.save()?;
        Ok(())
    }

    /// Get worker info
    pub fn get(&self, name: &str) -> Option<&WorkerInfo> {
        self.workers.get(name)
    }

    /// Get mutable worker info
    pub fn get_mut(&mut self, name: &str) -> Option<&mut WorkerInfo> {
        self.workers.get_mut(name)
    }

    /// Update worker status
    pub fn update_status(&mut self, name: &str, status: WorkerStatus) -> Result<()> {
        if let Some(worker) = self.workers.get_mut(name) {
            worker.status = status;
            self.save()?;
        }
        Ok(())
    }

    /// Increment message counter
    pub fn increment_messages(&mut self, name: &str) -> Result<()> {
        if let Some(worker) = self.workers.get_mut(name) {
            worker.messages_sent += 1;
            self.save()?;
        }
        Ok(())
    }

    /// List all workers
    pub fn list_all(&self) -> Vec<&WorkerInfo> {
        self.workers.values().collect()
    }

    /// List workers by agent type
    pub fn list_by_agent(&self, agent_type: &str) -> Vec<&WorkerInfo> {
        self.workers
            .values()
            .filter(|w| w.agent_type == agent_type)
            .collect()
    }

    /// List workers by status
    pub fn list_by_status(&self, status: WorkerStatus) -> Vec<&WorkerInfo> {
        self.workers
            .values()
            .filter(|w| w.status == status)
            .collect()
    }

    /// Find idle workers
    pub fn find_idle(&self) -> Vec<&WorkerInfo> {
        self.list_by_status(WorkerStatus::Idle)
    }

    /// Find worker by task ID
    pub fn find_by_task(&self, task_id: &str) -> Option<&WorkerInfo> {
        self.workers
            .values()
            .find(|w| w.task_id.as_deref() == Some(task_id))
    }

    /// Check if worker exists
    pub fn exists(&self, name: &str) -> bool {
        self.workers.contains_key(name)
    }

    /// Count workers
    pub fn count(&self) -> usize {
        self.workers.len()
    }

    /// Cleanup stopped workers
    pub fn cleanup_stopped(&mut self) -> Result<usize> {
        let stopped: Vec<String> = self.workers
            .iter()
            .filter(|(_, w)| w.status == WorkerStatus::Stopped)
            .map(|(name, _)| name.clone())
            .collect();

        let count = stopped.len();
        for name in stopped {
            self.workers.remove(&name);
        }

        if count > 0 {
            self.save()?;
        }

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_operations() {
        let mut registry = WorkerRegistry::new();

        let worker = WorkerInfo {
            name: "test-worker".to_string(),
            agent_type: "coding-agent".to_string(),
            task_id: Some("task-123".to_string()),
            tmux_session: "test-worker".to_string(),
            working_dir: "/tmp".to_string(),
            spawned_at: 12345,
            status: WorkerStatus::Ready,
            messages_sent: 0,
        };

        registry.register(worker).unwrap();

        assert_eq!(registry.count(), 1);
        assert!(registry.exists("test-worker"));

        registry.update_status("test-worker", WorkerStatus::Working).unwrap();
        assert_eq!(registry.get("test-worker").unwrap().status, WorkerStatus::Working);
    }
}
