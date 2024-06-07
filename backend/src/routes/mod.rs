use axum::Router;

mod calculations;
mod commissions;
mod staff;
mod tips;
mod wines;

pub fn routes() -> Router {
    Router::new()
        .merge(staff::routes())
        .merge(calculations::routes())
        .merge(tips::routes())
        .merge(wines::routes())
        .merge(commissions::routes())
}
