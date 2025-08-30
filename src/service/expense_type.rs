use crate::domain::expense_type::{ExpenseType, ExpenseTypeValidationError};
use crate::service::application_error::{ApplicationError, ApplicationErrorType};

// raw unvalidated user input DTO
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ExpenseTypeNew {
    pub name: String,
    pub description: String,
}

impl TryFrom<ExpenseTypeNew> for ExpenseType {
    type Error = ExpenseTypeValidationError;
    fn try_from(value: ExpenseTypeNew) -> Result<Self, ExpenseTypeValidationError> {
        ExpenseType::new(value.name, value.description)
    }
}

impl From<ExpenseTypeValidationError> for ApplicationError {
    fn from(err: ExpenseTypeValidationError) -> Self {
        match err {
            ExpenseTypeValidationError::MissingName =>
                ApplicationError { error_type: ApplicationErrorType::ValidationFailed, message: "Json without valid name.".to_string() },
            ExpenseTypeValidationError::MissingDescription =>
                ApplicationError { error_type: ApplicationErrorType::ValidationFailed, message: "Json without valid description.".to_string() },
            ExpenseTypeValidationError::DuplicateName =>
                ApplicationError { error_type: ApplicationErrorType::ValidationFailed, message: "Expense type with this name already exists.".to_string() },
        }
    }
}
