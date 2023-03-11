use crate::engine::Engine;
use anyhow::Result;
use std::sync::Arc;

#[derive(Clone)]
pub struct Module {
    inner: Arc<ModuleInner>,
}

pub struct ModuleInner {
    engine: Engine,
}

impl Module {
    pub fn from_binary(engine: &Engine, bytes: &[u8]) -> Result<Module> {
        Ok(Module {
            inner: Arc::new(ModuleInner {
                engine: engine.clone(),
            }),
        })
    }
}
