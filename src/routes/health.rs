use crate::empty::Empty;

/// Check if the server is up
/// Responds with a 200 OK if it is
pub async fn up() -> Empty {
    Empty
}
