use axum::{
    http::{HeaderValue, StatusCode},
    routing::{get, post},
    Json, Router,
};

use hyper::header::{
    ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_ORIGIN, ALLOW, AUTHORIZATION,
    CONTENT_ENCODING, CONTENT_TYPE,
};
use tower_http::cors::CorsLayer;

use serde::{Deserialize, Serialize};

use thirtyfour::{DesiredCapabilities, WebDriver};

#[derive(Debug, Deserialize, Clone)]
pub struct MessageData {
    message: String,
    send_user_id: i32,
    reci_user_id: i32,
}

#[derive(Debug, Serialize, Clone)]
pub struct RecievMessage {
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // CORSの設定

    let origins = [
        "http://localhost:3000".parse::<HeaderValue>().unwrap(),
        "http://localhost:8080".parse::<HeaderValue>().unwrap(),
        "http://localhost:5173".parse::<HeaderValue>().unwrap(),
    ];
    // let cors_middleware = CorsLayer::new()
    // .allow_origin(origins)
    // .allow_methods(vec!["GET", "POST"])
    // .allow_headers(vec!["content-type"])
    // .allow_credentials(true)
    // .max_age(Some(std::time::Duration::from_secs(3600)))
    // .build();

    //  Axumのルーターを作成
    let app = Router::new()
        .route("/", get(root))
        .route("/test", post(post_handler))
        .layer(
            CorsLayer::new()
                .allow_origin(origins)
                .allow_headers([
                    ACCESS_CONTROL_ALLOW_ORIGIN,
                    ACCESS_CONTROL_ALLOW_HEADERS,
                    CONTENT_TYPE,
                ])
                .allow_credentials(true),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("リスナーバインド失敗");
    axum::serve(listener, app).await.expect("サーバー作成失敗");
    //  サーバーを作成
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // let server = server::conn(&addr);

    //  todo    スクレイプ用
    // //  **   スクレイプの準備
    // let mut caps = DesiredCapabilities::chrome();
    // //  **  headlessモード
    // // let _ = caps.add_chrome_arg("--headless");

    // let web_driver = match WebDriver::new(WEB_DRIVER_URL, caps).await {
    //     Ok(v) => v,
    //     Err(_) => return Err("DriverError".to_string()),
    // };
    //  todo    ここまで

    Ok(())
}

async fn root() -> &'static str {
    "ルートです。"
}
async fn post_handler(Json(message_data): Json<MessageData>) -> (StatusCode, Json<RecievMessage>) {
    println!("test");
    let json_reciv_message = message_data.message;
    let reciv_message = RecievMessage {
        message: format!("返信用です {}", json_reciv_message),
    };

    let des_recov_message = Json(reciv_message);
    println!("des{:?}", des_recov_message);

    (StatusCode::CREATED, des_recov_message)
}
