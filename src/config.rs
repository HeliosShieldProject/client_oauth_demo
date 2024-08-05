use dotenvy::dotenv;
use once_cell::sync::Lazy;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub discord_client_secret: String,
    pub discord_client_id: String,
    pub github_client_secret: String,
    pub github_client_id: String,
    pub google_client_secret: String,
    pub google_client_id: String,
}

pub static ENV: Lazy<Config> = Lazy::new(|| {
    dotenv().ok();
    Config {
        discord_client_secret: env::var("DISCORD_CLIENT_SECRET")
            .expect("DISCORD_CLIENT_SECRET must be set"),
        discord_client_id: env::var("DISCORD_CLIENT_ID").expect("DISCORD_CLIENT_ID must be set"),
        github_client_secret: env::var("GITHUB_CLIENT_SECRET")
            .expect("GITHUB_CLIENT_SECRET must be set"),
        github_client_id: env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID must be set"),
        google_client_secret: env::var("GOOGLE_CLIENT_SECRET")
            .expect("GOOGLE_CLIENT_SECRET must be set"),
        google_client_id: env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set"),
    }
});
