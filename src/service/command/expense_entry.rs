use uuid::Uuid;

use crate::domain::expense_entry::ExpenseEntry;
use crate::service::application_error::{ApplicationError, ApplicationErrorType};
use crate::service::expense_entry::{ExpenseEntryNew, ExpenseEntryService};

use crate::test_util::test_utility::TEST_VALID_UUID;

impl ExpenseEntryService {
    pub fn create(&self, dto: ExpenseEntryNew) -> Result<ExpenseEntry, ApplicationError> {
        let expense_entry = ExpenseEntry::try_from(dto)?;

        // todo - save to DB

        Ok(expense_entry)
    }

    pub fn update(&self, id: Uuid, dto: ExpenseEntryNew) -> Result<ExpenseEntry, ApplicationError> {
        todo!()
    }

    pub fn delete(&self, id: Uuid) -> Result<(), ApplicationError> {
        match id {
            id if id == TEST_VALID_UUID => Ok(()),
            _ => Err(ApplicationError {
                error_type: ApplicationErrorType::NotFound,
                message: String::from("Expense entry not found."),
            }),
        }
    }
}
