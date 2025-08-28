use chrono::{TimeZone, Utc};
use uuid::Uuid;

use crate::domain::cost_bearer::CostBearer;
use crate::service::application_error::{ApplicationError, ApplicationErrorType};

use crate::test_util::test_utility::TEST_VALID_UUID;

pub fn get(id: Uuid) -> Result<CostBearer, ApplicationError> {
    match id {
        id if id == TEST_VALID_UUID => Ok(
            CostBearer::new(String::from("Barclays Credit Card"), Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(), Some(Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap())).unwrap()),
        _ => Err(ApplicationError { error_type: ApplicationErrorType::NotFound, message: String::from("Cost Bearer not found.") })
    }
}
