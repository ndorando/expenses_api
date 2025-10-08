use axum::extract::State;
use axum::{Json, extract::Path, http::StatusCode};
use uuid::Uuid;

use crate::api::routes::Services;
use crate::domain::expense_entry::ExpenseEntry;
use crate::service::application_error::ApplicationError;
use crate::service::expense_entry::ExpenseEntryNew;

pub async fn expense_entry_post(
    State(services): State<Services>,
    entry: Json<ExpenseEntryNew>,
) -> Result<Json<ExpenseEntry>, ApplicationError> {
    let new_entry_dto: ExpenseEntryNew = entry.0;
    let created_entry = services.expense_entry_service.create(new_entry_dto)?;
    Ok(Json(created_entry))
}

pub async fn expense_entry_update(
    State(services): State<Services>,
    Path(id): Path<Uuid>,
    entry: Json<ExpenseEntryNew>,
) -> Result<Json<ExpenseEntry>, ApplicationError> {
    let update_entry_dto: ExpenseEntryNew = entry.0;
    let updated_entry = services
        .expense_entry_service
        .update(id, update_entry_dto)?;
    Ok(Json(updated_entry))
}

pub async fn expense_entry_delete(
    State(services): State<Services>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApplicationError> {
    services.expense_entry_service.delete(id)?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn expense_entry_get(
    State(services): State<Services>,
    Path(id): Path<Uuid>,
) -> Result<Json<ExpenseEntry>, ApplicationError> {
    let found_entry = services.expense_entry_service.get(id)?;
    Ok(Json(found_entry))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        domain::cost_share::CostShare,
        repository::sqliterepository::{
            cost_bearer::{CostBearerReadSqliteRepository, CostBearerWriteSqliteRepository},
            expense_entry::{ExpenseEntryReadSqliteRepository, ExpenseEntryWriteSqliteRepository},
        },
        service::{cost_bearer::CostBearerService, expense_entry::ExpenseEntryService},
    };

    use super::*;
    use crate::test_util::test_utility::{TEST_INVALID_UUID, TEST_VALID_UUID};
    use axum::{
        Router,
        body::Body,
        http::{Method, Request, StatusCode},
        response::Response,
    };
    use chrono::TimeZone;
    use serde_json::json;
    use tower::ServiceExt;

    async fn setup_test_app() -> Router {
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

        crate::api::routes::setup_routing()
            .await
            .with_state(services)
    }

    async fn arrange_and_act_get_request(id: &str) -> Response<Body> {
        let app = setup_test_app().await;
        let uri = format!("/expense_entries/{}", id);

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

    async fn arrange_and_act_post_request(entry: String) -> Response<Body> {
        let app = setup_test_app().await;
        let uri = "/expense_entries";
        let body = Body::from(entry);

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
        let uri = format!("/expense_entries/{}", id);

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
    async fn expense_entry_get() {
        let response = arrange_and_act_get_request(&String::from(TEST_VALID_UUID)).await;

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to recieve body from response.");
        let entry: crate::domain::expense_entry::ExpenseEntry = serde_json::from_slice(&body)
            .expect("Failed to parse response into ExpenseEntry struct.");
        assert_eq!(entry.cost_shares().len(), 1);
        assert_eq!(entry.cost_shares().first().unwrap().amount, 12.5);
        assert!(!entry.expense_type().is_nil());
        assert_eq!(entry.description(), "Some Description");
    }

    #[tokio::test]
    async fn expense_entry_get_fails_not_found() {
        let response = arrange_and_act_get_request(&String::from(TEST_INVALID_UUID)).await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let error_message = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(error_message, "Expense entry not found.");
    }

    #[tokio::test]
    async fn expense_entry_get_fails_invalid_uuid() {
        let response = arrange_and_act_get_request("not-a-uuid").await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn expense_entry_post() {
        let cost_uuid = Uuid::new_v4();
        let expense_type_uuid = Uuid::new_v4();

        let new_expense_entry = ExpenseEntryNew {
            cost_shares: vec![CostShare {
                cost_bearer_id: cost_uuid,
                amount: 12.5,
            }],
            expense_type: expense_type_uuid,
            description: String::from("I bought something today."),
            expense_date: None,
        };
        let response = arrange_and_act_post_request(json!(new_expense_entry).to_string()).await;

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to recieve body from response.");
        let entry: crate::domain::expense_entry::ExpenseEntry = serde_json::from_slice(&body)
            .expect("Failed to parse response into ExpenseEntry struct.");

        assert_eq!(entry.cost_shares().len(), 1);
        assert_eq!(entry.cost_shares()[0].cost_bearer_id, cost_uuid);
        assert_eq!(entry.cost_shares()[0].amount, 12.5);
        assert_eq!(entry.expense_type(), expense_type_uuid);
        assert_eq!(entry.description(), "I bought something today.");

        let second_cost_uuid = Uuid::new_v4();
        let new_expense_entry = ExpenseEntryNew {
            cost_shares: vec![
                CostShare {
                    cost_bearer_id: cost_uuid,
                    amount: 12.5,
                },
                CostShare {
                    cost_bearer_id: second_cost_uuid,
                    amount: -12.5,
                },
            ],
            expense_type: expense_type_uuid,
            description: String::from("I bought something today."),
            expense_date: None,
        };
        let response = arrange_and_act_post_request(json!(new_expense_entry).to_string()).await;

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to recieve second body from response.");
        let entry: crate::domain::expense_entry::ExpenseEntry = serde_json::from_slice(&body)
            .expect("Failed to parse second response into ExpenseEntry struct.");

        assert_eq!(entry.cost_shares().len(), 2);
        assert_eq!(entry.cost_shares()[0].cost_bearer_id, cost_uuid);
        assert_eq!(entry.cost_shares()[0].amount, 12.5);
        assert_eq!(entry.cost_shares()[1].cost_bearer_id, second_cost_uuid);
        assert_eq!(entry.cost_shares()[1].amount, -12.5);

        assert_eq!(entry.expense_type(), expense_type_uuid);
        assert_eq!(entry.description(), "I bought something today.");
    }

    #[tokio::test]
    async fn expense_entry_post_with_explicit_expense_date() {
        let cost_uuid = Uuid::new_v4();
        let expense_type_uuid = Uuid::new_v4();

        // fixed date for deterministic assertion
        let explicit_date = chrono::Utc
            .with_ymd_and_hms(2024, 10, 1, 12, 30, 45)
            .unwrap();

        let new_expense_entry = ExpenseEntryNew {
            cost_shares: vec![CostShare {
                cost_bearer_id: cost_uuid,
                amount: 12.5,
            }],
            expense_type: expense_type_uuid,
            description: String::from("Dated explicitly"),
            expense_date: Some(explicit_date),
        };
        let response =
            arrange_and_act_post_request(serde_json::to_string(&new_expense_entry).unwrap()).await;

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to recieve body from response.");
        let entry: crate::domain::expense_entry::ExpenseEntry = serde_json::from_slice(&body)
            .expect("Failed to parse response into ExpenseEntry struct.");

        assert_eq!(entry.expense_date(), explicit_date);
        assert_eq!(entry.expense_type(), expense_type_uuid);
        assert_eq!(entry.description(), "Dated explicitly");
        assert_eq!(entry.cost_shares().len(), 1);
        assert_eq!(entry.cost_shares()[0].cost_bearer_id, cost_uuid);
        assert_eq!(entry.cost_shares()[0].amount, 12.5);
    }

    #[tokio::test]
    async fn expense_entry_post_with_none_uses_now() {
        let cost_uuid = Uuid::new_v4();
        let expense_type_uuid = Uuid::new_v4();

        let before = chrono::Utc::now();
        let new_expense_entry = ExpenseEntryNew {
            cost_shares: vec![CostShare {
                cost_bearer_id: cost_uuid,
                amount: 12.5,
            }],
            expense_type: expense_type_uuid,
            description: String::from("Implicit now date"),
            expense_date: None,
        };
        let response =
            arrange_and_act_post_request(serde_json::to_string(&new_expense_entry).unwrap()).await;
        let after = chrono::Utc::now();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to recieve body from response.");
        let entry: crate::domain::expense_entry::ExpenseEntry = serde_json::from_slice(&body)
            .expect("Failed to parse response into ExpenseEntry struct.");

        let ts = entry.expense_date();
        assert!(
            ts >= before && ts <= after,
            "timestamp {ts} not within [{before}, {after}]"
        );
        assert_eq!(entry.expense_type(), expense_type_uuid);
        assert_eq!(entry.description(), "Implicit now date");
        assert_eq!(entry.cost_shares().len(), 1);
        assert_eq!(entry.cost_shares()[0].cost_bearer_id, cost_uuid);
        assert_eq!(entry.cost_shares()[0].amount, 12.5);
    }

    #[tokio::test]
    async fn expense_entry_post_fails_invalid_json() {
        let invalid_json = String::from("{deliberately: invalid, json: parameter");
        let response = arrange_and_act_post_request(invalid_json).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to recieve body from response.");
        let my_str = str::from_utf8(&body);
        assert_eq!(
            my_str.unwrap(),
            "Failed to parse the request body as JSON: key must be a string at line 1 column 2"
        );
    }

    #[tokio::test]
    async fn expense_entry_post_fails_missing_cost_shares() {
        let expense_uuid = Uuid::new_v4();
        let missing_cost_shares_json = format!(
            r#"{{"expense_type": "{}", "description": "some"}}"#,
            expense_uuid
        );
        let response = arrange_and_act_post_request(missing_cost_shares_json).await;

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    // this can only happen once the DB is implemented
    /*#[tokio::test]
    async fn expense_entry_post_fails_invalid_cost_bearer_id() {
        let expense_uuid = Uuid::new_v4();
        let cost_bearer_uuid = Uuid::new_v4();
        let invalid_uuid_json = format!(
            r#"{{"cost_shares":[{{"cost_bearer_id": "{}", "amount": 12.5}}], "expense_type": "{}", "description": "something something"}}"#,
            cost_bearer_uuid,
            expense_uuid
        );
        let response = arrange_and_act_post_request(invalid_uuid_json).await;

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.expect("Failed to receive body from response.");
        let error_message = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(error_message, "Json without valid cost shares.");
    }*/

    #[tokio::test]
    async fn expense_entry_post_fails_nil_cost_bearer_id() {
        let expense_uuid = Uuid::new_v4();
        let cost_bearer_uuid = Uuid::nil();
        let nil_cost_bearer_json = format!(
            r#"{{"cost_shares":[{{"cost_bearer_id": "{}", "amount": 12.5}}], "expense_type": "{}", "description": "something something"}}"#,
            cost_bearer_uuid, expense_uuid
        );
        let response = arrange_and_act_post_request(nil_cost_bearer_json).await;

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to receive body from response.");
        let error_message = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(error_message, "Json without valid cost shares.");
    }

    #[tokio::test]
    async fn expense_entry_post_fails_duplicate_cost_bearer_ids() {
        let expense_uuid = Uuid::new_v4();
        let cost_bearer_uuid = Uuid::new_v4();
        let duplicate_cost_json = format!(
            r#"{{"cost_shares":[{{"cost_bearer_id": "{}", "amount": 12.5}}, {{"cost_bearer_id": "{}", "amount": -12.5}}], "expense_type": "{}", "description": "something something"}}"#,
            cost_bearer_uuid, cost_bearer_uuid, expense_uuid
        );
        let response = arrange_and_act_post_request(duplicate_cost_json).await;

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to receive body from response.");
        let error_message = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(error_message, "Json without valid cost shares.");
    }

    #[tokio::test]
    async fn expense_entry_post_fails_invalid_cost_amount() {
        let expense_uuid = Uuid::new_v4();
        let cost_bearer_uuid = Uuid::new_v4();
        let invalid_cost_json = format!(
            r#"{{"cost_shares":[{{"cost_bearer_id": "{}", "amount": 0.0}}], "expense_type": "{}", "description": "something something"}}"#,
            cost_bearer_uuid, expense_uuid
        );
        let response = arrange_and_act_post_request(invalid_cost_json).await;

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to receive body from response.");
        let error_message = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(error_message, "Json without valid cost shares.");
    }

    #[tokio::test]
    async fn expense_entry_post_fails_empty_cost_shares() {
        let expense_uuid = Uuid::new_v4();
        let empty_cost_json = format!(
            r#"{{"cost_shares":[], "expense_type": "{}", "description": "something something"}}"#,
            expense_uuid
        );
        let response = arrange_and_act_post_request(empty_cost_json).await;

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to receive body from response.");
        let error_message = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(error_message, "Json without valid cost shares.");
    }

    /*#[tokio::test]
    async fn expense_entry_post_fails_invalid_expense_id() {
        // needs DB
    }*/

    #[tokio::test]
    async fn expense_entry_post_fails_nil_expense_type() {
        // Test with Uuid::nil() for expense_type
        let expense_uuid = Uuid::nil();
        let cost_bearer_uuid = Uuid::new_v4();
        let invalid_expense_type_json = format!(
            r#"{{"cost_shares":[{{"cost_bearer_id": "{}", "amount": 12.5}}], "expense_type": "{}", "description": "Some description"}}"#,
            cost_bearer_uuid, expense_uuid
        );
        let response = arrange_and_act_post_request(invalid_expense_type_json).await;

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to receive body from response.");
        let error_message = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(error_message, "Json without valid expense id.");
    }

    #[tokio::test]
    async fn expense_entry_post_fails_invalid_description() {
        let expense_uuid = Uuid::new_v4();
        let cost_bearer_uuid = Uuid::new_v4();
        let invalid_description_json = format!(
            r#"{{"cost_shares":[{{"cost_bearer_id": "{}", "amount": 12.5}}], "expense_type": "{}", "description": ""}}"#,
            cost_bearer_uuid.clone(),
            expense_uuid.clone()
        );
        let response = arrange_and_act_post_request(invalid_description_json).await;

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        let body: axum::body::Bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to recieve body from response.");
        let error_message = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(error_message, "Json without valid description.");

        let whitespace_description_json = format!(
            r#"{{"cost_shares":[{{"cost_bearer_id": "{}", "amount": 12.5}}], "expense_type": "{}", "description": "   \t  "}}"#,
            cost_bearer_uuid, expense_uuid
        );
        let response = arrange_and_act_post_request(whitespace_description_json).await;

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        let body: axum::body::Bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to recieve body from response.");
        let error_message = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(error_message, "Json without valid description.");
    }

    #[tokio::test]
    async fn expense_entry_update() {}

    #[tokio::test]
    async fn expense_entry_update_fails() {}

    #[tokio::test]
    async fn expense_entry_delete() {
        let response = arrange_and_act_delete_request(&String::from(TEST_VALID_UUID)).await;

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to recieve body from response.");
        assert!(body.is_empty());

        // still todo: verify that the deletion actually took place in the DB
    }

    #[tokio::test]
    async fn expense_entry_delete_fails() {
        let response = arrange_and_act_delete_request(&String::from(TEST_INVALID_UUID)).await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body: axum::body::Bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to recieve body from response.");
        let error_message = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(error_message, "Expense entry not found.");
    }

    #[tokio::test]
    async fn expense_entry_delete_fails_invalid_uuid() {
        let response = arrange_and_act_delete_request("not-a-uuid").await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
