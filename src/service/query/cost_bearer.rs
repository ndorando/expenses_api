use chrono::{TimeZone, Utc};
use uuid::Uuid;

use crate::{domain::cost_bearer::CostBearer, service::application_error::{ApplicationError, ApplicationErrorType}};

const TEST_COST_BEARER_ID: &str = "123e4567-e89b-12d3-a456-426614174000";

pub fn get(id: Uuid) -> Result<CostBearer, ApplicationError> {
    let test_id = Uuid::parse_str(TEST_COST_BEARER_ID).unwrap();
    match id {
        id if id == test_id => Ok(
            CostBearer::new(String::from("Barclays Credit Card"), Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(), Some(Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap())).unwrap()),
        _ => Err(ApplicationError { error_type: ApplicationErrorType::NotFound, message: String::from("Cost Bearer not found.") })
    }
}
