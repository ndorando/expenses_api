use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
pub struct CostShare {
    pub cost_bearer_id: Uuid,
    pub amount: f64,
}