// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use mongodb::{bson::Document, options::ClientOptions, Client};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// #[tauri::command]
// async fn list_databases(
//     client: tauri::State<'_, Client>,
//     collection: String,
//     filter: bson::Document,
// ) -> Result<String, Box<dyn Error>> {
//     let mut client_options = ClientOptions::parse(
//         "mongodb://root:dev@localhost:27017/?authMechanism=DEFAULT&directConnection=true",
//     );

//     let client = Client::with_options(client_options);

//     // List the names of the databases in that deployment.
//     for db_name in client?.list_database_names(None, None).await? {
//         println!("{}", db_name);
//     }

//     Ok("OK".to_string())
// }

#[tauri::command]
async fn list_databases(client: tauri::State<'_, Client>) -> Result<Vec<Document>, ()> {
    // let db: mongodb::Database = client.default_database().unwrap();

    for db_name in client.list_database_names(None, None).await.unwrap() {
        println!("{}", db_name);
    }

    let results = Vec::new();
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
