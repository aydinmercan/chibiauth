use anyhow::Result;

pub fn run() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build();

    if let Err(err) = runtime {
        return;
    }
}
