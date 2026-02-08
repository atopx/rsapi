# RSAPI - 轻量级 Axum API 模板

一个面向生产可观测性的最小 Rust API 服务模板，默认包含：

- `tracing + tracing-subscriber` 结构化日志
- `tracing-opentelemetry` 自动 trace 上下文管理
- 自动提取并透传 `traceparent`（W3C Trace Context）
- 响应体 `trace_id` 与响应头 `x-trace-id` 一致输出
- `json` / `pretty` 双日志模式

## 关键能力

### 1. Handler 日志自动携带 trace_id

业务代码只需正常写日志：

```rust
tracing::info!("example handler");
```

无需手动拼接 `trace_id`，日志中会自动包含当前请求 span 的 `trace_id/span_id`。

### 2. 跨服务链路追踪

如果请求携带 `traceparent`，服务会继承上游 `trace_id`；否则自动生成新的 trace。

### 3. 一致的排障入口

每个请求都返回：

- Header: `x-trace-id`
- Body: `{"trace_id":"..."}`（在统一响应结构中）

可直接用于前后端/网关/日志平台联动排障。

## 配置（`setting.toml`）

```toml
[service]
listen_addr = "0.0.0.0:3000"

[logging]
level = "info,axum::rejection=trace"
format = "json" # json or pretty
service_name = "rsapi"
```

## 质量保障

已内置集成测试覆盖日志链路关键行为：

- `traceparent` 透传到响应 `trace_id`
- `x-trace-id` 与响应体 `trace_id` 一致且有效
