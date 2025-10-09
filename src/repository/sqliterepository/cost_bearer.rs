use chrono::{TimeZone, Utc};
use uuid::Uuid;

use crate::{
    domain::cost_bearer::CostBearer,
    service::{
        application_error::{ApplicationError, ApplicationErrorType},
        cost_bearer::{CostBearerReadPort, CostBearerWritePort},
    },
    test_util::test_utility::TEST_VALID_UUID,
};

pub struct CostBearerReadSqliteRepository {}
pub struct CostBearerWriteSqliteRepository {}

impl CostBearerReadSqliteRepository {
    pub fn new() -> Self {
        CostBearerReadSqliteRepository {}
    }
}

impl CostBearerWriteSqliteRepository {
    pub fn new() -> Self {
        CostBearerWriteSqliteRepository {}
    }
}

impl CostBearerReadPort for CostBearerReadSqliteRepository {
    fn get(&self, id: Uuid) -> Result<CostBearer, ApplicationError> {
        match id {
            id if id == TEST_VALID_UUID => Ok(CostBearer::new(
                String::from("Barclays Credit Card"),
                Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap(),
            )
            .unwrap()),
            _ => Err(ApplicationError {
                error_type: ApplicationErrorType::NotFound,
                message: String::from("Cost bearer not found."),
            }),
        }
    }
}

impl CostBearerWritePort for CostBearerWriteSqliteRepository {
    fn insert(&self, entry: CostBearer) -> Result<CostBearer, ApplicationError> {
        Ok(entry)
    }

    fn update(&self, _id: Uuid, _entry: CostBearer) -> Result<CostBearer, ApplicationError> {
        todo!()
    }

    fn delete(&self, id: Uuid) -> Result<(), ApplicationError> {
        match id {
            id if id == TEST_VALID_UUID => Ok(()),
            _ => Err(ApplicationError {
                error_type: ApplicationErrorType::NotFound,
                message: String::from("Cost bearer not found."),
            }),
        }
    }
}
