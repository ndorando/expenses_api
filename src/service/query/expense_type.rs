use uuid::Uuid;

use crate::domain::expense_type::ExpenseType;
use crate::service::application_error::ApplicationError;
use crate::service::expense_type::ExpenseTypeService;

impl ExpenseTypeService {
    pub fn get(&self, id: Uuid) -> Result<ExpenseType, ApplicationError> {
        self.read_repo.get(id)
    }
}
