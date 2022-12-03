use axum::extract::FromRef;

use crate::database::Database;

// clone if cloneable
pub struct AppState {
    database: Database,
}

impl FromRef<AppState> for Database {
    fn from_ref(state: &AppState) -> Database {
        state.database.clone()
    }
}
