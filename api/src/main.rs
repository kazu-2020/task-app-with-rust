// レイヤードアーキテクチャの各層をモジュールとして宣言
mod application;
mod domain;
mod infrastructure;
mod presentation;

use axum::Router;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // トレーシング（ロギング）の初期化
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 環境変数の読み込み
    dotenvy::dotenv().ok();

    // ルーターの構築
    let app = create_app();

    // サーバーの起動
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to address");

    tracing::info!("Server listening on http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

/// アプリケーションのルーターを構築
fn create_app() -> Router {
    // プレゼンテーション層のルーティング
    let routes = presentation::routes::create_routes();

    Router::new()
        .merge(routes)
        // ミドルウェアの追加
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}
