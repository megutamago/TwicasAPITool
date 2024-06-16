// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod modules;
use modules::supporting_list::{parse_json, ExtendSupportingData, SupportingData, SupportingList};
use serde_json::Value as JsonValue;
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
) -> Result<(i32, Vec<SupportingData>), Box<dyn Error>> {
    let supporting_list = SupportingList::new(user_id.to_string(), _token.unwrap_or_default());
    let resp = supporting_list.get_supporting_list(offset, limit)?;
    let json = parse_json(&resp)?;

    let (total, supporting_data) = SupportingList::get_supporting(&json).unwrap_or((0, vec![]));

    Ok((total, supporting_data))
}

fn convert_supporting_data(
    supporting_data: JsonValue,
    offset: i32,
) -> Result<Vec<ExtendSupportingData>, String> {
    let result: Result<Vec<SupportingData>, _> = serde_json::from_value(supporting_data.clone());
    let _supporting_data = match result {
        Ok(data) => data,
        Err(err) => return Err(err.to_string()),
    };
    let extend_supporting_data: Vec<ExtendSupportingData> = _supporting_data
        .into_iter()
        .enumerate()
        .map(|(index, supporting_data)| {
            ExtendSupportingData::new(index as i32 + offset + 1, supporting_data)
        })
        .collect();
    Ok(extend_supporting_data)
}

fn exe_api_loop(
    user_id: &str,
    _token: Option<String>,
    _total: &String,
) -> Result<Vec<ExtendSupportingData>, String> {
    let num: i32 = _total.parse().expect("Faied to conversion");
    let loop_count = num / 20 + 1;
    let mut tmp_data: Vec<ExtendSupportingData> = Vec::new();

    for i in 0..loop_count {
        let offset: i32 = i * 20;
        let limit: i32 = 20;
        #[allow(unused_variables)]
        let (total, supporting_data) = match exe_api(&user_id, _token.clone(), offset, limit) {
            Ok((total, supporting_data)) => {
                let supporting_data_json = serde_json::to_value(supporting_data)
                    .map_err(|e| format!("Failed to convert supporting data to JSON: {}", e))?;
                (total.to_string(), supporting_data_json)
            }
            Err(_) => return Err("Error occurred while executing APIs".to_string()),
        };

        // extend_supporting_data
        let converted_data = convert_supporting_data(supporting_data, offset)?;
        tmp_data.extend(converted_data);
    }
    Ok(tmp_data)
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn ladder(input: String) -> Result<(String, Vec<ExtendSupportingData>), String> {
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
    #[allow(unused_variables)]
    let (total, supporting_data) = match exe_api(&user_id, token.clone(), 0, 1) {
        Ok((total, supporting_data)) => {
            let supporting_data_json = serde_json::to_value(supporting_data)
                .map_err(|e| format!("Failed to convert supporting data to JSON: {}", e))?;
            (total.to_string(), supporting_data_json)
        }
        Err(_) => return Err("Error occurred while executing APIs".to_string()),
    };

    // API iterations
    let loop_supporting_data = match exe_api_loop(&user_id, token.clone(), &total) {
        Ok(data) => data,
        Err(err) => return Err(format!("Error occurred while executing loop APIs: {}", err)),
    };

    Ok((total, loop_supporting_data))
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
