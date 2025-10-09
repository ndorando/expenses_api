use uuid::Uuid;

use crate::domain::cost_bearer::CostBearer;
use crate::service::application_error::ApplicationError;
use crate::service::cost_bearer::{CostBearerNew, CostBearerService};

impl CostBearerService {
    pub fn create(&self, dto: CostBearerNew) -> Result<CostBearer, ApplicationError> {
        let cost_bearer = CostBearer::try_from(dto)?;
        self.write_repo.insert(cost_bearer)
    }

    pub fn update(&self, _id: Uuid, _dto: CostBearerNew) -> Result<CostBearer, ApplicationError> {
        todo!()
    }

    pub fn delete(&self, id: Uuid) -> Result<(), ApplicationError> {
        self.write_repo.delete(id)
    }
}
