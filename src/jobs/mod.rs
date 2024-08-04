use {
    crate::app::RunTime,
    anyhow::{Ok, Result},
    std::future::Future,
    std::pin::Pin,
    std::sync::Arc,
};

mod hello;

pub type JobEntry<'a> = Arc<
    dyn Fn(&'a RunTime) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> + Send + Sync + 'a,
>;

macro_rules! init_job_maps {
    ( { $($job_name:ident),* $(,)? } ) => {{
        let mut jobs: std::collections::HashMap<&str, JobEntry> = std::collections::HashMap::new();
        $(
            // jobs.insert(stringify!($job_name), $job_name::job_entry);
            jobs.insert(stringify!($job_name), Arc::new(|runtime| Box::pin(async { $job_name::job_entry(runtime).await })));
        )*
        jobs
    }};
}

pub async fn do_job(name: &str, _runtime: &RunTime) -> Result<()> {
    let jobs = init_job_maps!({ hello });
    let job = jobs.get(name).unwrap();
    job(_runtime).await?;
    Ok(())
}
