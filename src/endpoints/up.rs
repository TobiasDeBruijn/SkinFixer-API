use paperclip::actix::{get, api_v2_operation};
use crate::endpoints::Empty;

/// Check if the server is up
/// Responds with a 200 OK if it is
#[get("/health")]
#[api_v2_operation]
pub async fn up() -> Empty {
    Empty
}