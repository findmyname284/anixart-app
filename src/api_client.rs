use std::env;

use once_cell::sync::Lazy;
use reqwest::{Client, Proxy};
use serde_json::{Value, json};
use urlencoding::encode;

use crate::utils::{config::Config, filter};

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

    let http_proxy = env::var("http_proxy").ok();

    if let Some(proxy) = config.network.proxy {
        if !proxy.is_empty() {
            if let Ok(proxy) = Proxy::all(&proxy) {
                client_builder = client_builder.proxy(proxy);
            }
        } else {
            if let Some(proxy) = http_proxy {
                if let Ok(proxy) = Proxy::all(&proxy) {
                    client_builder = client_builder.proxy(proxy);
                }
            }
        }
    }

    let client = client_builder
        .build()
        .expect("Failed to create HTTP client");

    AnixartClient {
        client,
        base_url: String::from("https://api.anixart.tv"),
        token: config.auth.token.unwrap_or(String::new()),
    }
});

pub struct AnixartClient {
    pub client: reqwest::Client,
    pub base_url: String,
    token: String,
}

impl AnixartClient {
    pub fn global() -> &'static Self {
        &HTTP_CLIENT
    }

    pub async fn sign_up(
        &self,
        login: &str,
        email: &str,
        password: &str,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let body = format!(
            "login={}&email={}&password={}",
            encode(login),
            encode(email),
            encode(password)
        );

        let response = self
            .client
            .post(format!("{}/auth/signUp", self.base_url))
            .header("Host", "api.anixart.tv")
            .body(body)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .send()
            .await?;

        response.error_for_status_ref()?;

        let auth_response = response.json::<Value>().await?;
        Ok(auth_response)
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

    pub async fn filter(
        &self,
        filter: filter::FilterRequest,
        index: u32,
    ) -> Result<Value, reqwest::Error> {
        let url = format!(
            "{}/filter/{}?extended_mode=true&token={}",
            self.base_url, index, self.token
        );

        let response = self
            .client
            .post(&url)
            .header("Host", "api.anixart.tv")
            .json(&filter)
            .send()
            .await?;

        response.error_for_status_ref()?;
        let response_data = response.json::<Value>().await?;
        Ok(response_data)
    }

    pub async fn favorite(&self, index: u32) -> Result<Value, reqwest::Error> {
        let response = self
            .client
            .get(format!(
                "{}/favorite/all/{}?sort=1&filter_announce=0&token={}",
                self.base_url, index, self.token
            ))
            .send()
            .await?;

        response.error_for_status_ref()?;

        let favorite_response = response.json::<Value>().await?;
        Ok(favorite_response)
    }

    // https://api-alt.anixart.app/profile/list/all/1/0?sort=1&filter_announce=0&token=c9b442779655b81f2004c96f69d9943e065e338c # смотрю
    // https://api-alt.anixart.app/profile/list/all/2/0?sort=1&filter_announce=0&token=c9b442779655b81f2004c96f69d9943e065e338c # в планах

    pub async fn profile_list(&self, status_id: u32, index: u32) -> Result<Value, reqwest::Error> {
        let response = self
            .client
            .get(format!(
                "{}/profile/list/all/{}/{}?sort=1&filter_announce=0&token={}",
                self.base_url, status_id, index, self.token
            ))
            .send()
            .await?;

        response.error_for_status_ref()?;

        let profile_response = response.json::<Value>().await?;
        Ok(profile_response)
    }

    pub async fn history(&self, index: u32) -> Result<Value, reqwest::Error> {
        let response = self
            .client
            .get(format!(
                "{}/history/{}?token={}",
                self.base_url, index, self.token
            ))
            .send()
            .await?;

        response.error_for_status_ref()?;

        let history_response = response.json::<Value>().await?;
        Ok(history_response)
    }

    // https://api-alt.anixart.app/collectionFavorite/all/0?token=c9b442779655b81f2004c96f69d9943e065e338c
    pub async fn collection_favorite(&self, index: u32) -> Result<Value, reqwest::Error> {
        let response = self
            .client
            .get(format!(
                "{}/collectionFavorite/all/{}?token={}",
                self.base_url, index, self.token
            ))
            .send()
            .await?;

        response.error_for_status_ref()?;

        let collection_response = response.json::<Value>().await?;
        Ok(collection_response)
    }

    // https://api-alt.anixart.app/profile/1816556?token=c9b442779655b81f2004c96f69d9943e065e338c
    pub async fn profile(&self, id: u32) -> Result<Value, reqwest::Error> {
        let response = self
            .client
            .get(format!(
                "{}/profile/{}?token={}",
                self.base_url, id, self.token
            ))
            .send()
            .await?;

        response.error_for_status_ref()?;

        let profile_response = response.json::<Value>().await?;
        Ok(profile_response)
    }

    // https://api-alt.anixart.app/profile/preference/my?token=c9b442779655b81f2004c96f69d9943e065e338c
    pub async fn profile_preference(&self) -> Result<Value, reqwest::Error> {
        let response = self
            .client
            .get(format!(
                "{}/profile/preference/my?token={}",
                self.base_url, self.token
            ))
            .send()
            .await?;

        response.error_for_status_ref()?;

        let preference_response = response.json::<Value>().await?;
        Ok(preference_response)
    }

    // https://api-alt.anixart.app/search/releases/0?token=c9b442779655b81f2004c96f69d9943e065e338c
    pub async fn search_releases(&self, index: u32, query: &str) -> Result<Value, reqwest::Error> {
        let response = self
            .client
            .post(format!(
                "{}/search/releases/{}?token={}",
                self.base_url, index, self.token
            ))
            .json(&json!({ "query": query, "searchBy": 0 }))
            .send()
            .await?;

        response.error_for_status_ref()?;

        let search_response = response.json::<Value>().await?;
        Ok(search_response)
    }

    // https://api-alt.anixart.app/search/profile/list/1/0?token=c9b442779655b81f2004c96f69d9943e065e338c
    pub async fn search_profile_list(
        &self,
        status_id: u32,
        index: u32,
        query: &str,
    ) -> Result<Value, reqwest::Error> {
        let response = self
            .client
            .post(format!(
                "{}/search/profile/list/{}/{}?token={}",
                self.base_url, status_id, index, self.token
            ))
            .json(&json!({ "query": query, "searchBy": 0 }))
            .send()
            .await?;

        response.error_for_status_ref()?;

        let search_response = response.json::<Value>().await?;
        Ok(search_response)
    }

    // https://api-alt.anixart.app/profile/login/history/all/1816556/0?token=c9b442779655b81f2004c96f69d9943e065e338c
    pub async fn profile_login_history(
        &self,
        profile_id: u32,
        index: u32,
    ) -> Result<Value, reqwest::Error> {
        let response = self
            .client
            .get(format!(
                "{}/profile/login/history/all/{}/{}?token={}",
                self.base_url, profile_id, index, self.token
            ))
            .send()
            .await?;

        response.error_for_status_ref()?;

        let history_response = response.json::<Value>().await?;
        Ok(history_response)
    }

    // https://api-alt.anixart.app/release/18909?extended_mode=true&token=c9b442779655b81f2004c96f69d9943e065e338c
    pub async fn release(&self, id: u32) -> Result<Value, reqwest::Error> {
        let response = self
            .client
            .get(format!(
                "{}/release/{}?extended_mode=true&token={}",
                self.base_url, id, self.token
            ))
            .send()
            .await?;

        response.error_for_status_ref()?;

        let release_response = response.json::<Value>().await?;
        Ok(release_response)
    }

    // https://api-alt.anixart.app/release/streaming/platform/18909
    pub async fn release_streaming_platform(&self, id: u32) -> Result<Value, reqwest::Error> {
        let response = self
            .client
            .get(format!(
                "{}/release/streaming/platform/{}?token={}",
                self.base_url, id, self.token
            ))
            .send()
            .await?;

        response.error_for_status_ref()?;

        let platform_response = response.json::<Value>().await?;
        Ok(platform_response)
    }

    // https://api-alt.anixart.app/episode/18909?token=c9b442779655b81f2004c96f69d9943e065e338c
    pub async fn episode(&self, id: u32) -> Result<Value, reqwest::Error> {
        let response = self
            .client
            .get(format!(
                "{}/episode/{}?token={}",
                self.base_url, id, self.token
            ))
            .send()
            .await?;

        response.error_for_status_ref()?;

        let episode_response = response.json::<Value>().await?;
        Ok(episode_response)
    }

    // https://api-alt.anixart.app/episode/18909/29
    pub async fn episode_sources_by_type(
        &self,
        release_id: u32,
        type_id: u32,
    ) -> Result<Value, reqwest::Error> {
        let response = self
            .client
            .get(format!(
                "{}/episode/{}/{}",
                self.base_url, release_id, type_id
            ))
            .send()
            .await?;

        response.error_for_status_ref()?;

        let episode_response = response.json::<Value>().await?;
        Ok(episode_response)
    }

    // https://api-alt.anixart.app/episode/18909/29/30?sort=1&token=
    pub async fn episode_sources(
        &self,
        release_id: u32,
        type_id: u32,
        index: u32,
    ) -> Result<Value, reqwest::Error> {
        let response = self
            .client
            .get(format!(
                "{}/episode/{}/{}/{}?sort=1&token={}",
                self.base_url, release_id, type_id, index, self.token
            ))
            .send()
            .await?;

        response.error_for_status_ref()?;

        let episode_response = response.json::<Value>().await?;
        Ok(episode_response)
    }

    // https://api-alt.anixart.app/episode/target/18909/30/3
    pub async fn episode_target(
        &self,
        release_id: u32,
        type_id: u32,
        episode: u32,
    ) -> Result<Value, reqwest::Error> {
        let response = self
            .client
            .get(format!(
                "{}/episode/target/{}/{}/{}",
                self.base_url, release_id, type_id, episode
            ))
            .send()
            .await?;

        response.error_for_status_ref()?;

        let episode_response = response.json::<Value>().await?;
        Ok(episode_response)
    }

    // https://kodik.biz/api/video-links?p=56a768d08f43091901c44b54fe970049&link=//kodik.info/seria/1447065/00e6639fa57b1f86b8c3bad55de978ed/720p&d=2025062113&s=18a85af08d061f0bc14b2d12cc6e25ce638ca8f09c42ed7b7a3be10f2898f85f&ip=162.158.163.73
    // https://kodik.biz/api/video-links?p=56a768d08f43091901c44b54fe970049&link=//kodik.info/seria/1450956/d266751a41f01c560b2803282ab87d2d/720p&d=2025062300&s=uDFHE21MHLqQvmd4U6LX019JW3tBjir4hkRp6qOCkegQh3rh7RWqewOfauYZkNjL&ip=108.162.226.110
    pub async fn kodik_video_links(&self, link: &str) -> Result<Value, reqwest::Error> {
        let token = "56a768d08f43091901c44b54fe970049";
        let url = format!(
            "https://kodik.biz/api/video-links?p={}&link={}",
            token, link
        );

        let response = self.client.get(&url).send().await?;

        response.error_for_status_ref()?;

        let video_links_response = response.json::<Value>().await?;
        Ok(video_links_response)
    }

    pub async fn get_ip(&self) -> Result<Value, reqwest::Error> {
        let response = self.client.get("https://httpbin.org/ip").send().await?;

        response.error_for_status_ref()?;

        let ip_response = response.json::<Value>().await?;
        Ok(ip_response)
    }
}
