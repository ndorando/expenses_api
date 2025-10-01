mod api;
mod domain;
mod repository;
mod service;

//#[cfg(any(test, feature = "test-utils"))]
pub mod test_util;

use std::sync::Arc;

use api::routes::setup_routing;
use repository::sqliterepository::expense_entry::ExpenseEntryReadSqliteRepositry;

use crate::{api::routes::Services, service::expense_entry::ExpenseEntryService};

#[tokio::main]
async fn main() {
    let read_repo = Arc::new(ExpenseEntryReadSqliteRepositry::new());
    let expense_entry_service = Arc::new(ExpenseEntryService::new(read_repo));
    let services = Services {
        expense_entry_service: expense_entry_service.clone(),
    };

    let router = setup_routing().await.with_state(services);
    let addr = "0.0.0.0:6570";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
