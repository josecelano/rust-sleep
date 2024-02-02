use warp::Filter;
use std::{thread, time};
use log::{info, debug, LevelFilter};

#[tokio::main]
async fn main() {
    setup_logging(LevelFilter::Info);

    // Define the route
    let route = warp::path!("sleep")
        .map(|| {
            info!("Received a request to sleep");

            // Sleep for 5 seconds
            thread::sleep(time::Duration::from_secs(5));

            // Return "awake"
            warp::reply::with_status("awake", warp::http::StatusCode::OK)
        })
        .with(warp::log("api_server"));

    // Start the server
    warp::serve(route)
        .run(([0, 0, 0, 0], 3030))
        .await;
}

fn setup_logging(level: LevelFilter) {
    if let Err(_err) = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}][{}] {}",
                chrono::Local::now().format("%+"),
                record.target(),
                record.level(),
                message
            ));
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()
    {
        panic!("Failed to initialize logging.")
    }

    debug!("logging initialized.");
}