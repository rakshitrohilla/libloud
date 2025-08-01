pub mod auth;
pub mod client;
pub mod search;
pub mod stream;
pub mod models;
pub mod utils;

// Re-export common components for easier access
pub use auth::authenticate;
pub use client::SoundCloudClient;
pub use search::{search_tracks, TrackResult};
pub use stream::play_track;
pub use models::{Track, User};