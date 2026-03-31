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
    pub image_path: Option<String>,
}

impl ServiceInfo {
    pub fn status_cn(&self) -> &'static str {
        match self.status.to_uppercase().as_str() {
            "RUNNING" => "运行中",
            "STOPPED" => "已停止",
            "PAUSED" => "已暂停",
            _ => "未知",
        }
    }

    pub fn is_nssm_service(&self) -> bool {
        self.image_path
            .as_ref()
            .map(|p| p.to_lowercase().contains("nssm"))
            .unwrap_or(false)
    }
}

/// 一次性获取所有服务信息（包括路径），避免逐个查询
pub fn enumerate_services() -> Result<Vec<ServiceInfo>, String> {
    let mut command = Command::new("powershell");
    command.args(&[
        "-WindowStyle", "Hidden",
        "-NoProfile",
        "-Command",
        "$OutputEncoding = [Console]::OutputEncoding = [System.Text.Encoding]::UTF8; \
         [Console]::InputEncoding = [System.Text.Encoding]::UTF8; \
         chcp 65001 > $null; \
         Get-CimInstance Win32_Service | ForEach-Object { \
             $path = if ($_.PathName) { $_.PathName.Replace('|', 'PIPE') } else { '' }; \
             Write-Output ($_.Name + '|' + $_.DisplayName + '|' + $_.State.ToString() + '|' + $path) \
         }",
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
        let parts: Vec<&str> = line.splitn(4, '|').collect();
        if parts.len() == 4 {
            let name = parts[0].trim().to_string();
            let display_name = parts[1].trim().to_string();
            let status_raw = parts[2].trim().to_uppercase();
            let image_path_raw = parts[3].trim();

            let status = match status_raw.as_str() {
                "RUNNING" => "running",
                "STOPPED" => "stopped",
                "PAUSED" => "paused",
                _ => "unknown",
            };

            // 恢复被替换的管道符
            let image_path = if image_path_raw.is_empty() {
                None
            } else {
                Some(image_path_raw.replace("PIPE", "|"))
            };

            if !name.is_empty() {
                services.push(ServiceInfo {
                    name,
                    display_name,
                    status: status.to_string(),
                    image_path,
                });
            }
        }
    }

    Ok(services)
}
