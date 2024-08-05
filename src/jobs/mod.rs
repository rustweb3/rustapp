use {
    crate::app::RunTime,
    anyhow::{Ok, Result},
    entry::JobEntry,
    log::{error, info},
    std::sync::Arc,
};

mod entry;
mod hello;

macro_rules! init_job_maps {
    ( { $($job_name:ident),* $(,)? } ) => {{
        let mut jobs: std::collections::HashMap<&str,JobEntry> = std::collections::HashMap::new();
        $(
            jobs.insert(stringify!($job_name), Arc::new(|runtime| Box::pin(async { $job_name::job_entry(runtime).await })));
        )*
        jobs
    }};
}

pub async fn do_job(name: &str, _runtime: &RunTime) -> Result<()> {
    let jobs = init_job_maps!({ hello });
    if !jobs.contains_key(name) {
        error!("job {} not found", name);
        info!("available jobs: {:?}", jobs.keys());
        return Err(anyhow::anyhow!("job {} not found", name));
    }
    let job = jobs.get(name).unwrap();
    job(_runtime).await?;

    Ok(())
}
