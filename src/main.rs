use anyhow::Context;
use axum::{
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use axum_extra::response::Html;
use http::StatusCode;
use oauth2::basic::BasicClient;
use providers::{
    discord_auth, discord_client, discord_login_authorized, github_auth, github_client,
    github_login_authorized, google_auth, google_client, google_login_authorized,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod providers;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "test_oauth=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app_state = AppState {
        discord_client: discord_client().unwrap(),
        github_client: github_client().unwrap(),
        google_client: google_client().unwrap(),
    };

    let app = Router::new()
        .route("/", get(index))
        .route("/auth/discord", get(discord_auth))
        .route("/auth/discord/authorized", get(discord_login_authorized))
        .route("/auth/github", get(github_auth))
        .route("/auth/github/authorized", get(github_login_authorized))
        .route("/auth/google", get(google_auth))
        .route("/auth/google/authorized", get(google_login_authorized))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .context("failed to bind TcpListener")
        .unwrap();

    tracing::debug!(
        "listening on {}",
        listener
            .local_addr()
            .context("failed to return local address")
            .unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    discord_client: BasicClient,
    github_client: BasicClient,
    google_client: BasicClient,
}

async fn index() -> Html<&'static str> {
    Html("To login visit:\ndiscord: <a href='/auth/discord'>Discord</a>\ngithub: <a href='/auth/github'>Github</a>\ngoogle: <a href='/auth/google'>Google</a>")
}

#[derive(Debug)]
struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("Application error: {:#}", self.0);

        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
