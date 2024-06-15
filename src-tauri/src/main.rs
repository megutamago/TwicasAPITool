// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod modules;
use modules::supporting_list::{parse_json, SupportingData, SupportingList};
use serde_json::Value as JsonValue;
use std::error::Error;
use std::fs;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_access_token, ladder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn exe_apis(
    user_id: &str,
    _token: Option<String>,
) -> Result<(i32, Vec<SupportingData>), Box<dyn Error>> {
    let supporting_list = SupportingList::new(user_id.to_string(), _token.unwrap_or_default());
    let resp = supporting_list.get_supporting_list()?;
    let json = parse_json(&resp)?;

    let (total, supporting_data) = SupportingList::get_supporting(&json).unwrap_or((0, vec![]));

    Ok((total, supporting_data))
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn ladder(input: String) -> Result<(String, JsonValue), String> {
    let user_id = input;

    let token_result = read_file();
    let token = match token_result {
        Ok(contents) => Some(contents),
        Err(err) => {
            eprintln!("Failed to read token: {}", err);
            None // エラーが発生した場合は None を設定する
        }
    };

    // Execute APIs
    let (total, supporting_data) = match exe_apis(&user_id, token) {
        Ok((total, supporting_data)) => {
            let supporting_data_json = serde_json::to_value(supporting_data)
                .map_err(|e| format!("Failed to convert supporting data to JSON: {}", e))?;
            (total.to_string(), supporting_data_json)
        }
        Err(_) => return Err("Error occurred while executing APIs".to_string()),
    };

    Ok((total, supporting_data))
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
