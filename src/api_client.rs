use once_cell::sync::Lazy;
use reqwest::{Client, Proxy};
use serde::Serialize;
use serde_json::Value;
use urlencoding::encode;

use crate::utils::config::Config;

const USER_AGENT: &str = "AnixartApp/9.0 BETA 3-25021818 (Android 11; SDK 30; x86_64; Waydroid WayDroid x86_64 Device; en)";

static HTTP_CLIENT: Lazy<AnixartClient> = Lazy::new(|| {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Connection",
        reqwest::header::HeaderValue::from_static("keep-alive"),
    );

    let config = Config::load();
    let mut client_builder = Client::builder()
        .default_headers(headers)
        .user_agent(USER_AGENT)
        .danger_accept_invalid_certs(true)
        .timeout(std::time::Duration::from_secs(15));

    if let Some(proxy) = config.proxy {
        if !proxy.is_empty() {
            if let Ok(proxy) = Proxy::all(&proxy) {
                client_builder = client_builder.proxy(proxy);
            }
        }
    }

    let client = client_builder
        .build()
        .expect("Failed to create HTTP client");

    AnixartClient { client }
});

#[derive(Serialize)]
struct FilterRequest {
    country: Option<()>,
    season: Option<()>,
    sort: i32,
    studio: Option<()>,
    #[serde(rename = "age_ratings")]
    age_ratings: Vec<()>,
    category_id: Option<()>,
    end_year: Option<()>,
    episode_duration_from: Option<()>,
    episode_duration_to: Option<()>,
    episodes_from: Option<()>,
    episodes_to: Option<()>,
    genres: Vec<()>,
    is_genres_exclude_mode_enabled: bool,
    profile_list_exclusions: Vec<()>,
    start_year: Option<()>,
    status_id: Option<u8>,
    types: Vec<()>,
}

pub struct AnixartClient {
    pub client: reqwest::Client,
}

impl AnixartClient {
    pub fn global() -> &'static Self {
        &HTTP_CLIENT
    }

    pub async fn sign_in(
        &self,
        login: &str,
        password: &str,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let body = format!("login={}&password={}", encode(login), encode(password));

        let response = self
            .client
            .post("https://api.anixart.tv/auth/signIn")
            .header("Host", "api.anixart.tv")
            .body(body)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .send()
            .await?;

        response.error_for_status_ref()?;

        let auth_response = response.json::<Value>().await?;
        Ok(auth_response)
    }

    pub async fn apply_filter(&self, token: &str) -> Result<Value, reqwest::Error> {
        let body = FilterRequest {
            country: None,
            season: None,
            sort: 0,
            studio: None,
            age_ratings: vec![],
            category_id: None,
            end_year: None,
            episode_duration_from: None,
            episode_duration_to: None,
            episodes_from: None,
            episodes_to: None,
            genres: vec![],
            is_genres_exclude_mode_enabled: false,
            profile_list_exclusions: vec![],
            start_year: None,
            status_id: None,
            types: vec![],
        };

        let url = format!(
            "https://api.anixart.tv/filter/0?extended_mode=true&token={}",
            token
        );

        let response = self
            .client
            .post(&url)
            .header("Host", "api.anixart.tv")
            .json(&body)
            .send()
            .await?;

        response.error_for_status_ref()?;
        let response_data = response.json::<Value>().await?;
        Ok(response_data)
    }

    pub async fn get_ip(&self) -> Result<Value, reqwest::Error> {
        let response = self.client.get("https://httpbin.org/ip").send().await?;

        response.error_for_status_ref()?;

        let ip_response = response.json::<Value>().await?;
        Ok(ip_response)
    }
}
