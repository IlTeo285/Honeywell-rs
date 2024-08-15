use std::time::Duration;
use log::{info, error};
use honeywell_rs::{TotalComfort, Authentication, TotalComfortError, Device};

#[derive(Debug)]
struct DeviceLite{
    name: String, 
    temperature: f64,
    mode: String,
    heatpoint: f64,
    heatpoint_status: String
}

impl From<&Device> for DeviceLite {
    fn from(value: &Device) -> Self {
        Self {
            name: value.name.clone(),
            temperature: value.thermostat.indoor_temperature,
            mode: value.thermostat.changeable_values.mode.clone(),
            heatpoint: value.thermostat.changeable_values.heat_setpoint.value,
            heatpoint_status: value.thermostat.changeable_values.heat_setpoint.status.clone()
        }
    }
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut hw = TotalComfort::new();
    let auth = Authentication::from_env()?;
    hw.authenticate(auth).await?;

    loop{
        match hw.get_locations().await {
            Ok(location) => {
                let devices_names: Vec<DeviceLite> = location[0].devices.iter().map(|dev|{
                    DeviceLite::from(dev)
                }).collect();

                info!("{devices_names:?}");
                tokio::time::sleep(Duration::from_secs(60)).await;
            },
            Err(TotalComfortError::AuthenticationFailed(_)) => {
                error!("Authentication token expired... try renew");
                while let Err(_) = hw.renew().await {
                    error!("Renew failed");
                    tokio::time::sleep(Duration::from_secs(5*60)).await;
                }
                info!("Authentication renew");
            },
            Err(e) => {
                error!("{e}");
            }
        };   

        hw.renew().await;     
    }
}