use {crate::app::RunTime, anyhow::Result, std::future::Future, std::pin::Pin, std::sync::Arc};

pub type JobEntry<'a> = Arc<
    dyn Fn(&'a RunTime) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> + Send + Sync + 'a,
>;
