use crate::nssm::*;
use crate::service::*;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct FrontendServiceInfo {
    pub name: String,
    pub display_name: String,
    pub status: String,
    pub is_nssm: bool,
    pub image_path: Option<String>,
}

#[tauri::command]
pub async fn install_service_cmd(
    service_name: String,
    exe_path: String,
    args: Option<String>,
) -> Result<(), String> {
    install_service(&service_name, &exe_path, args.as_deref())
}

#[tauri::command]
pub async fn remove_service_cmd(service_name: String) -> Result<(), String> {
    remove_service(&service_name)
}

#[tauri::command]
pub async fn start_service_cmd(service_name: String) -> Result<(), String> {
    start_service(&service_name)
}

#[tauri::command]
pub async fn stop_service_cmd(service_name: String) -> Result<(), String> {
    stop_service(&service_name)
}

#[tauri::command]
pub async fn restart_service_cmd(service_name: String) -> Result<(), String> {
    restart_service(&service_name)
}

#[tauri::command]
pub async fn list_all_services_cmd() -> Result<Vec<FrontendServiceInfo>, String> {
    let services = enumerate_services()?;
    Ok(services
        .into_iter()
        .map(|s| FrontendServiceInfo {
            status: s.status,
            is_nssm: false,
            name: s.name,
            display_name: s.display_name,
            image_path: s.image_path,
        })
        .collect())
}

#[tauri::command]
pub async fn list_nssm_services_cmd() -> Result<Vec<FrontendServiceInfo>, String> {
    let services = enumerate_services()?;
    Ok(services
        .into_iter()
        .filter(|s| s.is_nssm_service())
        .map(|s| FrontendServiceInfo {
            status: s.status,
            is_nssm: true,
            name: s.name,
            display_name: s.display_name,
            image_path: s.image_path,
        })
        .collect())
}

#[tauri::command]
pub async fn get_service_log_cmd(service_name: String, lines: usize) -> Result<String, String> {
    get_service_log(&service_name, lines)
}
