pub mod authorization;
pub mod credentials;
pub mod foundation;
pub mod private_data;
pub mod project_access;

use std::sync::Mutex;

use project_access::ProjectAccess;
use tauri::State;

#[tauri::command]
fn create_project_session(
    directory: String,
    access: State<'_, Mutex<ProjectAccess>>,
) -> Result<String, String> {
    access
        .lock()
        .map_err(|_| "project access is unavailable".to_string())?
        .select_directory(std::path::Path::new(&directory))
        .map_err(|_| "directory could not be authorized".to_string())
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(Mutex::new(ProjectAccess::default()))
        .invoke_handler(tauri::generate_handler![create_project_session])
        .run(tauri::generate_context!())
        .expect("failed to run ProjectMender");
}
