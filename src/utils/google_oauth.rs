use crate::{error::Error, AppState};
use reqwest::{Client, Url};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct QueryCode {
    pub code: String,
    pub state: String,
}
#[derive(Deserialize)]
pub struct OAuthResponse {
    pub access_token: String,
    pub id_token: String,
}

#[derive(Deserialize)]
pub struct GoogleUserResult {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub locale: String,
}

pub async fn request_token(
    authorization_code: &str,
    data: &Arc<AppState>,
) -> Result<OAuthResponse, Error> {
    let redirect_url = data.env.google_oauth_redirect_url.to_owned();
    let client_secret = data.env.google_oauth_client_secret.to_owned();
    let client_id = data.env.google_oauth_client_id.to_owned();

    let root_url = "https://oauth2.googleapis.com/token";
    let client = Client::new();

    let params = [
        ("grant_type", "authorization_code"),
        ("redirect_uri", redirect_url.as_str()),
        ("client_id", client_id.as_str()),
        ("code", authorization_code),
        ("client_secret", client_secret.as_str()),
    ];
    let response = client.post(root_url).form(&params).send().await.unwrap();

    
    if response.status().is_success() {
        let oauth_response = response.json::<OAuthResponse>().await.unwrap();
        Ok(oauth_response)
    } else {
        let error_msg = response.text().await.expect("Error while parsing error message");
        Err(Error::RetrieveTokenError(error_msg))
        
    }
}

pub async fn get_google_user(
    access_token: &str,
    id_token: &str,
) -> Result<GoogleUserResult, Error> {
    let client = Client::new();
    let mut url = Url::parse("https://www.googleapis.com/oauth2/v1/userinfo").unwrap();
    url.query_pairs_mut().append_pair("alt", "json");
    url.query_pairs_mut()
        .append_pair("access_token", access_token);

    let response = client.get(url).bearer_auth(id_token).send().await.unwrap();

    if response.status().is_success() {
        let user_info = response.json::<GoogleUserResult>().await.unwrap();
        Ok(user_info)
    } else {
        Err(Error::RetriveUserError)
    }
}
