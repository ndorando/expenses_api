use axum::{extract::Path, http::StatusCode, Json};
use uuid::Uuid;

use crate::domain::cost_bearer::CostBearer;
use crate::service::application_error::ApplicationError;
use crate::service::cost_bearer::CostBearerNew;

pub async fn cost_bearer_post(entry: Json<CostBearerNew>) -> Result<Json<CostBearer>, ApplicationError> {
    let new_entry_dto: CostBearerNew = entry.0;
    let created_entry = crate::service::command::cost_bearer::create(new_entry_dto)?;
    Ok(Json(created_entry))
}

pub async fn cost_bearer_update(Path(id): Path<Uuid>, entry: Json<CostBearerNew>) -> Result<Json<CostBearer>, ApplicationError> {
    let update_entry_dto: CostBearerNew = entry.0;
    let updated_entry = crate::service::command::cost_bearer::update(id, update_entry_dto)?;
    Ok(Json(updated_entry))
}

pub async fn cost_bearer_delete(Path(id): Path<Uuid>) -> Result<StatusCode, ApplicationError> {
    crate::service::command::cost_bearer::delete(id)?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn cost_bearer_get(Path(id): Path<Uuid>) -> Result<Json<CostBearer>, ApplicationError> {
    let found_entry = crate::service::query::cost_bearer::get(id)?;
    Ok(Json(found_entry))
}


# [cfg(test)]
mod tests {
    use axum::{body::Body, http::{Method, Request, StatusCode}, response::Response};
    use chrono::{TimeZone, Utc};
    use serde_json::json;
    use tower::ServiceExt;
    use crate::test_util::test_utility::{TEST_VALID_UUID, TEST_INVALID_UUID};

    async fn arrange_and_act_get_request(id: &str) -> Response<Body> {
        let app = crate::api::routes::setup_routing().await;
        let uri = format!("/cost_bearers/{}", id);

        let request = Request::builder()
                                .method(Method::GET)
                                .uri(&uri)
                                .body(Body::empty())
                                .expect("Failed to finalize request.");

        let response = app.oneshot(request).await.expect("Failed to receive response.");

        response
    }

    async fn arrange_and_act_post_request(entry: String) -> Response<Body> {
        let app = crate::api::routes::setup_routing().await;
        let uri = "/cost_bearers";
        let body = Body::from(entry);

        let request = Request::builder()
                                .method(Method::POST)
                                .uri(uri)
                                .header("content-type", "application/json")
                                .body(body)
                                .expect("Failed to finalize request.");

        let response = app.oneshot(request).await.expect("Failed to receive response.");

        response
    }

    async fn arrange_and_act_delete_request(id: &str) -> Response<Body> {
        let app = crate::api::routes::setup_routing().await;
        let uri = format!("/cost_bearers/{}", id);

        let request = Request::builder()
                                .method(Method::DELETE)
                                .uri(&uri)
                                .body(Body::empty())
                                .expect("Failed to finalize request.");

        let response = app.oneshot(request).await.expect("Failed to receive response.");

        response
    }

    #[tokio::test]
    async fn cost_bearer_get() {
        let response = arrange_and_act_get_request(&String::from(TEST_VALID_UUID)).await;

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.expect("Failed to recieve body from response.");
        let bearer: crate::domain::cost_bearer::CostBearer = serde_json::from_slice(&body).expect("Failed to parse response into ExpenseEntry struct."); 
        assert_eq!(bearer.name(), "Barclays Credit Card");
        assert_eq!(bearer.exists_from(), Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap());
        assert_eq!(bearer.exists_to().unwrap(), Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap());
    }

    #[tokio::test]
    async fn cost_bearer_get_fails_not_found() {
        let response = arrange_and_act_get_request(&String::from(TEST_INVALID_UUID)).await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let error_message = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(error_message, "Cost Bearer not found.");
    }

    #[tokio::test]
    async fn cost_bearer_get_fails_invalid_uuid() {
        let response = arrange_and_act_get_request("not-a-uuid").await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn cost_bearer_post() {
        let exists_from = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let exists_to = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();

        let new_cost_bearer = json!({
            "name": "Barclays Credit Card",
            "exists_from": exists_from,
            "exists_to": exists_to
        });

        let response = arrange_and_act_post_request(new_cost_bearer.to_string()).await;

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.expect("Failed to recieve body from response.");
        let bearer: crate::domain::cost_bearer::CostBearer = serde_json::from_slice(&body).expect("Failed to parse response into CostBearer struct.");

        assert_eq!(bearer.name(), "Barclays Credit Card");
        assert_eq!(bearer.exists_from(), exists_from);
        assert_eq!(bearer.exists_to().unwrap(), exists_to);

        // second payload without exists_to
        let new_cost_bearer = json!({
            "name": "Barclays Credit Card",
            "exists_from": exists_from
        });

        let response = arrange_and_act_post_request(new_cost_bearer.to_string()).await;

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.expect("Failed to recieve second body from response.");
        let bearer: crate::domain::cost_bearer::CostBearer = serde_json::from_slice(&body).expect("Failed to parse second response into CostBearer struct.");

        assert_eq!(bearer.name(), "Barclays Credit Card");
        assert_eq!(bearer.exists_from(), exists_from);
        assert_eq!(bearer.exists_to(), None);

        // third payload with exist_to set to null
        let new_cost_bearer = json!({
            "name": "Barclays Credit Card",
            "exists_from": exists_from,
            "exists_to": null
        });

        let response = arrange_and_act_post_request(new_cost_bearer.to_string()).await;

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.expect("Failed to recieve body from response.");
        let bearer: crate::domain::cost_bearer::CostBearer = serde_json::from_slice(&body).expect("Failed to parse response into CostBearer struct.");

        assert_eq!(bearer.name(), "Barclays Credit Card");
        assert_eq!(bearer.exists_from(), exists_from);
        assert_eq!(bearer.exists_to(), None);
    }

    #[tokio::test]
    async fn cost_bearer_post_fails_invalid_json() {
        let invalid_json = String::from("{deliberately: invalid, json: parameter");
        let response = arrange_and_act_post_request(invalid_json).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.expect("Failed to recieve body from response.");
        let my_str = str::from_utf8(&body);
        assert_eq!(my_str.unwrap(), "Failed to parse the request body as JSON: key must be a string at line 1 column 2");
    }

    #[tokio::test]
    async fn cost_bearer_post_fails_invalid_name() {
        let exists_from = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let exists_to = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();

        let empty_name_json = json!({
            "name": "",
            "exists_from": exists_from,
            "exists_to": exists_to
        }).to_string();
        let response = arrange_and_act_post_request(empty_name_json).await;

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.expect("Failed to receive body from response.");
        let error_message = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(error_message, "Json without valid name.");

        let whitespace_name_json = json!({
            "name": "   \t  ",
            "exists_from": exists_from,
            "exists_to": exists_to
        }).to_string();
        let response = arrange_and_act_post_request(whitespace_name_json).await;

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.expect("Failed to receive body from response.");
        let error_message = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(error_message, "Json without valid name.");
    }

    #[tokio::test]
    async fn cost_bearer_post_fails_missing_exists_from() {
        let new_cost_bearer = json!({
            "name": "Barclays Credit Card"
        }).to_string();
        let response = arrange_and_act_post_request(new_cost_bearer).await;

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn cost_bearer_post_fails_wrong_type_for_name() {
        let exists_from = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let wrong_type_json = json!({
            "name": 12345,
            "exists_from": exists_from
        }).to_string();
        let response = arrange_and_act_post_request(wrong_type_json).await;

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn cost_bearer_post_fails_invalid_date_order() {
        let exists_from = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
        let exists_to = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let invalid_dates_json = json!({
            "name": "Barclays Credit Card",
            "exists_from": exists_from,
            "exists_to": exists_to
        }).to_string();
        let response = arrange_and_act_post_request(invalid_dates_json).await;

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn cost_bearer_update() {

    }

    #[tokio::test]
    async fn cost_bearer_update_fails() {

    }

    #[tokio::test]
    async fn cost_bearer_delete() {
        let response = arrange_and_act_delete_request(&String::from(TEST_VALID_UUID)).await;

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.expect("Failed to recieve body from response.");
        assert!(body.is_empty());
    }

    #[tokio::test]
    async fn cost_bearer_delete_fails() {
        let response = arrange_and_act_delete_request(&String::from(TEST_INVALID_UUID)).await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body: axum::body::Bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.expect("Failed to recieve body from response.");
        let error_message = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(error_message, "Cost Bearer not found.");
    }

    #[tokio::test]
    async fn cost_bearer_delete_fails_invalid_uuid() {
        let response = arrange_and_act_delete_request("not-a-uuid").await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}