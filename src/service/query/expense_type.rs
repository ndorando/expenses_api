use uuid::Uuid;

use crate::domain::expense_type::ExpenseType;
use crate::service::application_error::{ApplicationError, ApplicationErrorType};

use crate::test_util::test_utility::TEST_VALID_UUID;

pub fn get(id: Uuid) -> Result<ExpenseType, ApplicationError> {
    match id {
        id if id == TEST_VALID_UUID => Ok(ExpenseType::new(
            String::from("Food"),
            String::from("Expenses related to food and dining"),
        )
        .unwrap()),
        _ => Err(ApplicationError {
            error_type: ApplicationErrorType::NotFound,
            message: String::from("Expense type not found."),
        }),
    }
}
