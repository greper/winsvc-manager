# WinSvc Manager - 项目规格说明书

## 1. 项目概述

WinSvc Manager 是一个基于 Tauri 2.x 构建的 Windows 服务管理工具，封装了 NSSM (Non-Sucking Service Manager)，提供简洁的图形化界面来管理 Windows 服务。

### 1.1 核心价值

- 可视化包装应用成为 Windows 服务，使其能够在后台运行
- 简化 Windows 服务的安装、配置和管理流程
- 提供安全可靠的服务管理解决方案

## 2. 功能特性

### 2.1 核心功能

| 功能 | 描述 |
|------|------|
| **服务安装** | 可视化安装 Windows 服务，支持自动解析命令路径和参数 |
| **服务控制** | 启动、停止、重启服务 |
| **服务卸载** | 安全卸载已安装的服务 |
| **服务列表** | 查看 NSSM 安装的服务和系统所有服务 |
| **操作日志** | 实时记录所有操作历史 |
| **双架构支持** | 内置 x86 和 x64 版本的 NSSM |

### 2.2 扩展功能

- NSSM 版本检查与升级
- 服务日志查看
- 管理员权限检测与提示

## 3. 技术架构

### 3.1 技术栈

| 组件 | 技术 |
|------|------|
| 框架 | Tauri 2.x |
| 前端 | Vue 3 + TypeScript |
| UI 库 | Ant Design Vue |
| 后端 | Rust |
| 服务管理 | NSSM 2.24 |

### 3.2 系统架构

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

## 4. 项目结构

```
winsvc-manager/
├── src/                    # Vue 前端代码
│   ├── components/         # Vue 组件
│   │   ├── InstallDialog.vue
│   │   ├── ServiceList.vue
│   │   └── ServiceLogDialog.vue
│   ├── App.vue            # 主界面
│   ├── main.ts            # 应用入口
│   └── types.ts           # TypeScript 类型定义
├── src-tauri/             # Rust 后端
│   ├── src/
│   │   ├── commands.rs    # Tauri IPC 命令处理
│   │   ├── main.rs        # 应用入口
│   │   ├── nssm.rs        # NSSM 命令封装
│   │   └── service.rs     # Windows 服务 API 封装
│   ├── resources/         # NSSM 可执行文件
│   │   ├── win32/nssm.exe
│   │   └── win64/nssm.exe
│   ├── Cargo.toml         # Rust 依赖配置
│   └── tauri.conf.json    # Tauri 配置
├── docs/                  # 文档资源
│   ├── screenshots/       # 应用截图
│   └── superpowers/       # Superpowers 设计文档
└── package.json           # 前端依赖配置
```

## 5. API 接口

### 5.1 前端类型定义

```typescript
export interface ServiceInfo {
  name: string;
  display_name: string;
  status: 'running' | 'stopped' | 'paused' | 'unknown';
  is_nssm: boolean;
  image_path?: string;
}
```

### 5.2 后端 API 接口

| 命令 | 参数 | 返回值 | 描述 |
|------|------|--------|------|
| `install_service_cmd` | `service_name: String`, `exe_path: String`, `args: Option<String>` | `Result<(), String>` | 安装新服务 |
| `remove_service_cmd` | `service_name: String` | `Result<(), String>` | 卸载服务 |
| `start_service_cmd` | `service_name: String` | `Result<(), String>` | 启动服务 |
| `stop_service_cmd` | `service_name: String` | `Result<(), String>` | 停止服务 |
| `restart_service_cmd` | `service_name: String` | `Result<(), String>` | 重启服务 |
| `list_all_services_cmd` | - | `Result<Vec<FrontendServiceInfo>, String>` | 获取所有服务列表 |
| `list_nssm_services_cmd` | - | `Result<Vec<FrontendServiceInfo>, String>` | 获取 NSSM 安装的服务列表 |
| `get_service_log_cmd` | `service_name: String`, `lines: usize` | `Result<String, String>` | 获取服务日志 |
| `check_nssm_upgrade_cmd` | - | `Result<NssmUpgradeResult, String>` | 检查 NSSM 是否需要升级 |
| `perform_nssm_upgrade_cmd` | - | `Result<NssmUpgradeDoneResult, String>` | 执行 NSSM 升级 |

## 6. 部署说明

### 6.1 环境要求

- Node.js >= 18
- pnpm
- Rust >= 1.70
- Visual Studio Build Tools (Windows)

### 6.2 开发流程

```bash
# 安装依赖
pnpm install

# 开发模式
pnpm tauri dev

# 构建生产版本
pnpm tauri build
```

### 6.3 产物位置

构建产物位于 `src-tauri/target/release/bundle/nsis/` 目录。

## 7. 安全考虑

### 7.1 权限要求

- 管理 Windows 服务需要管理员权限
- 应用启动时会自动检测管理员权限
- 非管理员模式下会提示用户重新以管理员身份运行

### 7.2 数据安全

- 所有服务操作都通过 NSSM 安全执行
- 服务配置存储在 Windows 注册表中
- 操作日志仅保存在本地内存中

## 8. 维护与升级

### 8.1 NSSM 升级

- 支持自动检测 NSSM 版本
- 提供一键升级功能
- 升级过程中自动重启受影响的服务

### 8.2 版本管理

使用 SemVer 版本规范：`MAJOR.MINOR.PATCH`

## 9. 附录

### 9.1 NSSM 命令参考

NSSM (Non-Sucking Service Manager) 是一个开源的 Windows 服务管理工具，项目地址：https://nssm.cc/

### 9.2 Tauri 文档

Tauri 官方文档：https://tauri.app/

---

**文档版本**: 1.0.0
**最后更新**: 2026-04-03