use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct SstableConfig {
    pub timeout_ms: u64,
    pub max_retries: u32,
    pub enabled: bool,
}

impl Default for SstableConfig {
    fn default() -> Self {
        Self { timeout_ms: 5000, max_retries: 3, enabled: true }
    }
}

#[derive(Debug, Clone)]
pub struct SstableContext {
    pub correlation_id: String,
    pub payload: HashMap<String, String>,
    pub transitions: Vec<String>,
}

pub struct SstableEngine {
    config: SstableConfig,
    registry: Arc<RwLock<HashMap<String, String>>>,
    processed: Arc<RwLock<u64>>,
}

impl SstableEngine {
    pub fn new(config: SstableConfig) -> Self {
        Self {
            config,
            registry: Arc::new(RwLock::new(HashMap::new())),
            processed: Arc::new(RwLock::new(0)),
        }
    }

    pub fn process(&self, ctx: &mut SstableContext) -> String {
        let mut count = self.processed.write().unwrap();
        *count += 1;
        ctx.transitions.push("PROCESSED".to_string());
        format!("ok-{}", ctx.correlation_id)
    }
}
