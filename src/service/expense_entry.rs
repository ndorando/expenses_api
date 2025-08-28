use uuid::Uuid;

use crate::{domain::expense_entry::{ExpenseEntry, ExpenseEntryValidationError}, service::application_error::{ApplicationError, ApplicationErrorType}, util::cost_share::CostShare};

// raw unvalidated user input DTO
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ExpenseEntryNew {
    pub cost_shares: Vec<CostShare>,
    pub expense_type: Uuid,
    pub description: String,
} 

impl TryFrom<ExpenseEntryNew> for ExpenseEntry {
    type Error = ExpenseEntryValidationError;
    fn try_from(new: ExpenseEntryNew) -> Result<Self, ExpenseEntryValidationError> {
        ExpenseEntry::new(new.cost_shares, new.expense_type, new.description)
    }
}

impl From<ExpenseEntryValidationError> for ApplicationError {
    fn from(err: ExpenseEntryValidationError) -> Self {
        match err {
            ExpenseEntryValidationError::DuplicateCostBearerIds =>
                ApplicationError { error_type: ApplicationErrorType::ValidationFailed, message: "Json without valid cost shares.".to_string() },
            ExpenseEntryValidationError::InvalidCostBearerId =>
                ApplicationError { error_type: ApplicationErrorType::ValidationFailed, message: "Json without valid cost shares.".to_string() },
            ExpenseEntryValidationError::InvalidExpenseTypeId =>
                ApplicationError { error_type: ApplicationErrorType::ValidationFailed, message: "Json without valid expense id.".to_string() },
            ExpenseEntryValidationError::MissingCostShares =>
                ApplicationError { error_type: ApplicationErrorType::ValidationFailed, message: "Json without valid cost shares.".to_string() },
            ExpenseEntryValidationError::MalformedCostShares =>
                ApplicationError { error_type: ApplicationErrorType::ValidationFailed, message: "Json without valid cost shares.".to_string() },
            ExpenseEntryValidationError::MissingExpenseType =>
                ApplicationError { error_type: ApplicationErrorType::ValidationFailed, message: "Json without valid expense id.".to_string() },
            ExpenseEntryValidationError::MissingDescription =>
                ApplicationError { error_type: ApplicationErrorType::ValidationFailed, message: "Json without valid description.".to_string() },
        }
    }
}
