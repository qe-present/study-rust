# 测试发布crate
# 1. 创建一个新的crate
```shell
cargo new --lib my_crate
```
# 2.发布
```shell
cargo publish
```
# 3 弃用 crate
```shell
cargo yank --vers 0.1.0
```