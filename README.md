# ProjectMender

ProjectMender 是一个桌面优先的代码项目治理工作台。它面向本地或远程代码项目，计划将代码质量、安全治理、技术债、重构和工程化增强建议，整理为带证据、优先级与审批状态的可执行清单。

项目采用分阶段 OpenSpec 流程交付：先建立安全边界，再逐步加入只读扫描、问题聚合、云端 AI 审查、方案治理和受控修复能力。

> 当前为早期开发版本。已完成第 1 阶段的大部分桌面基础与安全边界；尚未提供真实的项目扫描、云端模型调用、命令执行或自动改码功能。

## 已实现：第 1 阶段桌面与安全基础

- Tauri 2 + React/TypeScript + Rust 的桌面应用骨架。
- 用户显式选择本地项目目录后，使用不透明会话标识符进行受限只读访问。
- 每次项目读取都拒绝绝对路径、`..` 穿越、失效会话和逃逸到项目目录外的符号链接。
- 应用私有数据目录与被分析项目目录分离；初始化失败时进入恢复模式，不会回退写入项目目录。
- 模型 API Key 通过操作系统凭据设施保存；常规设置只保存供应商、模型名和凭据引用，不序列化原始密钥。
- 高风险操作的授权生命周期模型：待确认、已批准、已拒绝、已取消、已过期。第 1 阶段的授权只表达状态，不能触发任何执行。
- 最小 Tauri 命令面，以及前端的授权状态展示与显式批准、拒绝、取消意图。

第 1 阶段明确不包含：网络客户端、云端模型请求、子进程/命令执行、扫描器、项目写入、补丁应用和遥测。

完整边界和阶段二交接约束见 [Phase 1 security boundary](docs/phase-1-security-boundary.md)。

## 路线图

ProjectMender 计划按六个独立 OpenSpec 变更推进：

1. `desktop-foundation-and-security-boundaries`：桌面基础、安全边界与授权状态。
2. `readonly-scan-orchestration`：只读扫描编排与受控工作单元。
3. `v1-diagnostic-rule-packs`：初版质量、安全和工程化规则包。
4. `findings-baselines-and-prioritization`：问题聚合、证据、基线和优先级。
5. `ai-review-and-openspec-governance`：模型审查、上下文预算、人工审批与 OpenSpec 治理。
6. `patch-validation-and-repair-loop`：受控改码、验证和返工闭环。

## 本地运行

### 前置条件

- Node.js 20 或更高版本
- Rust stable 工具链（`rustc` 与 `cargo`）
- macOS：Xcode Command Line Tools
- Windows：Microsoft C++ Build Tools 与 WebView2 运行时

### 安装与启动

```bash
npm install
npm run dev
```

在另一个终端启动桌面开发窗口：

```bash
npx tauri dev
```

### 验证

```bash
npm test
npm run build
cargo test --manifest-path src-tauri/Cargo.toml
```

## 安全与隐私原则

- 项目目录访问从用户显式选择开始，且限定为当前会话的项目根目录。
- 原始 API Key 只应存在于操作系统钥匙串；不得写入项目目录、扫描报告、常规配置、日志或普通 UI 状态。
- 将来任何向云端模型发送内容的功能，必须先生成可检查的上下文预览，并经过用户明确批准；敏感文件默认不发送。
- 将来任何命令执行、项目写入或补丁应用，都必须单独审批，不能因“已信任项目”或“已批准 AI 审查”而自动获得权限。

## OpenSpec 状态

第 1 阶段的 OpenSpec 任务目前为 19/22 完成。未完成项是：

- 对所有未来持久化面进行 API Key 无泄漏的全链路证明。
- 对禁止能力（网络、进程、模型、扫描、项目写入和补丁）的系统性配置验证。
- Windows 11 x64 的真实运行、打包和凭据设施不可用场景验证。

因此，当前提交可作为 macOS 上的基础实现增量使用，但不是完整的跨平台安全发布。

## 开发约定

- 设计与任务以 `openspec/changes/` 中的变更工件为准。
- 每个阶段独立规划、实施、验证和归档，不跳过前置安全边界。
- 代码修改应先有测试或可验证的失败场景，再以最小实现使其通过。
- 不要将密钥、私钥、`.env` 内容或真实项目源代码添加到仓库。

## 许可

许可证尚未确定。
