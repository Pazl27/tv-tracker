use logic;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn hello_world() -> String {
    logic::hello_world()
}

#[tauri::command]
fn api() -> String {
    "test".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![hello_world, greet, api])
        .setup(|_| {
            let message = logic::hello_world();
            println!("{}", message);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
