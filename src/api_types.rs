use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    #[serde(rename = "userID")]
    pub user_id: u64,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub street_address: String,
    pub city: String,
    pub zipcode: String,
    pub country: String,
    pub telephone: String,
    pub user_language: String,
    pub is_activated: bool,
    pub device_count: u32,
    #[serde(rename = "tenantID")]
    pub tenant_id: u32,
    pub security_question1: String,
    pub security_question2: String,
    pub security_question3: String,
    pub latest_eula_accepted: bool,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SessionData {
    pub session_id: String,
    pub user_info: UserInfo,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    #[serde(rename = "locationID")]
    pub location_id: u64,
    pub name: String,
    pub street_address: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub zipcode: String,
    #[serde(rename = "type")]
    pub location_type: String,
    pub has_station: bool,
    pub devices: Vec<Device>,
    pub one_touch_buttons: Vec<String>,
    pub weather: Weather,
    pub daylight_saving_time_enabled: bool,
    pub time_zone: TimeZone,
    pub one_touch_actions_suspended: bool,
    pub is_location_owner: bool,
    #[serde(rename = "locationOwnerID")]
    pub location_owner_id: u64,
    pub location_owner_name: String,
    pub location_owner_user_name: String,
    pub can_search_for_contractors: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub gateway_id: u64,
    #[serde(rename = "deviceID")]
    pub device_id: u64,
    pub thermostat_model_type: String,
    pub device_type: u64,
    pub name: String,
    pub schedule_capable: bool,
    pub hold_until_capable: bool,
    pub thermostat: Thermostat,
    pub alert_settings: AlertSettings,
    pub is_upgrading: bool,
    pub is_alive: bool,
    pub thermostat_version: String,
    #[serde(rename = "macID")]
    pub mac_id: String,
    #[serde(rename = "locationID")]
    pub location_id: u64,
    #[serde(rename = "domainID")]
    pub domain_id: u64,
    pub instance: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Thermostat {
    pub units: String,
    pub indoor_temperature: f64,
    pub outdoor_temperature: f64,
    pub outdoor_temperature_available: bool,
    pub outdoor_humidity: f64,
    pub outdoot_humidity_available: bool,
    pub indoor_humidity: f64,
    pub indoor_temperature_status: String,
    pub indoor_humidity_status: String,
    pub outdoor_temperature_status: String,
    pub outdoor_humidity_status: String,
    pub is_commercial: bool,
    pub allowed_modes: Vec<String>,
    pub deadband: f64,
    pub min_heat_setpoint: f64,
    pub max_heat_setpoint: f64,
    pub min_cool_setpoint: f64,
    pub max_cool_setpoint: f64,
    pub changeable_values: ChangeableValues,
    pub schedule_capable: bool,
    pub vacation_hold_changeable: bool,
    pub vacation_hold_cancelable: bool,
    pub schedule_heat_sp: f64,
    pub schedule_cool_sp: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChangeableValues {
    pub mode: String,
    pub heat_setpoint: HeatSetpoint,
    pub vacation_hold_days: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HeatSetpoint {
    pub value: f64,
    pub status: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AlertSettings {
    #[serde(rename = "deviceID")]
    pub device_id: u64,
    pub temp_higher_than_active: bool,
    pub temp_higher_than: f64,
    pub temp_higher_than_minutes: u64,
    pub temp_lower_than_active: bool,
    pub temp_lower_than: f64,
    pub temp_lower_than_minutes: u64,
    pub fault_condition_exists_active: bool,
    pub fault_condition_exists_hours: u64,
    pub normal_conditions_active: bool,
    pub communication_lost_active: bool,
    pub communication_lost_hours: u64,
    pub communication_failure_active: bool,
    pub communication_failure_minutes: u64,
    pub device_lost_active: bool,
    pub device_lost_hours: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Weather {
    pub condition: String,
    pub temperature: f64,
    pub units: String,
    pub humidity: u64,
    pub phrase: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeZone {
    pub id: String,
    pub display_name: String,
    pub offset_minutes: u64,
    pub current_offset_minutes: u64,
    pub using_daylight_saving_time: bool,
}

