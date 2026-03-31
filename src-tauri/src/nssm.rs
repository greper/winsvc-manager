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
