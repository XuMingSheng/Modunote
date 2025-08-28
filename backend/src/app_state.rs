use storage::Database;
use storage::RepositoryProvider;

use axum::extract::FromRef;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub repos: RepositoryProvider,
}

impl FromRef<AppState> for RepositoryProvider {
    fn from_ref(app_state: &AppState) -> RepositoryProvider {
        app_state.repos.clone()
    }
}
