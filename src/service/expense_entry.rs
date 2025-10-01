use std::sync::Arc;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::cost_share::CostShare;
use crate::domain::expense_entry::{ExpenseEntry, ExpenseEntryValidationError};
use crate::service::application_error::{ApplicationError, ApplicationErrorType};

// raw unvalidated user input DTO
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ExpenseEntryNew {
    pub cost_shares: Vec<CostShare>,
    pub expense_type: Uuid,
    pub description: String,
    pub expense_date: Option<DateTime<Utc>>,
}

impl TryFrom<ExpenseEntryNew> for ExpenseEntry {
    type Error = ExpenseEntryValidationError;
    fn try_from(new: ExpenseEntryNew) -> Result<Self, ExpenseEntryValidationError> {
        ExpenseEntry::new(
            new.cost_shares,
            new.expense_type,
            new.description,
            new.expense_date,
        )
    }
}

impl From<ExpenseEntryValidationError> for ApplicationError {
    fn from(err: ExpenseEntryValidationError) -> Self {
        match err {
            ExpenseEntryValidationError::DuplicateCostBearerIds(uuid) => ApplicationError {
                error_type: ApplicationErrorType::ValidationFailed,
                message: "Json without valid cost shares.".to_string(),
            },
            ExpenseEntryValidationError::InvalidCostBearerId(uuid) => ApplicationError {
                error_type: ApplicationErrorType::ValidationFailed,
                message: "Json without valid cost shares.".to_string(),
            },
            ExpenseEntryValidationError::InvalidExpenseTypeId(uuid) => ApplicationError {
                error_type: ApplicationErrorType::ValidationFailed,
                message: "Json without valid expense id.".to_string(),
            },
            ExpenseEntryValidationError::MissingCostShares => ApplicationError {
                error_type: ApplicationErrorType::ValidationFailed,
                message: "Json without valid cost shares.".to_string(),
            },
            ExpenseEntryValidationError::MalformedCostShares => ApplicationError {
                error_type: ApplicationErrorType::ValidationFailed,
                message: "Json without valid cost shares.".to_string(),
            },
            ExpenseEntryValidationError::MissingExpenseType => ApplicationError {
                error_type: ApplicationErrorType::ValidationFailed,
                message: "Json without valid expense id.".to_string(),
            },
            ExpenseEntryValidationError::MissingDescription => ApplicationError {
                error_type: ApplicationErrorType::ValidationFailed,
                message: "Json without valid description.".to_string(),
            },
        }
    }
}

#[derive(Clone)]
pub struct ExpenseEntryService {
    pub read_repo: Arc<dyn ExpenseEntryReadPort + Send + Sync>,
}

impl ExpenseEntryService {
    pub fn new(read_repo: Arc<dyn ExpenseEntryReadPort + Send + Sync>) -> Self {
        ExpenseEntryService { read_repo }
    }
}

pub trait ExpenseEntryReadPort {
    fn get(&self, id: Uuid) -> Result<ExpenseEntry, ApplicationError>;
}

/*pub trait ExpenseEntryWritePort {
    pub fn insert();
    pub fn update();
    pub fn delete();
}*/
