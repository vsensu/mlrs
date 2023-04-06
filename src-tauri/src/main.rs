// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::vec;

use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(serde::Serialize)]
struct GraphData {
    x: Vec<f32>,
    y: Vec<f32>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    let split: Vec<&str> = name.trim().split(',').collect();
    let x = split[0];
    let x: f32 = match x.parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };
    let y = split[1];
    let y: f32 = match y.parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };
    let data_set = vec![
        vec![1.0, 1.1],
        vec![1.0, 1.0],
        vec![0.0, 0.0],
        vec![0.0, 0.1],
    ];
    let labels = vec![1, 1, 2, 2];
    let in_x = vec![x, y];
    let k = 3;
    let result = mlrs::classify0(&in_x, &data_set, &labels, k);
    format!(
        "The input point {:?} is classified as label {}",
        in_x, result
    )
}

#[tauri::command]
fn plot() -> GraphData {
    GraphData {
        x: vec![1.0, 2.0, 3.0],
        y: vec![1.0, 2.0, 3.0],
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, plot])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
