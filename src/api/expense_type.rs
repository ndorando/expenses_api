use axum::{Json, extract::Path, extract::State, http::StatusCode};
use uuid::Uuid;

use crate::api::routes::Services;
use crate::domain::expense_type::ExpenseType;
use crate::service::application_error::ApplicationError;
use crate::service::expense_type::ExpenseTypeNew;

pub async fn expense_type_post(
    State(services): State<Services>,
    entry: Json<ExpenseTypeNew>,
) -> Result<Json<ExpenseType>, ApplicationError> {
    let new_entry_dto: ExpenseTypeNew = entry.0;
    let created_entry = services.expense_type_service.create(new_entry_dto)?;
    Ok(Json(created_entry))
}

pub async fn expense_type_update(
    State(services): State<Services>,
    Path(id): Path<Uuid>,
    entry: Json<ExpenseTypeNew>,
) -> Result<Json<ExpenseType>, ApplicationError> {
    let update_entry_dto: ExpenseTypeNew = entry.0;
    let updated_entry = services.expense_type_service.update(id, update_entry_dto)?;
    Ok(Json(updated_entry))
}

pub async fn expense_type_delete(
    State(services): State<Services>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApplicationError> {
    services.expense_type_service.delete(id)?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn expense_type_get(
    State(services): State<Services>,
    Path(id): Path<Uuid>,
) -> Result<Json<ExpenseType>, ApplicationError> {
    let found_entry = services.expense_type_service.get(id)?;
    Ok(Json(found_entry))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::api::routes::Services;
    use crate::repository::sqliterepository::{
        cost_bearer::{CostBearerReadSqliteRepository, CostBearerWriteSqliteRepository},
        expense_entry::{ExpenseEntryReadSqliteRepository, ExpenseEntryWriteSqliteRepository},
        expense_type::{ExpenseTypeReadSqliteRepository, ExpenseTypeWriteSqliteRepository},
    };
    use crate::service::expense_type::ExpenseTypeNew;
    use crate::service::{
        cost_bearer::CostBearerService, expense_entry::ExpenseEntryService,
        expense_type::ExpenseTypeService,
    };
    use crate::test_util::test_utility::{TEST_INVALID_UUID, TEST_VALID_UUID};
    use axum::Router;
    use axum::{
        body::Body,
        http::{Method, Request, StatusCode},
        response::Response,
    };
    use serde_json::json;
    use tower::ServiceExt;

    async fn setup_test_app() -> Router {
        let expense_entry_read_repo = Arc::new(ExpenseEntryReadSqliteRepository::new());
        let expense_entry_write_repo = Arc::new(ExpenseEntryWriteSqliteRepository::new());
        let expense_entry_service = Arc::new(ExpenseEntryService::new(
            expense_entry_read_repo,
            expense_entry_write_repo,
        ));

        let expense_type_read_repo = Arc::new(ExpenseTypeReadSqliteRepository::new());
        let expense_type_write_repo = Arc::new(ExpenseTypeWriteSqliteRepository::new());
        let expense_type_service = Arc::new(ExpenseTypeService::new(
            expense_type_read_repo,
            expense_type_write_repo,
        ));

        let cost_bearer_read_repo = Arc::new(CostBearerReadSqliteRepository::new());
        let cost_bearer_write_repo = Arc::new(CostBearerWriteSqliteRepository::new());
        let cost_bearer_service = Arc::new(CostBearerService::new(
            cost_bearer_read_repo,
            cost_bearer_write_repo,
        ));

        let services = Services {
            expense_entry_service: expense_entry_service.clone(),
            expense_type_service: expense_type_service.clone(),
            cost_bearer_service: cost_bearer_service.clone(),
        };

        crate::api::routes::setup_routing()
            .await
            .with_state(services)
    }

    async fn arrange_and_act_get_request(id: &str) -> Response<Body> {
        let app = setup_test_app().await;
        let uri = format!("/expense_types/{}", id);

        let request = Request::builder()
            .method(Method::GET)
            .uri(&uri)
            .body(Body::empty())
            .expect("Failed to finalize request.");

        let response = app
            .oneshot(request)
            .await
            .expect("Failed to receive response.");

        response
    }

    async fn arrange_and_act_post_request(expense_type: String) -> Response<Body> {
        let app = setup_test_app().await;
        let uri = "/expense_types";
        let body = Body::from(expense_type);

        let request = Request::builder()
            .method(Method::POST)
            .uri(uri)
            .header("content-type", "application/json")
            .body(body)
            .expect("Failed to finalize request.");

        let response = app
            .oneshot(request)
            .await
            .expect("Failed to receive response.");

        response
    }

    async fn arrange_and_act_delete_request(id: &str) -> Response<Body> {
        let app = setup_test_app().await;
        let uri = format!("/expense_types/{}", id);

        let request = Request::builder()
            .method(Method::DELETE)
            .uri(&uri)
            .body(Body::empty())
            .expect("Failed to finalize request.");

        let response = app
            .oneshot(request)
            .await
            .expect("Failed to receive response.");

        response
    }

    #[tokio::test]
    async fn expense_type_get() {
        let response = arrange_and_act_get_request(&String::from(TEST_VALID_UUID)).await;

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to receive body from response.");
        let expense_type: crate::domain::expense_type::ExpenseType = serde_json::from_slice(&body)
            .expect("Failed to parse response into ExpenseType struct.");
        assert_eq!(expense_type.name(), "Food");
        assert_eq!(
            expense_type.description(),
            "Expenses related to food and dining"
        );
        assert!(!expense_type.id().is_nil());
    }

    #[tokio::test]
    async fn expense_type_get_fails_not_found() {
        let response = arrange_and_act_get_request(&String::from(TEST_INVALID_UUID)).await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let error_message = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(error_message, "Expense type not found.");
    }

    #[tokio::test]
    async fn expense_type_get_fails_invalid_uuid() {
        let response = arrange_and_act_get_request("not-a-uuid").await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn expense_type_post() {
        let new_expense_type = ExpenseTypeNew {
            name: String::from("Transportation"),
            description: String::from("Expenses related to transportation and travel"),
        };
        let response = arrange_and_act_post_request(json!(new_expense_type).to_string()).await;

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to receive body from response.");
        let expense_type: crate::domain::expense_type::ExpenseType = serde_json::from_slice(&body)
            .expect("Failed to parse response into ExpenseType struct.");

        assert_eq!(expense_type.name(), "Transportation");
        assert_eq!(
            expense_type.description(),
            "Expenses related to transportation and travel"
        );
        assert!(!expense_type.id().is_nil());
    }

    #[tokio::test]
    async fn expense_type_post_fails_invalid_json() {
        let invalid_json = String::from("{deliberately: invalid, json: parameter");
        let response = arrange_and_act_post_request(invalid_json).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to receive body from response.");
        let my_str = str::from_utf8(&body);
        assert_eq!(
            my_str.unwrap(),
            "Failed to parse the request body as JSON: key must be a string at line 1 column 2"
        );
    }

    #[tokio::test]
    async fn expense_type_post_fails_empty_name() {
        let empty_name_json = r#"{"name": "", "description": "Some description"}"#;
        let response = arrange_and_act_post_request(empty_name_json.to_string()).await;

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to receive body from response.");
        let error_message = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(error_message, "Json without valid name.");
    }

    #[tokio::test]
    async fn expense_type_post_fails_whitespace_name() {
        let whitespace_name_json = r#"{"name": "   \t  ", "description": "Some description"}"#;
        let response = arrange_and_act_post_request(whitespace_name_json.to_string()).await;

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to receive body from response.");
        let error_message = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(error_message, "Json without valid name.");
    }

    #[tokio::test]
    async fn expense_type_post_fails_empty_description() {
        let empty_description_json = r#"{"name": "Some name", "description": ""}"#;
        let response = arrange_and_act_post_request(empty_description_json.to_string()).await;

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to receive body from response.");
        let error_message = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(error_message, "Json without valid description.");
    }

    #[tokio::test]
    async fn expense_type_post_fails_whitespace_description() {
        let whitespace_description_json = r#"{"name": "Some name", "description": "   \t  "}"#;
        let response = arrange_and_act_post_request(whitespace_description_json.to_string()).await;

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to receive body from response.");
        let error_message = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(error_message, "Json without valid description.");
    }

    // this can only happen once the DB is implemented
    /*#[tokio::test]
    async fn expense_type_post_fails_duplicate_name() {
        // First, create an expense type
        let first_expense_type = ExpenseTypeNew{
            name: String::from("Food"),
            description: String::from("First food expense type")
        };
        let response = arrange_and_act_post_request(json!(first_expense_type).to_string()).await;
        assert_eq!(response.status(), StatusCode::OK);

        // Then try to create another with the same name
        let duplicate_expense_type = ExpenseTypeNew{
            name: String::from("Food"),
            description: String::from("Second food expense type")
        };
        let response = arrange_and_act_post_request(json!(duplicate_expense_type).to_string()).await;
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.expect("Failed to receive body from response.");
        let error_message = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(error_message, "Expense type with this name already exists.");
    }*/

    #[tokio::test]
    async fn expense_type_update() {
        // TODO: Implement once the update function is implemented in the service layer
        // This test will verify that updating an expense type works correctly
    }

    #[tokio::test]
    async fn expense_type_update_fails() {
        // TODO: Implement once the update function is implemented in the service layer
        // This test will verify that updating an expense type fails appropriately
    }

    #[tokio::test]
    async fn expense_type_delete() {
        let response = arrange_and_act_delete_request(&String::from(TEST_VALID_UUID)).await;

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to receive body from response.");
        assert!(body.is_empty());

        // still todo: verify that the deletion actually took place in the DB
    }

    #[tokio::test]
    async fn expense_type_delete_fails() {
        let response = arrange_and_act_delete_request(&String::from(TEST_INVALID_UUID)).await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body: axum::body::Bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to receive body from response.");
        let error_message = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(error_message, "Expense type not found.");
    }

    #[tokio::test]
    async fn expense_type_delete_fails_invalid_uuid() {
        let response = arrange_and_act_delete_request("not-a-uuid").await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
