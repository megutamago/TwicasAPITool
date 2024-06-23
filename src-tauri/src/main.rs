// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod modules;
use modules::supporters_list::{parse_json, ExtendSupportersData, SupportersData, SupportersList};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_access_token, ladder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn exe_api(
    user_id: &str,
    _token: Option<String>,
    offset: i32,
    limit: i32,
) -> Result<(i32, Vec<SupportersData>), Box<dyn Error>> {
    let supporters_list = SupportersList::new(user_id.to_string(), _token.unwrap_or_default());
    let resp = supporters_list.get_supporters_list(offset, limit)?;
    let json = parse_json(&resp)?;

    let (total, supporters_data) = SupportersList::get_supporters(&json).unwrap_or((0, vec![]));

    Ok((total, supporters_data))
}

fn convert_supporters_data(
    supporters_data: JsonValue,
    offset: i32,
) -> Result<Vec<ExtendSupportersData>, String> {
    let result: Result<Vec<SupportersData>, _> = serde_json::from_value(supporters_data.clone());
    let _supporters_data = match result {
        Ok(data) => data,
        Err(err) => return Err(err.to_string()),
    };
    let extend_supporters_data: Vec<ExtendSupportersData> = _supporters_data
        .into_iter()
        .enumerate()
        .map(|(index, supporters_data)| {
            ExtendSupportersData::new(index as i32 + offset + 1, supporters_data)
        })
        .collect();
    Ok(extend_supporters_data)
}

fn exe_api_loop(
    user_id: &str,
    _token: Option<String>,
    _total: &String,
    _offset: i32,
) -> Result<Vec<ExtendSupportersData>, String> {
    let total: i32 = _total.parse().expect("Faied to conversion");
    let loop_count = (total - _offset) / 20 + 1;
    let mut loop_supporters_data: Vec<ExtendSupportersData> = Vec::new();

    for i in 0..loop_count {
        let offset: i32 = _offset + i * 20;
        let limit: i32 = 20;
        #[allow(unused_variables)]
        let (total, supporters_data) = match exe_api(&user_id, _token.clone(), offset, limit) {
            Ok((total, supporters_data)) => {
                let supporters_data_json = serde_json::to_value(supporters_data)
                    .map_err(|e| format!("Failed to convert supporters data to JSON: {}", e))?;
                (total.to_string(), supporters_data_json)
            }
            Err(_) => return Err("Error occurred while executing APIs".to_string()),
        };

        // extend_supporters_data
        let converted_data = convert_supporters_data(supporters_data, offset)?;
        loop_supporters_data.extend(converted_data);
    }
    Ok(loop_supporters_data)
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn ladder(input: HashMap<String, String>) -> Result<(String, Vec<ExtendSupportersData>), String> {
    let _offset = match input.get("offset") {
        Some(id) => id.clone(),
        None => return Err("offset is not specified".to_string()),
    };

    let offset: i32 = _offset.parse().unwrap_or(1);

    let user_id = match input.get("user_id") {
        Some(id) => id.clone(),
        None => return Err("user_id is not specified".to_string()),
    };

    let token_result = read_file();
    let token = match token_result {
        Ok(contents) => Some(contents),
        Err(err) => {
            eprintln!("Failed to read token: {}", err);
            None // エラーが発生した場合は None を設定する
        }
    };

    // Execute APIs
    #[allow(unused_variables)]
    let (total, supporters_data) = match exe_api(&user_id, token.clone(), 0, 1) {
        Ok((total, supporters_data)) => {
            let supporters_data_json = serde_json::to_value(supporters_data)
                .map_err(|e| format!("Failed to convert supporters data to JSON: {}", e))?;
            (total.to_string(), supporters_data_json)
        }
        Err(_) => return Err("Error occurred while executing APIs".to_string()),
    };

    // API iterations
    let loop_supporters_data = match exe_api_loop(&user_id, token.clone(), &total, offset - 1) {
        Ok(data) => data,
        Err(err) => return Err(format!("Error occurred while executing loop APIs: {}", err)),
    };

    Ok((total, loop_supporters_data))
}

#[tauri::command]
fn save_access_token(token: String) -> Result<(), String> {
    // ファイルを保存する
    let current_dir = fs::canonicalize(".").map_err(|e| e.to_string())?;
    let file_path = current_dir.join("access_token.txt");
    fs::write(&file_path, token).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn read_file() -> Result<String, Box<dyn Error>> {
    // ファイルを読み込む
    let current_dir = fs::canonicalize(".").map_err(|e| e.to_string())?;
    let file_path = current_dir.join("access_token.txt");
    let contents = fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
    Ok(contents)
}
