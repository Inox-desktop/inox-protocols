# AGENTS.md — inox-protocols

本仓库遵循根级 [AGENTS.md](https://github.com/Inox-desktop/inox/blob/main/AGENTS.md) 的**全部硬性规则**，本文件只补充仓库特有内容，不放宽任何根级约束。

## 职责

标准协议绑定与自定义协议扩展。所属层级：**L2**。

## 仓库特有约束

- 默认禁止 `unsafe`：`Cargo.toml` 中 `[lints.rust] unsafe_code = "forbid"`。
- 非测试路径禁止 `unwrap()` / `expect()` / `panic!()` / `todo!()` / `unimplemented!()`。
- 日志统一使用 `tracing`，禁止 `println!` / `eprintln!`。
- **MSRV 为 Rust 1.86**：`wayland-protocols` 0.32 要求 rustc ≥ 1.86，故不再支持 1.85。版本在 `Cargo.toml` 的 `rust-version` 与 `rust-toolchain.toml` 中锁定。
- **协议以 XML 为唯一事实来源**：`protocol/` 下的协议 XML 是唯一事实来源；修改协议必须编辑 XML 并让绑定重新生成，**禁止直接修改生成的 Rust 代码**。`src/ipc.rs` 中的 `inox-ipc` 绑定由 `wayland-scanner` 在编译期从 `protocol/inox-ipc.xml` 生成。
- **`inox-ipc` 为 unstable 私有协议**：在 Inox 1.0 之前接口允许破坏性变更。消费者必须通过 `version` 事件协商能力，不得假定固定版本。破坏性变更必须先写 ADR（根级 AGENTS.md §4 / §7.2），并在 PRD 协议矩阵同步更新。
- 尚未引入 Inox 跨仓库依赖；首次需要时按根级 §8 使用锁定 `tag` / `rev` 的 git 依赖，禁止未锁定的 `branch`。

## 主要依赖

- 核心：`wayland-client`、`wayland-server`、`wayland-backend`。
- 标准协议：`wayland-protocols`（client / server / staging / unstable）、`wayland-protocols-wlr`（client / server）。
- 构建期：`wayland-scanner`（由 `protocol/inox-ipc.xml` 生成绑定）。

## 命令

> 仅作参考，不作为本地验收依据；真正判定在 CI。

| 目的 | 命令 |
|------|------|
| 格式化 | `cargo fmt --all` |
| 格式检查 | `cargo fmt --all -- --check` |
| 静态检查 | `cargo clippy --all-targets --all-features -- -D warnings` |
| 测试 | `cargo test --all-features` |
| 构建 | `cargo build --release --all-features` |
| 文档 | `cargo doc --no-deps --all-features` |
| 依赖检查 | `cargo deny check` |

## 测试要求

- 单元测试（`#[cfg(test)]`）：`src/ipc.rs` 内的接口元数据测试，断言 `inox_ipc` 的接口名 / 版本、请求集、唯一析构请求、事件集与 `reload_result` 枚举取值稳定。
- 集成测试：`tests/ipc_roundtrip.rs` 通过真实 Unix socket pair 完成 client ↔ server 往返，断言 `version` 与 `config_reloaded` 事件；两者都必须保持通过。
- 新增或修改行为必须同时新增 / 更新对应测试。
- 测试代码不受“禁止 unwrap / expect / panic”约束。

## 目录结构

```
src/            源码（lib.rs 库入口；standard.rs 标准协议绑定再导出；ipc.rs 由 XML 生成的 inox-ipc 绑定）
protocol/       协议定义 XML（源码级唯一事实来源，如 inox-ipc.xml）
tests/          集成测试（如 ipc_roundtrip.rs）
.github/        CI 工作流
deny.toml       cargo-deny 依赖与许可证策略
```

## 提交信息格式

Conventional Commits，scope 使用 `protocols`。
