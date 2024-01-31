use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct Driver {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub business_name: Option<String>,
    pub address: Option<String>,
    pub iata_code: String,
    pub ic_number: String,
    pub vehicle_types: Vec<String>,
}
