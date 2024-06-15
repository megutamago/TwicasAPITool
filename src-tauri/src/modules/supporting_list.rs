use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

#[derive(Serialize, Debug, Deserialize)]
pub struct SupportingData {
    pub screen_id: String,
    pub name: String,
    pub point: i32,
    pub total_point: i32,
}

pub struct SupportingList {
    user_id: String,
    token: String,
}

pub fn parse_json(text: &str) -> Result<Value, Box<dyn Error>> {
    let json: Value = serde_json::from_str(text)?;
    Ok(json)
}

impl SupportingList {
    pub fn new(user_id: String, token: String) -> Self {
        Self { user_id, token }
    }

    pub fn get_supporting_list(&self) -> Result<String, Box<dyn std::error::Error>> {
        let offset = 10;
        let limit = 20;
        let url = format!(
            "https://apiv2.twitcasting.tv/users/{}/supporting?offset={}&limit={}",
            self.user_id, offset, limit
        );

        let client = Client::new();
        let resp = client
            .get(&url)
            .header("Accept", "application/json")
            .header("X-Api-Version", "2.0")
            .header("Authorization", &format!("Bearer {}", self.token))
            .send()?;

        let text = resp.text()?;
        Ok(text)
    }

    pub fn get_supporting(json: &Value) -> Result<(i32, Vec<SupportingData>), Box<dyn Error>> {
        let total = json["total"].as_i64().unwrap_or(0) as i32;
        let supporting: Vec<SupportingData> = {
            if let Some(data) = json["supporting"].as_array() {
                data.iter()
                    .filter_map(|item| serde_json::from_value::<SupportingData>(item.clone()).ok())
                    .collect()
            } else {
                Vec::new()
            }
        };
        Ok((total, supporting))
    }
}