use mimalloc::MiMalloc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use modulaga_config_api::app_state::AppState;
use modulaga_config_api::grpc::{application, menu_entry, workspace};
use tonic::codegen::http::{header, HeaderName, HeaderValue};
use tower_http::set_header::SetResponseHeaderLayer;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = AppState::new().await;
    let addr = ([0, 0, 0, 0], 8080).into();
    tracing::info!("listening on {}", addr);
    tonic::transport::Server::builder()
        .layer(SetResponseHeaderLayer::overriding(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("x-xss-protection"),
            HeaderValue::from_static("1; mode=block"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("x-download-options"),
            HeaderValue::from_static("noopen"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("referrer-policy"),
            HeaderValue::from_static("strict-origin-when-cross-origin"),
        ))
        .add_service(application::server(state.db.clone()))
        .add_service(menu_entry::server(state.db.clone()))
        .add_service(workspace::server(state.db))
        .serve(addr)
        .await
        .expect("server error");
}