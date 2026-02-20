use std::{net::SocketAddr, path::PathBuf, sync::Mutex};

use backend::{config::Config, webserver::{AppState, websocket_handler}};
use axum::{extract::State, Router, routing::any};
use tower_http::{services::ServeDir,trace::{DefaultMakeSpan, TraceLayer}};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use scribble::{Opts, OutputType, Scribble};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let config = Config::new();
    let server_state = AppState{
        value: 0,
        scribble: std::sync::Arc::new(Mutex::new(Scribble::new(
            [config.get_model_path().to_str().ok_or("model_path error")?],
            config.get_vad_model_path().to_str().ok_or("vlad_model_path error")?,
        )?)),
    };
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(config.get_rust_log()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let assets_dir = PathBuf::from("./static/");
    let app = Router::new().fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
        .route("/ws", any(websocket_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        ).with_state(server_state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000").await?;
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    println!("ws-Webserver created!");
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;

    Ok(())
}
