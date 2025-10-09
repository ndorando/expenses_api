use std::sync::Arc;
use uuid::Uuid;

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
            ExpenseTypeValidationError::MissingName => ApplicationError {
                error_type: ApplicationErrorType::ValidationFailed,
                message: "Json without valid name.".to_string(),
            },
            ExpenseTypeValidationError::MissingDescription => ApplicationError {
                error_type: ApplicationErrorType::ValidationFailed,
                message: "Json without valid description.".to_string(),
            },
            ExpenseTypeValidationError::DuplicateName => ApplicationError {
                error_type: ApplicationErrorType::ValidationFailed,
                message: "Expense type with this name already exists.".to_string(),
            },
        }
    }
}

#[derive(Clone)]
pub struct ExpenseTypeService {
    pub(in crate::service) read_repo: Arc<dyn ExpenseTypeReadPort + Send + Sync>,
    pub(in crate::service) write_repo: Arc<dyn ExpenseTypeWritePort + Send + Sync>,
}

impl ExpenseTypeService {
    pub fn new(
        read_repo: Arc<dyn ExpenseTypeReadPort + Send + Sync>,
        write_repo: Arc<dyn ExpenseTypeWritePort + Send + Sync>,
    ) -> Self {
        ExpenseTypeService {
            read_repo,
            write_repo,
        }
    }
}

pub trait ExpenseTypeReadPort {
    fn get(&self, id: Uuid) -> Result<ExpenseType, ApplicationError>;
}

pub trait ExpenseTypeWritePort {
    fn insert(&self, entry: ExpenseType) -> Result<ExpenseType, ApplicationError>;
    fn update(&self, id: Uuid, entry: ExpenseType) -> Result<ExpenseType, ApplicationError>;
    fn delete(&self, id: Uuid) -> Result<(), ApplicationError>;
}
