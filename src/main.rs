use futures_util::stream::StreamExt;
use std::string::String;

mod search;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut multi_threaded_runtime = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .thread_name("multi-threaded")
        .build()?;

    multi_threaded_runtime.block_on(async move {
        let before = tokio::time::Instant::now();

        let fetches = futures::stream::iter(vec![
            tokio::spawn(search::crawl_rst("opel")),
            tokio::spawn(search::crawl_rst("bmw")),
            tokio::spawn(search::crawl_rst("toyota")),
            tokio::spawn(search::crawl_ria("opel")),
            tokio::spawn(search::crawl_ria("bmw")),
            tokio::spawn(search::crawl_ria("toyota")),
        ])
        .buffer_unordered(3)
        .map(|r| {
            println!(
                "finished request: {}",
                match r {
                    Ok(rr) => format!("{:?}", rr),
                    Err(_) => String::from("Bad"),
                }
            );
        })
        .collect::<Vec<_>>();

        fetches.await;

        println!("elapsed time: {:.2?}", before.elapsed());
    });

    Ok(())
}
