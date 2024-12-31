use warp::Filter;
use log::info;

pub async fn start_api() {
    // Define the `/echo/{message}` endpoint
    let echo = warp::path!("echo" / String)
        .map(|message: String| {
            info!("Received API request with message: {}", message);
            warp::reply::json(&format!("You said: {}", message))
        });

    info!("Starting API server on http://127.0.0.1:3030");
    // Run the server
    warp::serve(echo).run(([127, 0, 0, 1], 3030)).await;
}
