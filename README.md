# cowfs

一个基于 Rust 的轻量级 Copy-On-Write（COW）内存/磁盘/网络文件系统，并支持快照（Snapshot）管理，通过 RESTful API 提供服务。  
本项目采用依赖倒置原则，后端存储实现可插拔，便于扩展为多种存储方式（如内存、磁盘、NetDisk等）。

## 主要特性

- **COW机制**：文件变更自动实现高效内存共享与复制，节约资源。
- **快照管理**：支持创建和恢复文件系统快照，适合回滚和版本控制。
- **多后端支持**：
  - 内存（默认，适合测试和小型应用）
  - 磁盘（持久化至本地目录）
  - NetDisk（预留接口，便于后续扩展远程存储）
- **接口可扩展**：通过 trait 抽象文件系统操作，后端实现可插拔。
- **RESTful API**：标准 HTTP 接口，易与其他语言/系统集成。

## 快速开始

1. 克隆仓库并安装 Rust 依赖
2. 选择后端运行（默认为内存，可选 `disk` 或 `netdisk`）：
   ```bash
   FS_BACKEND=memory cargo run
   FS_BACKEND=disk cargo run
   FS_BACKEND=netdisk cargo run
   ```
3. 使用接口
   - 创建或更新文件: `PUT /file/{name}`，内容为文本
   - 获取文件内容: `GET /file/{name}`
   - 创建快照: `POST /snapshot`
   - 恢复快照: `POST /snapshot/{id}/restore`

## 系统启动命令说明

项目启动时需指定文件系统后端类型（内存、磁盘或网络），通过环境变量 `FS_BACKEND` 控制，具体命令如下：

```bash
# 启动内存后端（默认，适合快速测试和开发）
FS_BACKEND=memory cargo run

# 启动磁盘后端（持久化数据到本地目录）
FS_BACKEND=disk cargo run

# 启动网络磁盘后端（如有实现，便于远程扩展）
FS_BACKEND=netdisk cargo run
```

如需在生产环境编译并运行 release 版本，可使用：

```bash
FS_BACKEND=memory cargo run --release
```

如果未指定 `FS_BACKEND`，系统将默认采用内存后端。

## 目录结构

- `src/filesystem.rs`   —— 文件系统操作 trait 抽象
- `src/mini_fs.rs`      —— 内存 COW 文件系统实现
- `src/disk_fs.rs`      —— 磁盘文件系统实现
- `src/net_disk_fs.rs`  —— 网络文件系统（预留Stub）
- `src/snapshot_manager.rs` —— 快照管理
- `src/main.rs`         —— 服务入口及 REST API 实现

## 接口示例

```sh
# 创建文件
curl -X PUT http://localhost:8080/file/example.txt -d "hello cowfs"
# 读取文件
curl http://localhost:8080/file/example.txt
# 创建快照
curl -X POST http://localhost:8080/snapshot
# 恢复快照
curl -X POST http://localhost:8080/snapshot/{id}/restore
```

## 适用场景

- 测试/开发中的临时文件系统
- 微服务分布式存储原型
- 教学/研究 COW 与快照机制
- 可扩展的文件服务后端

## 扩展说明

- 新增后端只需实现 `FileSystem` trait 并注册到服务层即可。
- 支持多语言客户端调用 REST API。
- 可进一步扩展为 gRPC、CLI 等服务接口。

---

**欢迎 PR 和建议！**
