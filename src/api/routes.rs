use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::{routing::get, routing::post, Router};

use crate::api::cost_bearer::{cost_bearer_delete, cost_bearer_get, cost_bearer_post, cost_bearer_update};
use crate::api::expense_entry::{expense_entry_delete, expense_entry_get, expense_entry_post, expense_entry_update};

pub async fn setup_routing() -> Router {
    let router = Router::new()
                            .merge(route_expense_entry().await)
                            .merge(route_cost_bearer().await)
                            .fallback(handle_routing_error);

    router
}

async fn handle_routing_error(uri: Uri) -> Response {
    (StatusCode::BAD_REQUEST, format!("No such endpoint: {}", uri.path())).into_response()
}

async fn route_expense_entry() -> Router {
    let expense_entry_router = Router::new()
                            .route("/expense_entries/{id}", get(expense_entry_get).patch(expense_entry_update).delete(expense_entry_delete))
                            .route("/expense_entries", post(expense_entry_post));

    expense_entry_router
}

async fn route_cost_bearer() -> Router {
    let cost_bearer_router = Router::new()
                            .route("/cost_bearers/{id}", get(cost_bearer_get).patch(cost_bearer_update).delete(cost_bearer_delete))
                            .route("/cost_bearers", post(cost_bearer_post));

    cost_bearer_router
}

