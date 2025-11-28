use axum::Router;
use axum::extract::{FromRef, State};
use axum::http::Method;
use axum::response::Response;
use axum::routing::any;
use axum::{body::Body, http::HeaderMap};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::{Level, info};

use crate::Result;
use crate::config::AppArgs;

#[derive(Clone)]
pub struct Config {
    pub debug_headers: bool,
}

#[derive(Clone, FromRef)]
pub struct AppState {
    pub config: Config,
}

pub async fn start_server(args: AppArgs) -> Result<()> {
    let config = Config {
        debug_headers: args.debug_headers,
    };
    let state = AppState { config };

    let routes = Router::new().merge(ok_routes(state)).layer(
        ServiceBuilder::new().layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        ),
    );

    // Setup the server
    let host = args.host.unwrap_or("127.0.0.1".to_string());
    let addr = format!("{}:{}", host, args.port);
    info!("HTTP server started at {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn ok_routes(state: AppState) -> Router {
    Router::new()
        .route("/", any(ok_handler))
        .fallback(any(ok_handler))
        .with_state(state)
}

async fn ok_handler(
    state: State<AppState>,
    headers: HeaderMap,
    method: Method,
    body: Body,
) -> Response<Body> {
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

    if state.config.debug_headers {
        info!("Request Headers: {:#?}", headers);
    }

    Response::builder()
        .header("Content-Type", content_type)
        .status(200)
        .body(content)
        .unwrap()
}
