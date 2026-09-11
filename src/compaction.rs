use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct CompactionConfig {
    pub timeout_ms: u64,
    pub max_retries: u32,
    pub enabled: bool,
}

impl Default for CompactionConfig {
    fn default() -> Self {
        Self { timeout_ms: 5000, max_retries: 3, enabled: true }
    }
}

#[derive(Debug, Clone)]
pub struct CompactionContext {
    pub correlation_id: String,
    pub payload: HashMap<String, String>,
    pub transitions: Vec<String>,
}

pub struct CompactionEngine {
    config: CompactionConfig,
    registry: Arc<RwLock<HashMap<String, String>>>,
    processed: Arc<RwLock<u64>>,
}

impl CompactionEngine {
    pub fn new(config: CompactionConfig) -> Self {
        Self {
            config,
            registry: Arc::new(RwLock::new(HashMap::new())),
            processed: Arc::new(RwLock::new(0)),
        }
    }

    pub fn process(&self, ctx: &mut CompactionContext) -> String {
        let mut count = self.processed.write().unwrap();
        *count += 1;
        ctx.transitions.push("PROCESSED".to_string());
        format!("ok-{}", ctx.correlation_id)
    }
}
