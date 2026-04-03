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

#[derive(Serialize, Clone)]
pub struct NssmUpgradeResult {
    pub needs_upgrade: bool,
    pub is_first_install: bool,
    pub running_services_count: usize,
}

#[derive(Serialize, Clone)]
pub struct NssmUpgradeDoneResult {
    pub success: bool,
    pub restarted_services: Vec<String>,
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

#[tauri::command]
pub async fn check_nssm_upgrade_cmd() -> Result<NssmUpgradeResult, String> {
    match check_nssm_upgrade_needed() {
        Ok(NssmUpgradeStatus::NoActionNeeded) => {
            Ok(NssmUpgradeResult {
                needs_upgrade: false,
                is_first_install: false,
                running_services_count: 0,
            })
        }
        Ok(NssmUpgradeStatus::FirstInstall) => {
            Ok(NssmUpgradeResult {
                needs_upgrade: true,
                is_first_install: true,
                running_services_count: 0,
            })
        }
        Ok(NssmUpgradeStatus::UpgradeNeeded) => {
            let nssm_services = enumerate_services()?;
            let running_count = nssm_services
                .iter()
                .filter(|s| s.is_nssm_service() && s.status.to_uppercase() == "RUNNING")
                .count();
            Ok(NssmUpgradeResult {
                needs_upgrade: true,
                is_first_install: false,
                running_services_count: running_count,
            })
        }
        Err(e) => Err(e),
    }
}

#[tauri::command]
pub async fn get_version_cmd() -> Result<String, String> {
    Ok(env!("CARGO_PKG_VERSION").to_string())
}

#[tauri::command]
pub async fn perform_nssm_upgrade_cmd() -> Result<NssmUpgradeDoneResult, String> {
    let restarted = perform_nssm_upgrade()?;
    Ok(NssmUpgradeDoneResult {
        success: true,
        restarted_services: restarted,
    })
}
