#![deny(rust_2018_idioms, warnings)]

use actix_cors::Cors;
use actix_web::{
    http::ContentEncoding,
    middleware::{Compress, Logger},
    App, HttpServer,
};

use env_logger::Env;

mod handlers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(move || {
        App::new()
            .wrap(Compress::new(ContentEncoding::Br))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(Cors::default())
            .service(handlers::get_cars_by_make)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// use futures_util::stream::StreamExt;
// use std::string::String;

// mod search;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let mut multi_threaded_runtime = tokio::runtime::Builder::new()
//         .threaded_scheduler()
//         .enable_all()
//         .thread_name("multi-threaded")
//         .build()?;

//     multi_threaded_runtime.block_on(async move {
//         let before = tokio::time::Instant::now();

//         let fetches = futures::stream::iter(vec![
//             tokio::spawn(search::crawl_rst("opel")),
//             tokio::spawn(search::crawl_rst("bmw")),
//             tokio::spawn(search::crawl_rst("toyota")),
//             tokio::spawn(search::crawl_ria("opel")),
//             tokio::spawn(search::crawl_ria("bmw")),
//             tokio::spawn(search::crawl_ria("toyota")),
//         ])
//         .buffer_unordered(3)
//         .map(|r| {
//             println!(
//                 "finished request: {}",
//                 match r {
//                     Ok(rr) => format!("{:?}", rr),
//                     Err(_) => String::from("Bad"),
//                 }
//             );
//         })
//         .collect::<Vec<_>>();

//         fetches.await;

//         println!("elapsed time: {:.2?}", before.elapsed());
//     });

//     Ok(())
// }
