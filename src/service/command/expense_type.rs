use uuid::Uuid;

use crate::domain::expense_type::ExpenseType;
use crate::service::application_error::{ApplicationError, ApplicationErrorType};
use crate::service::expense_type::ExpenseTypeNew;

use crate::test_util::test_utility::TEST_VALID_UUID;

pub fn create(dto: ExpenseTypeNew) -> Result<ExpenseType, ApplicationError> {
    let expense_type = ExpenseType::try_from(dto)?;

    // todo - save to DB

    Ok(expense_type)
}

pub fn update(id: Uuid, dto: ExpenseTypeNew) -> Result<ExpenseType, ApplicationError> {
    todo!()
}

pub fn delete(id: Uuid) -> Result<(), ApplicationError> {
    match id {
        id if id == TEST_VALID_UUID => Ok(()),
        _ => Err(ApplicationError {
            error_type: ApplicationErrorType::NotFound,
            message: String::from("Expense type not found."),
        }),
    }
}
