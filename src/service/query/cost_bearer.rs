use uuid::Uuid;

use crate::domain::cost_bearer::CostBearer;
use crate::service::application_error::ApplicationError;
use crate::service::cost_bearer::CostBearerService;

impl CostBearerService {
    pub fn get(&self, id: Uuid) -> Result<CostBearer, ApplicationError> {
        self.read_repo.get(id)
    }
}
