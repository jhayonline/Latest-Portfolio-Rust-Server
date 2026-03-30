use salvo::prelude::*;

use crate::handlers::wall::{add_like, add_message, get_messages};

pub fn build_router() -> Router {
    // /api/wall/messages    → GET (list), POST (create)
    // /api/wall/messages/:id/like → POST (like)
    Router::with_path("api/wall").push(
        Router::with_path("messages")
            .get(get_messages) // GET  /api/wall/messages
            .post(add_message) // POST /api/wall/messages
            .push(
                Router::with_path("<id>/like").post(add_like), // POST /api/wall/messages/:id/like
            ),
    )
}
