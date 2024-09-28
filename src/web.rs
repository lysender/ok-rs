use axum::http::Method;
use axum::response::Response;
use axum::Router;
use axum::{body::Body, http::HeaderMap};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::{info, Level};

use crate::Result;

pub async fn start_server(port: u16) -> Result<()> {
    let routes = Router::new().fallback(ok_handler).layer(
        ServiceBuilder::new().layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        ),
    );

    // Setup the server
    let ip = "127.0.0.1";
    let addr = format!("{}:{}", ip, port);
    info!("HTTP server started at {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn ok_handler(headers: HeaderMap, method: Method, body: Body) -> Response<Body> {
    let mut content = Body::from("OK");
    let mut content_type = "text/plain; charset=UTF8";

    // For POST, PUT and PATCH requests, send back the body if JSON
    // Otherwise, just send OK
    if method == "POST" || method == "PUT" || method == "PATCH" {
        if let Some("application/json") = headers.get("content-type").map(|c| c.to_str().unwrap()) {
            content_type = "application/json";
            content = body;
        }
    }

    Response::builder()
        .header("Content-Type", content_type)
        .status(200)
        .body(content)
        .unwrap()
}
