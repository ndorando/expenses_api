use chrono::{DateTime, Utc};

use crate::{domain::cost_bearer::{CostBearer, CostBearerValidationError}, service::application_error::{ApplicationError, ApplicationErrorType}};

// raw unvalidated user input DTO
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CostBearerNew {
    name: String,
    exists_from: DateTime<Utc>,
    exists_to: Option<DateTime<Utc>>
} 

impl TryFrom<CostBearerNew> for CostBearer {
    type Error = CostBearerValidationError;
    fn try_from(value: CostBearerNew) -> Result<Self, CostBearerValidationError> {
        CostBearer::new(value.name, value.exists_from, value.exists_to)
    }
}

impl From<CostBearerValidationError> for ApplicationError {
    fn from(err: CostBearerValidationError) -> Self {
        match err {
            CostBearerValidationError::MissingName =>
                ApplicationError { error_type: ApplicationErrorType::ValidationFailed, message: "Json without valid name.".to_string() },
            CostBearerValidationError::InvalidDate =>
                ApplicationError { error_type: ApplicationErrorType::ValidationFailed, message: "Json without valid date.".to_string() },
        }
    }
}
