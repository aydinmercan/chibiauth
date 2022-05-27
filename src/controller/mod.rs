use std::sync::Arc;

use axum::Router;

use crate::CsrfMap;

pub fn setup_router(csrf_map: Arc<CsrfMap>) -> Router {
    Router::new()
}
