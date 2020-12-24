use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    level: String,
    message: String,
}

#[tokio::main]
async fn main() {
    let buf = Arc::new(Mutex::new(String::new()));
    let shared = buf.clone();

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let get = warp::path!("getUpdate")
        .map(move || shared.clone())
        .and_then(|buf: Arc<Mutex<String>>| async move {
            let mut buf = buf.lock().await;
            let data = buf.to_string();
            println!("got {}", data);
            buf.clear();
            let body = format!(
                r#"
        <html>
            <head>
                <meta charset="UTF-8">
                <title>Echo!</title>
            </head>
            <body>
                {}
            </body>
        </html>
        "#,
                data
            );
            Ok::<_, std::convert::Infallible>(warp::reply::html(body))
        });

    let shared = buf.clone();
    let push = warp::post()
        .and(warp::path!("pushEvent"))
        .and(warp::body::json())
        .and(warp::any().map(move || shared.clone()))
        .and_then(|event: Event, buf: Arc<Mutex<String>>| async move {
            let mut buf = buf.lock().await;
            buf.push_str(&format!("<div>{}</div>\n", event.message));
            println!("{}", buf);
            Ok::<_, std::convert::Infallible>(warp::reply::json(&event))
        });

    warp::serve(get.or(push)).run(([0, 0, 0, 0], 3030)).await;
}
