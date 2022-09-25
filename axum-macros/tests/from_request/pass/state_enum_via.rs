use axum::{
    extract::{State, FromRef},
    routing::get,
    Router,
};
use axum_macros::FromRequest;

fn main() {
    let _: Router<AppState> = Router::with_state(AppState::default())
        .route("/a", get(|_: AppState| async {}))
        .route("/b", get(|_: InnerState| async {}));
}

#[derive(Clone, FromRequest)]
#[from_request(via(State))]
enum AppState {
    One,
}

impl Default for AppState {
    fn default() -> AppState {
        Self::One
    }
}

#[derive(FromRequest)]
#[from_request(via(State), state(AppState))]
enum InnerState {}

impl FromRef<AppState> for InnerState {
    fn from_ref(_: &AppState) -> Self {
        todo!(":shrug:")
    }
}