use std::sync::Arc;

use axum::{AddExtensionLayer, Router};

use crate::CsrfMap;

pub fn setup_router(csrf_map: Arc<CsrfMap>) -> Router {
    Router::new().layer(AddExtensionLayer::new(csrf_map))
}
