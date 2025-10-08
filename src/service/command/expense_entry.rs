use uuid::Uuid;

use crate::domain::expense_entry::ExpenseEntry;
use crate::service::application_error::ApplicationError;
use crate::service::expense_entry::{ExpenseEntryNew, ExpenseEntryService};

impl ExpenseEntryService {
    pub fn create(&self, dto: ExpenseEntryNew) -> Result<ExpenseEntry, ApplicationError> {
        let expense_entry = ExpenseEntry::try_from(dto)?;
        self.write_repo.insert(expense_entry)
    }

    pub fn update(&self, id: Uuid, dto: ExpenseEntryNew) -> Result<ExpenseEntry, ApplicationError> {
        todo!()
    }

    pub fn delete(&self, id: Uuid) -> Result<(), ApplicationError> {
        self.write_repo.delete(id)
    }
}
