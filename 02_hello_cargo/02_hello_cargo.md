# Hello_cargo_2

Cargo是Rust的构建系统和包管理器。

## 学习总结

1. 使用`cargo new`创建项目：`cargo new project_name`
2. Cargo生成的项目默认使用Git版本控制（除非在已有Git仓库中创建）
3. 项目文件结构：
   - `Cargo.toml`：配置文件（类似package.json或Gemfile）
   - `src/`目录：源代码放在这里
   - `.gitignore`：Git忽略文件
4. `Cargo.toml`包含：
   - `[package]`：项目元数据
   - `[dependencies]`：项目依赖
5. Cargo命令：
   - `cargo build`：构建项目
   - `cargo run`：构建并运行项目
   - `cargo check`：检查代码是否可编译（比build快）
6. Cargo在`target/debug`目录下生成构建文件
7. `Cargo.lock`文件确保可重现的构建
8. `cargo build --release`创建优化的生产构建

---

| [上一页：Hello World](../01_hello_world/01_hello_world.md) | [下一页：猜数游戏](../03_guessing_game/03_guessing_game.md) |
|------------------------|------------------------|