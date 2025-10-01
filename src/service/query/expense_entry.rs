use uuid::Uuid;

use crate::domain::expense_entry::ExpenseEntry;
use crate::service::application_error::ApplicationError;

use crate::service::expense_entry::ExpenseEntryService;

impl ExpenseEntryService {
    pub fn get(&self, id: Uuid) -> Result<ExpenseEntry, ApplicationError> {
        self.read_repo.get(id)
    }
}
