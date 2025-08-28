use uuid::Uuid;

use crate::domain::expense_entry::ExpenseEntry;
use crate::service::application_error::{ApplicationError, ApplicationErrorType};
use crate::service::expense_entry::ExpenseEntryNew;

const TEST_EXPENSE_ENTRY_ID: &str = "123e4567-e89b-12d3-a456-426614174000";

pub fn create(dto: ExpenseEntryNew) -> Result<ExpenseEntry, ApplicationError> {
    let entry = ExpenseEntry::try_from(dto)?;

    // todo - save to DB
    
    Ok(entry)
}

pub fn update(id: Uuid, dto: ExpenseEntryNew) -> Result<ExpenseEntry, ApplicationError>  {
    todo!()
}

pub fn delete(id: Uuid) -> Result<(), ApplicationError> {
    let test_id = Uuid::parse_str(TEST_EXPENSE_ENTRY_ID).unwrap();
    match id {
        id if id == test_id => Ok(()),
        _ => Err(ApplicationError { error_type: ApplicationErrorType::NotFound, message: String::from("Expense entry not found.") })
    }
}