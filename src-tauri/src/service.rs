use std::process::Command;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ServiceInfo {
    pub name: String,
    pub display_name: String,
    pub status: String,
}

pub fn enumerate_services() -> Result<Vec<ServiceInfo>, String> {
    let mut command = Command::new("powershell");
    command.args(&[
        "-WindowStyle", "Hidden",
        "-NoProfile",
        "-Command",
        "$OutputEncoding = [Console]::OutputEncoding = [System.Text.Encoding]::UTF8; [Console]::InputEncoding = [System.Text.Encoding]::UTF8; chcp 65001 > $null; Get-Service | ForEach-Object { Write-Output ($_.Name + '|' + $_.DisplayName + '|' + $_.Status.ToString()) }",
    ]);

    #[cfg(windows)]
    command.creation_flags(CREATE_NO_WINDOW);

    let output = command
        .output()
        .map_err(|e| format!("Failed to run powershell: {}", e))?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(format!("powershell failed: {}", err));
    }

    let text = String::from_utf8_lossy(&output.stdout);
    parse_powershell_output(&text)
}

fn parse_powershell_output(text: &str) -> Result<Vec<ServiceInfo>, String> {
    let mut services = Vec::new();

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.splitn(3, '|').collect();
        if parts.len() == 3 {
            let name = parts[0].trim().to_string();
            let display_name = parts[1].trim().to_string();
            let status_raw = parts[2].trim().to_uppercase();

            let status = match status_raw.as_str() {
                "RUNNING" => "running",
                "STOPPED" => "stopped",
                "PAUSED" => "paused",
                _ => "unknown",
            };

            if !name.is_empty() {
                services.push(ServiceInfo {
                    name,
                    display_name,
                    status: status.to_string(),
                });
            }
        }
    }

    Ok(services)
}

pub fn get_service_image_path(service_name: &str) -> Result<String, String> {
    let mut command = Command::new("sc");
    command.args(&["qc", service_name]);

    #[cfg(windows)]
    command.creation_flags(CREATE_NO_WINDOW);

    let output = command
        .output()
        .map_err(|e| format!("Failed to run sc qc: {}", e))?;

    if !output.status.success() {
        return Err("Failed to query service config".to_string());
    }

    let text = String::from_utf8_lossy(&output.stdout);
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with("BINARY_PATH_NAME") {
            if let Some(path) = line.splitn(2, ':').nth(1) {
                return Ok(path.trim().to_string());
            }
        }
    }

    Err("Image path not found".to_string())
}
