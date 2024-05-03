
pub mod api_errors;

use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;
use wasm_bindgen::JsValue;


use self::api_errors::ApiError;

const BASE_URL: &str = include_str!("api_base_uri.txt");



#[derive(Serialize, Deserialize,Debug)]
pub struct User {
    pub id_token:String,
    pub access_token: String,
    pub refesh_token:String
}

#[derive(Serialize, Deserialize)]
pub struct GetResponse {
    pub id: i32,
    pub user_name: String,
    pub location :String,
    pub data :String,
    pub name:String,
}

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
pub struct Task1 {
    pub id: i32,
    pub user_name: String,
    pub location :String,
    pub data :String,
    pub name:String,
}





#[derive(Serialize, Deserialize)]
pub struct SignupResponse {
    pub status:String,
    pub message:String,
}



#[derive(Debug,Serialize, Deserialize)]
pub struct SigninResponse {
    pub id_token:String,
    pub access_token:String,
    pub refresh_token: Option<String>, // Optional in case it's missing
}


pub async fn create_account(username: String, password: String, email: String) -> Result<SignupResponse, JsValue> {
    let response = Request::post(&format!("{}signup", BASE_URL))
        .header("Content-Type", "application/json")
        .body(
            json!({
                "username": username,
                "password": password,
                "email": email
            })
            .to_string(),
        )
        .send()
        .await;

    match response {
        Ok(http_response) => {

            if http_response.ok() {
                let auth_response = http_response.json::<SignupResponse>().await;
                match auth_response {
                    Ok(data) => Ok(data), 
                    Err(_) => Err(JsValue::from_str("Error parsing response JSON")),
                }
            } else {
                Err(JsValue::from_str(&format!("HTTP Error: {}", http_response.status())))
            }
        },
        Err(_) => Err(JsValue::from_str("Network request failed")),
    }
}






pub async fn login(username: String, password: String) -> Result<SigninResponse, JsValue> {
    let response = gloo_net::http::Request::post(&format!("{}signin", BASE_URL))
        .header("Content-Type", "application/json")
        .body(json!({
            "username": username,
            "password": password,
        }).to_string()).unwrap()
        .send()
        .await;

        match response {
            Ok(http_response) => {
                if http_response.ok() {
                    // Only read the response body once
                    let json_result = http_response.json::<SigninResponse>().await;
    
                    match json_result {
                        Ok(data) => Ok(data),
                        Err(e) => Err(JsValue::from_str(&format!("Error parsing JSON: {:?}", e))),
                    }
                } else {
                    Err(JsValue::from_str(&format!("HTTP Error: {}", http_response.status())))
                }
            },
            Err(e) => {
                Err(JsValue::from_str(&format!("Network request failed: {:?}", e)))
            },
        }
    }



// use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Note {
    pub data: String,
    pub id: u32,
    pub location: String,
    pub name: String,
    pub user_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTasksResponse {
    pub notes: Vec<Note>,
    pub results: u32,
    pub status: String,
}


pub async fn get_tasks(token: &str) -> Result<GetTasksResponse, ApiError> {
    let client = reqwest::Client::new();
    let response = client.get(&format!("{}get/user", BASE_URL))
        .header("AUTHORIZATION", token)
        .send()
        .await.expect("failed");

    if response.status().is_success() {
        let result = response.json::<GetTasksResponse>().await.unwrap();
        Ok(result)
    } else {
        Err(handle_errors(response.status().into()))
    }
}



pub async fn logout(token: &str) -> Result<(), ApiError> {
    let request = Request::post(&format!("{}signout", BASE_URL))
        .header("AUTHORIZATION", token)
        .send()
        .await
        .unwrap();
    if request.ok() {
        Ok(())
    } else {
        Err(handle_errors(request.status()))
    }
}

fn handle_errors(status: u16) -> ApiError {
    match status {
        401 => ApiError::NotAuthenticated,
        _ => ApiError::Unknown,
    }
}
