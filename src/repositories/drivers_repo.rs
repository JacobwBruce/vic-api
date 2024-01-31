use crate::models::driver::Driver;
use sqlx::{MySql, Pool, Row};
use tracing::error;

#[derive(Clone)]
pub struct DriversRepository {
    pub db: &'static Pool<MySql>,
}

const BASE_QUERY: &str = "
  SELECT
    D.firstName, D.lastName, D.email, D.phone, D.address, D.businessName, D.icNumber,
    C.iataCode, GROUP_CONCAT(V.vehicleType) AS vehicleTypes
  FROM Driver D
  INNER JOIN City C ON D.cityId = C.id
  LEFT JOIN Vehicle V ON D.id = V.driverId
  GROUP BY D.id, D.firstName, D.lastName, D.email, D.phone, D.address, D.businessName, D.icNumber, C.iataCode
";

pub enum DriverError {
    // NotFound,
    Other,
}

pub struct Error {
    pub message: String,
    pub error: DriverError,
}

impl DriversRepository {
    pub async fn get_all_drivers(&self) -> Result<Vec<Driver>, Error> {
        let res = sqlx::query(&BASE_QUERY).fetch_all(self.db).await;

        match res {
            Ok(drivers) => {
                let drivers: Vec<Driver> = drivers
                    .iter()
                    .map(|driver| {
                        let vehicle_types_str: Option<String> = driver.get(8); // Assuming index 8 corresponds to the 'vehicleTypes' column
                        let vehicle_types: Vec<String> = match vehicle_types_str {
                            Some(types) => types.split(",").map(|s| s.to_string()).collect(),
                            None => vec![],
                        };

                        Driver {
                            first_name: driver.get(0),
                            last_name: driver.get(1),
                            email: driver.get(2),
                            phone: driver.get(3),
                            address: driver.get(4),
                            business_name: driver.get(5),
                            ic_number: driver.get(6),
                            iata_code: driver.get(7),
                            vehicle_types,
                        }
                    })
                    .collect();
                Ok(drivers)
            }
            Err(err) => {
                error!("Error getting drivers: {:?}", err);
                Err(Error {
                    message: "Internal Server Error".to_string(),
                    error: DriverError::Other,
                })
            }
        }
    }
}
