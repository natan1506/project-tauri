// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent};
use tauri::Manager;
use dotenv::dotenv;
use std::env;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
  dotenv().ok();
  let tray_menu = SystemTrayMenu::new(); 
  let system_tray = SystemTray::new().with_menu(tray_menu);

  tauri::Builder::default()
  .system_tray(system_tray)
      .invoke_handler(tauri::generate_handler![greet])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}

fn my_custom_command() -> String {
  let api_key = env::var("API_KEY").expect("API_KEY não encontrada");
  api_key
}
