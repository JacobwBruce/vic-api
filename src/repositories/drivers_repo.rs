use crate::models::driver::Driver;
use sqlx::{mysql::MySqlRow, MySql, Pool, Row};
use tracing::error;
use tracing_subscriber::fmt::format;

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
";

const GROUP_BY_QUERY: &str = "GROUP BY D.id, D.firstName, D.lastName, D.email, D.phone, D.address, D.businessName, D.icNumber, C.iataCode";

pub enum DriverError {
    NotFound,
    Other,
}

pub struct Error {
    pub message: String,
    pub error: DriverError,
}

fn get_driver_from_row(row: &MySqlRow) -> Driver {
    let vehicle_types_str: Option<String> = row.get(8); // Assuming index 8 corresponds to the 'vehicleTypes' column
    let vehicle_types: Vec<String> = match vehicle_types_str {
        Some(types) => types.split(",").map(|s| s.to_string()).collect(),
        None => vec![],
    };

    Driver {
        first_name: row.get(0),
        last_name: row.get(1),
        email: row.get(2),
        phone: row.get(3),
        address: row.get(4),
        business_name: row.get(5),
        ic_number: row.get(6),
        iata_code: row.get(7),
        vehicle_types,
    }
}

impl DriversRepository {
    pub async fn get_all_drivers(&self) -> Result<Vec<Driver>, Error> {
        let query = format!("{} {}", BASE_QUERY, GROUP_BY_QUERY);
        let res = sqlx::query(&query).fetch_all(self.db).await;

        match res {
            Ok(drivers) => {
                let drivers: Vec<Driver> = drivers
                    .iter()
                    .map(|row| get_driver_from_row(&row))
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

    pub async fn get_driver_by_phone_number(&self, phone_number: &str) -> Result<Driver, Error> {
        let query = format!("{} WHERE D.phone = ? {}", BASE_QUERY, GROUP_BY_QUERY);
        let res = sqlx::query(&query)
            .bind(phone_number)
            .fetch_one(self.db)
            .await;

        match res {
            Ok(row) => {
                let driver = get_driver_from_row(&row);
                Ok(driver)
            }
            Err(err) => {
                error!("Error getting driver by phone number: {:?}", err);

                if let sqlx::Error::RowNotFound = err {
                    return Err(Error {
                        message: format!("Driver with phone number {} not found", phone_number),
                        error: DriverError::NotFound,
                    });
                }
                Err(Error {
                    message: "Internal Server Error".to_string(),
                    error: DriverError::Other,
                })
            }
        }
    }
}
