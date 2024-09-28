use axum::body::Body;
use axum::response::Response;
use axum::Router;
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
    info!("Listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn ok_handler() -> Response<Body> {
    let content = "OK";
    Response::builder()
        .status(200)
        .body(Body::from(content))
        .unwrap()
}
