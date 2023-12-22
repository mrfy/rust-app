// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::process::Command;

use mongodb::{bson::Document, options::ClientOptions, Client};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn list_databases(client: tauri::State<'_, Client>) -> Result<Vec<String>, ()> {
    let mut results = Vec::new();

    let cmd = format!("mongorestore -v --uri 'mongodb://root:dev@localhost:27017/?directConnection=true' --drop --archive=bakap --nsFrom=\"platform.*\" --nsTo=\"platform_quality_ori.*\" --gzip");

    println!("{}", cmd);

    let output = Command::new("mongorestore").output();

    match output {
        Ok(res) => {
            println!("{:?}", res);
        }
        Err(err) => {
            println!("{}", err);
            // panic!(err);
        }
    }
    for db_name in client.list_database_names(None, None).await.unwrap() {
        println!("{}", db_name);
        results.push(db_name)
    }

    Ok(results)
}

fn main() {
    let db_url = "mongodb://root:dev@localhost:27017/?directConnection=true";

    let options = ClientOptions::parse(db_url).expect("invalid database url");

    let client = Client::with_options(options).unwrap();

    tauri::Builder::default()
        .manage(client)
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![list_databases])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
