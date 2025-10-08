mod api;
mod domain;
mod repository;
mod service;

//#[cfg(any(test, feature = "test-utils"))]
pub mod test_util;

use std::sync::Arc;

use api::routes::setup_routing;
use repository::sqliterepository::cost_bearer::{
    CostBearerReadSqliteRepository, CostBearerWriteSqliteRepository,
};
use repository::sqliterepository::expense_entry::{
    ExpenseEntryReadSqliteRepository, ExpenseEntryWriteSqliteRepository,
};

use crate::{
    api::routes::Services, service::cost_bearer::CostBearerService,
    service::expense_entry::ExpenseEntryService,
};

#[tokio::main]
async fn main() {
    let expense_entry_read_repo = Arc::new(ExpenseEntryReadSqliteRepository::new());
    let expense_entry_write_repo = Arc::new(ExpenseEntryWriteSqliteRepository::new());
    let expense_entry_service = Arc::new(ExpenseEntryService::new(
        expense_entry_read_repo,
        expense_entry_write_repo,
    ));

    let cost_bearer_read_repo = Arc::new(CostBearerReadSqliteRepository::new());
    let cost_bearer_write_repo = Arc::new(CostBearerWriteSqliteRepository::new());
    let cost_bearer_service = Arc::new(CostBearerService::new(
        cost_bearer_read_repo,
        cost_bearer_write_repo,
    ));

    let services = Services {
        expense_entry_service: expense_entry_service.clone(),
        cost_bearer_service: cost_bearer_service.clone(),
    };

    let router = setup_routing().await.with_state(services);
    let addr = "0.0.0.0:6570";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
