# Tauri NSSM Windows Service Manager Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 创建一个基于 Tauri 的 Windows 服务管理工具，封装 NSSM，提供图形化界面进行服务安装、卸载、启动、停止、重启和列表查看。

**Architecture:** 使用 Tauri 框架，前端 Vue 3 + TypeScript + Ant Design Vue 提供 UI，后端 Rust 封装 NSSM 命令和 Windows 服务 API，通过 Tauri IPC 通信。NSSM.exe 内置打包到应用中。

**Tech Stack:** Tauri 2.x, Vue 3, TypeScript, Ant Design Vue, Rust, NSSM 2.24

---

## File Structure

```
tauri-nssm-service-manager/
├── index.html
├── package.json
├── tsconfig.json
├── vite.config.ts
├── src/
│   ├── main.ts
│   ├── App.vue
│   ├── types.ts
│   ├── components/
│   │   ├── ServiceList.vue
│   │   ├── InstallDialog.vue
│   │   └── ServiceActions.vue
│   └── views/
│       └── ServiceManager.vue
├── src-tauri/
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands.rs
│   │   ├── nssm.rs
│   │   └── service.rs
│   └── resources/
│       └── nssm.exe
```

---

## Tasks

### Task 1: 初始化 Tauri Vue 项目

**Files:**
- Create: 项目目录结构

- [ ] **Step 1: 使用 npm create tauri-app 初始化项目**

```bash
npm create tauri-app@latest -- --template vue-ts . --manager npm
```

- [ ] **Step 2: 安装基础依赖**

```bash
npm install
```

- [ ] **Step 3: 添加 Ant Design Vue 和图标库**

```bash
npm install ant-design-vue @ant-design/icons-vue
```

### Task 2: 下载并配置 NSSM

**Files:**
- Create: `src-tauri/resources/nssm.exe`

- [ ] **Step 1: 下载 NSSM 2.24 64-bit**

从 https://nssm.cc/release/nssm-2.24.zip 下载，解压后复制 `nssm-2.24/win64/nssm.exe`

- [ ] **Step 2: 放置到 src-tauri/resources 目录**

创建 `src-tauri/resources` 目录并放入 `nssm.exe`

### Task 3: 实现后端 NSSM 封装模块

**Files:**
- Create: `src-tauri/src/nssm.rs`

- [ ] **Step 1: 创建 nssm.rs**

```rust
use std::path::PathBuf;
use std::process::Command;

pub fn get_nssm_path() -> PathBuf {
    let exe_path = std::env::current_exe().expect("Failed to get current exe path");
    let exe_dir = exe_path.parent().expect("Failed to get parent directory");
    let nssm_path = exe_dir.join("resources").join("nssm.exe");
    
    if !nssm_path.exists() {
        // 开发环境下，从项目资源目录查找
        let resource_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("resources");
        resource_dir.join("nssm.exe")
    } else {
        nssm_path
    }
}

pub fn run_nssm(args: &[&str]) -> Result<String, String> {
    let nssm_path = get_nssm_path();
    let mut command = Command::new(&nssm_path);
    command.args(args);
    
    let output = command.output()
        .map_err(|e| format!("Failed to execute NSSM: {}", e))?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr).to_string();
        Err(format!("NSSM error: {}", error_msg))
    }
}

pub fn install_service(service_name: &str, exe_path: &str, args: Option<&str>) -> Result<(), String> {
    let mut cmd_args = vec!["install", service_name, exe_path];
    if let Some(a) = args {
        cmd_args.push(a);
    }
    run_nssm(&cmd_args)?;
    Ok(())
}

pub fn remove_service(service_name: &str) -> Result<(), String> {
    run_nssm(&["remove", service_name, "confirm"])?;
    Ok(())
}

pub fn start_service(service_name: &str) -> Result<(), String> {
    run_nssm(&["start", service_name])?;
    Ok(())
}

pub fn stop_service(service_name: &str) -> Result<(), String> {
    run_nssm(&["stop", service_name])?;
    Ok(())
}

pub fn restart_service(service_name: &str) -> Result<(), String> {
    run_nssm(&["restart", service_name])?;
    Ok(())
}
```
