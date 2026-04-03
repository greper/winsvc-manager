# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

## [1.2.0](https://github.com/greper/winsvc-manager/compare/v1.1.0...v1.2.0) (2026-04-03)


### Features

* 添加 silent 参数到 checkAndUpgradeNssm ([3e6f516](https://github.com/greper/winsvc-manager/commit/3e6f516bee907c4dcf51e356b6f69d048b77f4a2))
* 添加版本获取命令并更新应用标题 ([7a7434f](https://github.com/greper/winsvc-manager/commit/7a7434f1cacc9142c090567fab131493c8416238))
* 添加工具菜单和 NSSM 升级功能 ([e2f1664](https://github.com/greper/winsvc-manager/commit/e2f16645b11fff1f30ab0fb2ab07cb974be49cca))
* 添加自定义发布脚本，支持编译检查和版本确认 ([5153377](https://github.com/greper/winsvc-manager/commit/515337797c37ee6d83169d113491ec583522f2ad))


### Bug Fixes

* 将升级 NSSM 改为独立按钮，去掉下拉菜单 ([3ceaee5](https://github.com/greper/winsvc-manager/commit/3ceaee5d58b1df4d2952b81561192726cd95181f))
* 修复发布脚本，移除 postrelease 钩子避免 Ctrl+C 后继续执行 ([d8184b1](https://github.com/greper/winsvc-manager/commit/d8184b1e6525ee149e7fc0bfa030041e2db0d3f3))
* 修复工具菜单点击无响应问题 ([3a50941](https://github.com/greper/winsvc-manager/commit/3a50941b0cb648fd352d33b3e7b1833bced97f2e))
* **nssm:** 优化NSSM自动升级被占用的问题 ([cdec0c8](https://github.com/greper/winsvc-manager/commit/cdec0c89ed50fa5f4bf67e6c24828787b8c37bcd))


### Performance Improvements

* **ui:** 优化服务列表和日志显示并添加中文语言支持 ([02581cc](https://github.com/greper/winsvc-manager/commit/02581cc662c23917a4201bd2b8bd1cd2dba98d4a))


### Documentation

* readme ([31929ad](https://github.com/greper/winsvc-manager/commit/31929addcfc036a24c1b343e3322804a42d9da15))


### Refactoring

* 移除停止所有服务按钮 ([7a5d6dd](https://github.com/greper/winsvc-manager/commit/7a5d6ddc889a010ede9964cd4f2c897ff5cf9e0b))

## [1.1.0](https://github.com/greper/winsvc-manager/compare/v1.0.0...v1.1.0) (2026-04-01)


### Features

* 添加发布前自动编译检查 ([cde4129](https://github.com/greper/winsvc-manager/commit/cde4129129e0b12cc4d45254e1b89a89cc4529be))


### Bug Fixes

* 修复 Tauri 2.x NSIS 配置 ([93133c4](https://github.com/greper/winsvc-manager/commit/93133c47b9f31cf2d26cef3520d93cb9bcac9c83))

## 1.0.0 (2026-04-01)


### Features

* 界面版本号动态获取 ([f55294c](https://github.com/greper/winsvc-manager/commit/f55294ca4012e125e935a3226d6d4522cb48dd63))
* 添加发布脚本和停止所有服务功能 ([e22424b](https://github.com/greper/winsvc-manager/commit/e22424bb8bc24deaba6eeb2e010f86052843d5eb))
* NSSM Windows Service Manager - initial commit ([3fbd228](https://github.com/greper/winsvc-manager/commit/3fbd228111567363c0524d46b97c2d8e9908ac47))
* v1.0.0 - add admin UAC, version display, service log viewer ([26d0903](https://github.com/greper/winsvc-manager/commit/26d0903c4862bafd61e1db2184f83090feabd003))


### Bug Fixes

* 修复版本更新脚本支持 ES module 项目 ([7f58b21](https://github.com/greper/winsvc-manager/commit/7f58b21233ed01749191509d0f0585b94e8a20e3))
* 修复服务日志显示问题 ([38dd9c1](https://github.com/greper/winsvc-manager/commit/38dd9c11eb5ee9ddf7c3740a1b1437a3dfd7f9ba))


### Performance Improvements

* 支持打开请求管理员运行 ([e24b759](https://github.com/greper/winsvc-manager/commit/e24b7590d74dfc9d288589a9a5567179ff06e790))


### Documentation

* add README.md ([546fe32](https://github.com/greper/winsvc-manager/commit/546fe3255047accc348fef3d1b71d2e8639d6a62))


### CI/CD

* 更新 GitHub Action 使用 pnpm ([9526499](https://github.com/greper/winsvc-manager/commit/9526499037dd8b4966ec917b62583ce1aa46d80b))
* 添加 GitHub Action 发布工作流程 ([e43554b](https://github.com/greper/winsvc-manager/commit/e43554b403e2e82aa858d8cba4db3803a089d63f))

### [0.0.1](https://github.com/greper/winsvc-manager/compare/v1.0.0...v0.0.1) (2026-04-01)


### Features

* 界面版本号动态获取 ([f55294c](https://github.com/greper/winsvc-manager/commit/f55294ca4012e125e935a3226d6d4522cb48dd63))
* 添加发布脚本和停止所有服务功能 ([e22424b](https://github.com/greper/winsvc-manager/commit/e22424bb8bc24deaba6eeb2e010f86052843d5eb))


### Bug Fixes

* 修复版本更新脚本支持 ES module 项目 ([7f58b21](https://github.com/greper/winsvc-manager/commit/7f58b21233ed01749191509d0f0585b94e8a20e3))
