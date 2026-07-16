## Why

ProjectMender 需要先具备可在 macOS 与 Windows 上运行的最小桌面基础，以及可验证的本地权限边界，后续扫描、模型和补丁能力才不会默认继承用户的完整文件、进程和凭据权限。现在建立该基础，可以让后续每个阶段在明确的安全约束下独立交付与验证。

## What Changes

- 新建 Tauri 2、React/TypeScript 与 Rust 的跨平台桌面应用骨架，并提供最小应用状态与错误边界。
- 建立用户显式选择项目目录后的受限访问能力，以及与应用私有数据目录分离的本地数据边界。
- 建立 API Key 的系统钥匙串存储、读取和移除能力；密钥不得写入项目目录、普通配置、报告或日志。
- 建立操作授权状态模型，为后续网络、进程、项目写入和模型出站审批保留统一接口；本阶段不实际执行这些高风险操作。
- 新增基础本地审计元数据和错误呈现，不收集或发送遥测、项目源代码或凭据。

## Capabilities

### New Capabilities

- `desktop-application-shell`: 在 macOS Apple Silicon 与 Windows 11 x64 上启动最小 ProjectMender 桌面应用，并提供基础状态和错误边界。
- `project-directory-access`: 只访问用户显式选择的项目目录，并将项目访问与应用私有数据隔离。
- `local-credential-storage`: 使用系统钥匙串保存、读取和删除模型 API Key，且不将原始值持久化到普通文件。
- `operation-authorization-state`: 表达高风险操作的待确认、批准、拒绝与取消状态，但不在本阶段执行网络、命令或项目写入。

### Modified Capabilities

无。仓库当前没有既有 OpenSpec 规格。

## Impact

- 新增桌面应用工程、跨平台构建配置、Rust 命令边界、前端基础状态与本地数据目录抽象。
- 新增系统钥匙串集成和仅含元数据的本地审计记录。
- 不新增项目扫描规则、云端模型请求、外部 worker、命令执行、OpenSpec 工件写入或源代码补丁能力；这些均由后续阶段独立变更实现。
