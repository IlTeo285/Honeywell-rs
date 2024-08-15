
mod api_types;
pub use api_types::{SessionData, UserInfo, Location, Device};

use thiserror::Error;
use reqwest::{
    header::{ACCEPT, CONTENT_LENGTH, CONTENT_TYPE},
    Client, StatusCode,
};

use serde::Serialize;
use std::{fmt, env};

use log::{error, info};

const BASE_SERVER_URL: &str = "https://mytotalconnectcomfort.com/WebApi/";

macro_rules! url {
    ($page:expr) => { 
            format!("{}{}", BASE_SERVER_URL, $page)
    };
}

#[derive(Error, Debug)]
pub enum TotalComfortError {
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("Renew failed: {0}")]
    RenewFailed(String),

    #[error("Data retrieve failed: {0}")]
    DataRetrive(String)
}

type Result<T> = std::result::Result<T, TotalComfortError>;


#[derive(Serialize)]
 pub struct Authentication{
    username: String,
    password: String,
    #[serde(rename = "applicationId")]
    application_id: String
}

impl fmt::Debug for Authentication {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("User")
            .field("username", &self.username)
            .field("application_id", &self.application_id)
            .finish()
    }
}

impl Authentication {
    pub fn new(username: &str, password: &str) -> Result<Self> {
        let application_id = env::var("HONEYWELL_APPLICATION_ID")
            .map_err(|err|{
                TotalComfortError::AuthenticationFailed(err.to_string())
            })?;

        Ok(Self {
            username: username.to_owned(), 
            password: password.to_owned(),
            application_id
        })
    }

    pub fn from_env() -> Result<Self> {
        let application_id = env::var("HONEYWELL_APPLICATION_ID")
            .map_err(|err|{
                TotalComfortError::AuthenticationFailed(err.to_string())
            })?;
        
        let username = env::var("HONEYWELL_USERNAME")
            .map_err(|err|{
                TotalComfortError::AuthenticationFailed(err.to_string())
            })?;

        let password = env::var("HONEYWELL_PASSWORD")
            .map_err(|err|{
                TotalComfortError::AuthenticationFailed(err.to_string())
            })?;

        Ok(Self {
            username, 
            password,
            application_id
        })
    }
}

#[derive(Debug, Default)]
pub struct TotalComfort(Option<SessionData>, Option<Authentication>);

impl TotalComfort {
    pub fn new() -> Self {
        TotalComfort::default()
    }

    pub async fn renew(&mut self) -> Result<()> {
        
        let id = self.0.as_ref().ok_or(TotalComfortError::RenewFailed("must be authenticate first".into()))?;

        let client = Client::new();
        let response = client
            .put(url!("api/session"))
            .header("sessionId", id.session_id.clone())
            .header(ACCEPT, "application/json")
            .header(CONTENT_LENGTH, 0)
            .send()
            .await
            .map_err(|err| {
                TotalComfortError::RenewFailed(err.to_string())
            })?;
        
        let status = response.status();
        if status.is_success() {
            Ok(())
        }
        else {
            //try with regular auth from 
            if status == StatusCode::UNAUTHORIZED {
                info!("unable to renew, try a full authentication");
                return self.authenticate(Authentication::from_env()?).await;
            }
            Err(TotalComfortError::RenewFailed(status.to_string()))
        }
    }

    pub async fn authenticate(&mut self, auth: Authentication) -> Result<()> {
        let client = Client::new();

        let response = client
            .post(url!("api/Session"))
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json")
            .json(&auth)
            .send()
            .await
            .map_err(|err| {
                TotalComfortError::AuthenticationFailed(err.to_string())
            })?;
        
        let session_data: SessionData = if response.status().is_success() {
            response.json().await            
                .map_err(|err| {
                TotalComfortError::AuthenticationFailed(err.to_string())
            })
        } else {
            Err(TotalComfortError::AuthenticationFailed(format!("Auth failed: {}", response.status())))
        }?;

        self.0 = Some(session_data);
        self.1 = Some(auth);

        Ok(())
    }

    pub fn authenticate_with_session(&mut self, sesssion: SessionData) {
        self.0 = Some(sesssion);
    }

    // https://mytotalconnectcomfort.com/WebApi/Help/Api/GET-api-locations_userId_allData_include
    pub async fn get_locations(&self) -> Result<Vec<Location>> {

        let session = self.0.as_ref().ok_or(TotalComfortError::DataRetrive("must be authenticate first".into()))?;

        let client = Client::new();

        
        let rb = client
            .get(url!("api/locations"))
            .header(ACCEPT, "application/json")
            .header("sessionId", session.session_id.clone())
            .query(&[
                ("userId", session.user_info.user_id.to_string()),
                ("allData", "true".to_owned()),
            ]);
        
        let response = rb.send()
            .await
            .map_err(|err| {
                TotalComfortError::DataRetrive(err.to_string())
            })?;
        
        let status = response.status();

        let data: Vec<Location> = if status.is_success() {           
            response.json::<Vec<Location>>().await            
                .map_err(|err| {
                TotalComfortError::DataRetrive(err.to_string())
            })
        } else if status == StatusCode::UNAUTHORIZED {
            Err(TotalComfortError::AuthenticationFailed(format!("Retrieve failed: {}", response.status())))
        } 
        else {
            Err(TotalComfortError::DataRetrive(format!("Retrieve failed: {}", response.status())))
        }?;

        Ok(data)
    }
}
