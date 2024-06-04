use axum::Router;

mod calculations;
mod staff;

pub fn routes() -> Router {
    Router::new()
        .merge(staff::routes())
        .merge(calculations::routes())
}
