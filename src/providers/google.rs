use anyhow::Context;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use axum_extra::response::Html;
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl,
};
use serde::Deserialize;

use crate::{config::ENV, AppError, AppState};

pub fn google_client() -> Result<BasicClient, AppError> {
    let redirect_url = "http://localhost:3000/auth/google/authorized".to_string();
    let auth_url = "https://accounts.google.com/o/oauth2/v2/auth".to_string();
    let token_url = "https://google.com/login/oauth/access_token".to_string();

    Ok(BasicClient::new(
        ClientId::new(ENV.google_client_id.clone()),
        Some(ClientSecret::new(ENV.google_client_secret.clone())),
        AuthUrl::new(auth_url).context("failed to create new authorization server URL")?,
        Some(TokenUrl::new(token_url).context("failed to create new token endpoint URL")?),
    )
    .set_redirect_uri(
        RedirectUrl::new(redirect_url).context("failed to create new redirection URL")?,
    ))
}

pub async fn google_auth(State(state): State<AppState>) -> impl IntoResponse {
    let (auth_url, _csrf_token) = state
        .google_client
        .authorize_url(CsrfToken::new_random)
        .add_scopes([
            Scope::new("https://www.googleapis.com/auth/userinfo.email".to_string()),
            Scope::new("https://www.googleapis.com/auth/userinfo.profile".to_string()),
        ])
        .url();

    Redirect::to(auth_url.as_ref())
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AuthRequest {
    code: String,
    state: String,
}

pub async fn google_login_authorized(
    Query(query): Query<AuthRequest>,
) -> Result<Html<String>, AppError> {
    println!("{:?}", query);
    let code = query.code;
    Ok(Html(format!(
        "<p>The code: {}<p><p>Go home: <a href='/'>Home</a></p>",
        code
    )))
}
