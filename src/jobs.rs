use {
    crate::app::RunTime,
    log::{info, warn},
    std::thread,
};

pub fn hello_job(r: &RunTime) {
    info!("config form hello.debug Job : {}", r.config.main.debug);
    warn!("cli form hello.debug Job : {}", r.cli.debug);

    let name = r.cli.name.as_deref().unwrap_or("world");
    info!("Hello, {}!", name);
    let mut thread_handles = vec![];

    for i in 0..10 {
        let context = r.context.clone();
        let handle = thread::spawn(move || {
            info!("thread {} is running", i);
            let mut ctx = context.lock().unwrap();
            info!("context value: {}", ctx.value);
            ctx.value += 1;
        });
        thread_handles.push(handle);
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }

    info!("now the value is : {}", r.context.lock().unwrap().value);
}
