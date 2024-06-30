use {crate::app::RunTime, anyhow::Result, std::collections::HashMap};

#[derive(Hash, Eq, PartialEq)]
pub enum HookPoint {
    RunInit,
}

pub type HookFn = fn(runtime: &RunTime) -> Result<()>;

pub struct AppHooks {
    pub all_hooks: HashMap<HookPoint, Vec<HookFn>>,
}

impl AppHooks {
    pub fn new() -> Self {
        Self {
            all_hooks: HashMap::new(),
        }
    }

    pub fn register_hook(&mut self, point: HookPoint, hook: HookFn) {
        self.all_hooks
            .entry(point)
            .or_insert_with(Vec::new)
            .push(hook);
    }

    pub fn run_hooks(&self, point: HookPoint, runtime: &RunTime) -> Result<()> {
        if let Some(hooks) = self.all_hooks.get(&point) {
            for hook in hooks {
                hook(runtime)?;
            }
        }
        Ok(())
    }
}
