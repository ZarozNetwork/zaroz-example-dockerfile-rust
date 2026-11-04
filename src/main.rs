//! A small HTTP service with axum and tokio.
//!
//! It listens on `$PORT` (8080 when unset), answers `GET /` and `GET /healthz`, and on SIGTERM
//! stops accepting connections and lets the requests in flight finish.

use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

fn app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/healthz", get(healthz))
}

async fn root() -> Json<Value> {
    Json(json!({ "message": "hello from a Zaroz git build", "language": "rust" }))
}

async fn healthz() -> Json<Value> {
    Json(json!({ "ok": true }))
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("install the Ctrl+C handler");
    };
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("install the SIGTERM handler")
            .recv()
            .await;
    };
    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_owned());
    let listener =
        tokio::net::TcpListener::bind(("0.0.0.0", port.parse().expect("PORT is a number"))).await?;
    println!("listening on :{port}");
    axum::serve(listener, app())
        .with_graceful_shutdown(shutdown_signal())
        .await
}

#[cfg(test)]
mod tests {
    use super::app;
    use axum::{body::Body, http::Request, http::StatusCode};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    async fn get(path: &str) -> (StatusCode, String) {
        let response = app()
            .oneshot(Request::get(path).body(Body::empty()).unwrap())
            .await
            .unwrap();
        let status = response.status();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        (status, String::from_utf8(body.to_vec()).unwrap())
    }

    #[tokio::test]
    async fn root_names_the_language() {
        let (status, body) = get("/").await;
        assert_eq!(status, StatusCode::OK);
        assert!(body.contains(r#""language":"rust""#), "{body}");
    }

    #[tokio::test]
    async fn healthz_is_ok() {
        assert_eq!(get("/healthz").await.0, StatusCode::OK);
    }

    #[tokio::test]
    async fn unknown_paths_are_404() {
        assert_eq!(get("/nope").await.0, StatusCode::NOT_FOUND);
    }
}
