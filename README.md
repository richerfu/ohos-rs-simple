# ohos-rs

## 环境初始化

- Rust
- OpenHarmony SDK    
  需要设置`OHOS_NDK_HOME`环境变量

## 构建命令

```shell
# armv8a
cargo +nightly build --target aarch64-unknown-linux-ohos -Z build-std --release

# armv7a
cargo +nightly build --target armv7-unknown-linux-ohos -Z build-std --release

# x86_64
cargo +nightly build --target x86_64-unknown-linux-ohos -Z build-std --release
```