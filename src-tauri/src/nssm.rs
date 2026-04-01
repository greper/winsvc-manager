use std::path::PathBuf;
use std::process::Command;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

pub fn get_nssm_path() -> PathBuf {
    #[cfg(target_arch = "x86_64")]
    let arch_dir = "win64";

    #[cfg(target_arch = "x86")]
    let arch_dir = "win32";

    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
    compile_error!("Unsupported architecture");

    let exe_path = std::env::current_exe().expect("Failed to get current exe path");
    let exe_dir = exe_path.parent().expect("Failed to get parent directory");
    let nssm_path = exe_dir.join("resources").join(arch_dir).join("nssm.exe");

    if !nssm_path.exists() {
        let resource_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("resources")
            .join(arch_dir);
        resource_dir.join("nssm.exe")
    } else {
        nssm_path
    }
}

pub fn run_nssm(args: &[&str]) -> Result<String, String> {
    let nssm_path = get_nssm_path();
    let mut command = Command::new(&nssm_path);
    command.args(args);

    #[cfg(windows)]
    command.creation_flags(CREATE_NO_WINDOW);

    let output = command
        .output()
        .map_err(|e| format!("Failed to execute NSSM: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        let out = String::from_utf8_lossy(&output.stdout);
        let combined = format!("{} {}", err, out);

        if combined.contains("Access is denied") || combined.contains("5:") {
            return Err("Access Denied. Please run as Administrator.".to_string());
        }

        if combined.contains("The specified service does not exist") {
            return Err("Service not found.".to_string());
        }

        Err(format!(
            "NSSM error: code {}",
            output.status.code().unwrap_or(-1)
        ))
    }
}

pub fn run_sc(args: &[&str]) -> Result<String, String> {
    let cmd = args.join(" ");
    let ps_cmd = format!(
        "[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; sc.exe {}",
        cmd
    );

    let mut command = Command::new("powershell");
    command.args(&["-WindowStyle", "Hidden", "-Command", &ps_cmd]);

    #[cfg(windows)]
    command.creation_flags(CREATE_NO_WINDOW);

    let output = command
        .output()
        .map_err(|e| format!("Failed to execute sc: {}", e))?;

    let out = String::from_utf8_lossy(&output.stdout);
    let err = String::from_utf8_lossy(&output.stderr);

    if output.status.success() {
        Ok(out.to_string())
    } else {
        let combined = format!("{} {}", out, err);

        if combined.contains("Access is denied") || combined.contains("5:") {
            return Err("Access Denied (Error 5). Please run as Administrator.".to_string());
        }

        if combined.contains("does not exist") || combined.contains("not exist") {
            return Err("Service not found.".to_string());
        }

        let code = output.status.code().unwrap_or(-1);
        Err(format!("sc command failed with code {}", code))
    }
}

pub fn install_service(
    service_name: &str,
    exe_path: &str,
    args: Option<&str>,
) -> Result<(), String> {
    let name = service_name.trim();
    let mut cmd_args = vec!["install", name, exe_path];
    if let Some(a) = args {
        cmd_args.push(a);
    }
    run_nssm(&cmd_args)?;
    
    // Configure logging
    let log_dir = std::env::temp_dir().join("nssm_logs");
    let _ = std::fs::create_dir_all(&log_dir);
    
    let stdout_path = log_dir.join(format!("{}_stdout.log", name));
    let stderr_path = log_dir.join(format!("{}_stderr.log", name));
    
    let _ = run_nssm(&["set", name, "AppStdout", stdout_path.to_str().unwrap_or("")]);
    let _ = run_nssm(&["set", name, "AppStderr", stderr_path.to_str().unwrap_or("")]);
    let _ = run_nssm(&["set", name, "AppStdoutRotation", "1"]);
    let _ = run_nssm(&["set", name, "AppStderrRotation", "1"]);
    let _ = run_nssm(&["set", name, "AppRotateFiles", "10"]);
    
    Ok(())
}

pub fn remove_service(service_name: &str) -> Result<(), String> {
    let name = service_name.trim();
    let _ = run_sc(&["stop", name]);
    run_sc(&["delete", name])?;
    Ok(())
}

pub fn start_service(service_name: &str) -> Result<(), String> {
    let name = service_name.trim();
    run_sc(&["start", name])?;
    Ok(())
}

pub fn stop_service(service_name: &str) -> Result<(), String> {
    let name = service_name.trim();
    run_sc(&["stop", name])?;
    Ok(())
}

pub fn restart_service(service_name: &str) -> Result<(), String> {
    let name = service_name.trim();
    run_nssm(&["restart", name])?;
    Ok(())
}

pub fn get_service_log(service_name: &str, lines: usize) -> Result<String, String> {
    let name = service_name.trim();
    
    let mut info = Vec::new();
    info.push(format!("📋 Service: {}", name));
    
    // Try to get current log configuration
    let stdout_log = run_nssm(&["get", name, "AppStdout"]).ok();
    let stderr_log = run_nssm(&["get", name, "AppStderr"]).ok();
    
    let stdout_path = stdout_log.as_ref().map(|p| p.trim()).unwrap_or("");
    let stderr_path = stderr_log.as_ref().map(|p| p.trim()).unwrap_or("");
    
    info.push(format!("📄 stdout: {}", if stdout_path.is_empty() { "(not set)" } else { stdout_path }));
    info.push(format!("📄 stderr: {}", if stderr_path.is_empty() { "(not set)" } else { stderr_path }));
    info.push(String::new());
    
    // Check file status
    let mut file_status = Vec::new();
    
    if !stderr_path.is_empty() {
        match std::fs::metadata(stderr_path) {
            Ok(meta) => {
                file_status.push(format!("✅ stderr file exists ({} bytes)", meta.len()));
            }
            Err(_) => {
                file_status.push("❌ stderr file does not exist".to_string());
            }
        }
    }
    
    if !stdout_path.is_empty() {
        match std::fs::metadata(stdout_path) {
            Ok(meta) => {
                file_status.push(format!("✅ stdout file exists ({} bytes)", meta.len()));
            }
            Err(_) => {
                file_status.push("❌ stdout file does not exist".to_string());
            }
        }
    }
    
    if !file_status.is_empty() {
        info.push("📊 File status:".to_string());
        for status in file_status {
            info.push(format!("   {}", status));
        }
        info.push(String::new());
    }
    
    // Try to read logs if paths are available
    let mut logs = Vec::new();
    
    if !stderr_path.is_empty() {
        if let Ok(content) = read_last_lines(stderr_path, lines) {
            if !content.is_empty() {
                logs.push(format!("=== stderr ===\n{}", content));
            }
        }
    }
    
    if !stdout_path.is_empty() {
        if let Ok(content) = read_last_lines(stdout_path, lines) {
            if !content.is_empty() {
                logs.push(format!("=== stdout ===\n{}", content));
            }
        }
    }
    
    if logs.is_empty() {
        let mut message = info.join("\n");
        message.push_str("⚠️ No log content found.\n\n");
        message.push_str("💡 Next steps:\n");
        
        let stdout_exists = !stdout_path.is_empty() && std::path::Path::new(stdout_path).exists();
        let stderr_exists = !stderr_path.is_empty() && std::path::Path::new(stderr_path).exists();
        
        if stdout_exists || stderr_exists {
            message.push_str("   • ✅ Log files exist, but are empty\n");
            message.push_str("   • The service may not produce any output\n");
        } else if !stdout_path.is_empty() || !stderr_path.is_empty() {
            message.push_str("   • 🔄 Logs configured, but files not created yet\n");
            message.push_str("   • ⚠️ FULL STOP then START the service (not restart!)\n");
        } else {
            message.push_str("   • Logs not configured\n");
            message.push_str("   • Reinstall the service to enable logging\n");
        }
        
        message.push_str("   • Ensure the application produces console output\n");
        
        return Ok(message);
    }
    
    Ok(logs.join("\n\n"))
}

fn read_last_lines(path: &str, count: usize) -> Result<String, String> {
    use std::fs;
    let content =
        fs::read_to_string(path).map_err(|e| format!("Failed to read log file: {}", e))?;
    let lines: Vec<&str> = content.lines().collect();
    let start = if lines.len() > count {
        lines.len() - count
    } else {
        0
    };
    Ok(lines[start..].join("\n"))
}
