# Tauri NSSM Windows Service Manager Design

## Overview

使用 Tauri 构建一个 Windows 服务管理小工具，封装 NSSM (Non-Sucking Service Manager)，提供图形化界面管理 Windows 服务。

### Technology Stack

- **Framework**: Tauri 2.x
- **Frontend**: Vue 3 + TypeScript
- **UI Library**: Ant Design Vue
- **Backend**: Rust
- **NSSM**: 内置打包 64-bit 版本 2.24

### Features

1. **Service Installation**
   - GUI 选择可执行文件
   - 填写服务名称和参数
   - 调用 `nssm install <servicename> <app> [args]`

2. **Service Removal**
   - 选择已安装服务
   - 调用 `nssm remove <servicename> confirm`

3. **Service Control**
   - Start: `nssm start <servicename>`
   - Stop: `nssm stop <servicename>`
   - Restart: `nssm restart <servicename>`

4. **Service Listing**
   - Tab 1: NSSM 安装的服务
   - Tab 2: 所有 Windows 服务
   - 显示服务名称、状态、可执行文件路径

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                    Frontend (Vue)                   │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐  │
│  │ Service List│ │Install Form │ │Control Buttons│  │
│  └─────────────┘ └─────────────┘ └─────────────┘  │
└─────────────────────────────────────────────────────┘
                              ↓ IPC
┌─────────────────────────────────────────────────────┐
│               Backend (Rust Tauri)                  │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐  │
│  │  Commands   │ │   NSSM      │ │  Windows    │  │
│  │  Handler    │ │   Wrapper   │ │ Service API │  │
│  └─────────────┘ └─────────────┘ └─────────────┘  │
└─────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────┐
│                System / NSSM.exe                    │
└─────────────────────────────────────────────────────┘
```

## Frontend Components

### Pages

- `src/views/ServiceManager.vue` - 主界面，包含所有功能

### Components

- `src/components/ServiceList.vue` - 服务列表表格
- `src/components/InstallDialog.vue` - 安装服务对话框
- `src/components/ServiceActions.vue` - 操作按钮（启动/停止/重启/卸载）

### Types

```typescript
interface ServiceInfo {
  name: string;
  displayName: string;
  status: 'running' | 'stopped' | 'paused' | 'unknown';
  isNssm: boolean;
  imagePath?: string;
}
```

## Backend (Rust)

### Commands

```rust
// 安装服务
fn install_service(service_name: String, exe_path: String, args: Option<String>) -> Result<(), String>

// 卸载服务
fn remove_service(service_name: String) -> Result<(), String>

// 控制服务
fn start_service(service_name: String) -> Result<(), String>
fn stop_service(service_name: String) -> Result<(), String>
fn restart_service(service_name: String) -> Result<(), String>

// 获取服务列表
fn list_services() -> Result<Vec<ServiceInfo>, String>

// 获取 nssm 安装的服务
fn list_nssm_services() -> Result<Vec<ServiceInfo>, String>

// 查询单个服务状态
fn get_service_status(service_name: String) -> Result<ServiceStatus, String>
```

### NSSM Wrapper (`nssm.rs`)

- 定位内置的 `nssm.exe` 路径
- 执行 nssm 命令并捕获输出
- 错误处理

### Windows Service API (`service.rs`)

- 使用 Windows API 枚举所有服务
- 检测服务是否由 nssm 安装（通过可执行路径判断）
- 获取服务状态

## NSSM Integration

- `nssm.exe` (64-bit 2.24) 放置在 `src-tauri/resources/nssm.exe`
- Tauri 在构建时将其打包到应用
- 运行时通过 `std::env::current_exe()` 获取应用目录，找到 nssm.exe

## UI Layout

```
┌─────────────────────────────────────────────────┐
│  🛠️ NSSM Service Manager          [刷新] [设置] │
├─────────────────────────────────────────────────┤
│  [NSSM 服务] [所有服务]                           │
├─────────────────────────────────────────────────┤
│ ┌─────────────────────────────────────────────┐ │
│ │ Name             Status   Actions            │ │
│ ├─────────────────────────────────────────────┤ │
│ │ MyService        Running  ▶ ⏹ 🔄 🗑️       │ │
│ │ AnotherService   Stopped  ▶ ⏹ 🔄 🗑️       │ │
│ └─────────────────────────────────────────────┘ │
├─────────────────────────────────────────────────┤
│  [+ 安装新服务]                                   │
└─────────────────────────────────────────────────┘
```

## Error Handling

- 前端显示错误信息（使用 Ant Design 消息提示）
- 后端捕获命令执行错误，返回错误信息给前端
- 权限检测：需要管理员权限才能管理服务

## Security Considerations

- 只允许选择可执行文件安装
- 所有命令执行在后端 Rust 进行，前端只传递参数
- NSSM 内置，不依赖外部环境

## Success Criteria

- 能正确列出所有 Windows 服务
- 能正确识别 nssm 安装的服务
- 能安装新服务并启动
- 能停止、重启、卸载已安装服务
- 应用打包后包含 nssm.exe，无需用户额外安装
