use axum::Router;

mod calculations;
mod staff;
mod tips;

pub fn routes() -> Router {
    Router::new()
        .merge(staff::routes())
        .merge(calculations::routes())
        .merge(tips::routes())
}
