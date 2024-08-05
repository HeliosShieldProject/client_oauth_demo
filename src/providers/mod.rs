pub mod discord;
pub use discord::{discord_auth, discord_client, discord_login_authorized};

pub mod github;
pub use github::{github_auth, github_client, github_login_authorized};

pub mod google;
pub use google::{google_auth, google_client, google_login_authorized};
