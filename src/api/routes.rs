use std::sync::Arc;

use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::{Router, routing::get, routing::post};

use crate::api::cost_bearer::{
    cost_bearer_delete, cost_bearer_get, cost_bearer_post, cost_bearer_update,
};
use crate::api::expense_entry::{
    expense_entry_delete, expense_entry_get, expense_entry_post, expense_entry_update,
};
use crate::api::expense_type::{
    expense_type_delete, expense_type_get, expense_type_post, expense_type_update,
};
use crate::service::expense_entry::ExpenseEntryService;

#[derive(Clone)]
pub struct Services {
    pub expense_entry_service: Arc<ExpenseEntryService>,
}

pub async fn setup_routing() -> Router<Services> {
    Router::new()
        .merge(route_expense_entry())
        .merge(route_cost_bearer())
        .merge(route_expense_type())
        .fallback(handle_routing_error)
}

async fn handle_routing_error(uri: Uri) -> Response {
    (
        StatusCode::BAD_REQUEST,
        format!("No such endpoint: {}", uri.path()),
    )
        .into_response()
}

fn route_expense_entry() -> Router<Services> {
    Router::new()
        .route(
            "/expense_entries/{id}",
            get(expense_entry_get)
                .patch(expense_entry_update)
                .delete(expense_entry_delete),
        )
        .route("/expense_entries", post(expense_entry_post))
}

fn route_cost_bearer() -> Router<Services> {
    Router::new()
        .route(
            "/cost_bearers/{id}",
            get(cost_bearer_get)
                .patch(cost_bearer_update)
                .delete(cost_bearer_delete),
        )
        .route("/cost_bearers", post(cost_bearer_post))
}

fn route_expense_type() -> Router<Services> {
    Router::new()
        .route(
            "/expense_types/{id}",
            get(expense_type_get)
                .patch(expense_type_update)
                .delete(expense_type_delete),
        )
        .route("/expense_types", post(expense_type_post))
}
