use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

#[derive(Serialize, Debug, Deserialize)]
pub struct SupportersData {
    pub id: String,
    pub screen_id: String,
    pub name: String,
    pub image: String,
    pub profile: String,
    pub level: i32,
    pub last_movie_id: Option<String>,
    pub is_live: bool,
    pub supported: i32,
    pub supporter_count: i32,
    pub supporting_count: i32,
    pub point: i32,
    pub total_point: i32,
    pub created: i32,
}

#[derive(Serialize)]
pub struct ExtendSupportersData {
    pub _id: i32,
    #[serde(flatten)]
    pub supporters_data: SupportersData,
}

impl ExtendSupportersData {
    pub fn new(_id: i32, supporters_data: SupportersData) -> Self {
        Self {
            _id,
            supporters_data,
        }
    }
}

#[derive(Serialize)]
pub struct ExtendSupportersListData {
    pub total: i32,
    #[serde(flatten)]
    pub extend_supporters_data: ExtendSupportersData,
}

pub struct SupportersList {
    user_id: String,
    token: String,
}

pub fn parse_json(text: &str) -> Result<Value, Box<dyn Error>> {
    let json: Value = serde_json::from_str(text)?;
    Ok(json)
}

impl SupportersList {
    pub fn new(user_id: String, token: String) -> Self {
        Self { user_id, token }
    }

    pub fn get_supporters_list(
        &self,
        offset: i32,
        limit: i32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let offset = offset;
        let limit = limit;
        let url = format!(
            "https://apiv2.twitcasting.tv/users/{}/supporters?offset={}&limit={}&sort=ranking",
            self.user_id, offset, limit
        );

        let client = Client::new();
        let resp = client
            .get(&url)
            .header("Accept", "application/json")
            .header("X-Api-Version", "2.0")
            .header("Authorization", &format!("Bearer {}", self.token))
            .send()?;

        println!("{:?}", resp);

        let text = resp.text()?;
        Ok(text)
    }

    pub fn get_supporters(json: &Value) -> Result<(i32, Vec<SupportersData>), Box<dyn Error>> {
        let total = json["total"].as_i64().unwrap_or(0) as i32;
        let supporters: Vec<SupportersData> = {
            if let Some(data) = json["supporters"].as_array() {
                data.iter()
                    .filter_map(|item| serde_json::from_value::<SupportersData>(item.clone()).ok())
                    .collect()
            } else {
                Vec::new()
            }
        };
        Ok((total, supporters))
    }
}
