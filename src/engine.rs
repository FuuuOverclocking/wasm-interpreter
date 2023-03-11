use crate::config::Config;
use anyhow::Result;
use std::sync::Arc;

#[derive(Clone)]
pub struct Engine {
    inner: Arc<EngineInner>,
}

struct EngineInner {
    config: Config,
}

impl Engine {
    pub fn new(config: &Config) -> Result<Engine> {
        Ok(Engine {
            inner: Arc::new(EngineInner {
                config: config.clone(),
            }),
        })
    }
}
