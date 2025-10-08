use chrono::{DateTime, Utc};
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::cost_bearer::{CostBearer, CostBearerValidationError};
use crate::service::application_error::{ApplicationError, ApplicationErrorType};

// raw unvalidated user input DTO
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CostBearerNew {
    pub name: String,
    pub exists_from: DateTime<Utc>,
    pub exists_to: Option<DateTime<Utc>>,
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
            CostBearerValidationError::MissingName => ApplicationError {
                error_type: ApplicationErrorType::ValidationFailed,
                message: "Json without valid name.".to_string(),
            },
            CostBearerValidationError::InvalidDate => ApplicationError {
                error_type: ApplicationErrorType::ValidationFailed,
                message: "Json without valid date.".to_string(),
            },
        }
    }
}

#[derive(Clone)]
pub struct CostBearerService {
    pub(in crate::service) read_repo: Arc<dyn CostBearerReadPort + Send + Sync>,
    pub(in crate::service) write_repo: Arc<dyn CostBearerWritePort + Send + Sync>,
}

impl CostBearerService {
    pub fn new(
        read_repo: Arc<dyn CostBearerReadPort + Send + Sync>,
        write_repo: Arc<dyn CostBearerWritePort + Send + Sync>,
    ) -> Self {
        CostBearerService {
            read_repo,
            write_repo,
        }
    }
}

pub trait CostBearerReadPort {
    fn get(&self, id: Uuid) -> Result<CostBearer, ApplicationError>;
}

pub trait CostBearerWritePort {
    fn insert(&self, entry: CostBearer) -> Result<CostBearer, ApplicationError>;
    fn update(&self, id: Uuid, entry: CostBearer) -> Result<CostBearer, ApplicationError>;
    fn delete(&self, id: Uuid) -> Result<(), ApplicationError>;
}
