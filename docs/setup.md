# 环境配置

参考：[https://rcore-os.github.io/rCore-Tutorial-deploy/docs/pre-lab/env.html](https://rcore-os.github.io/rCore-Tutorial-deploy/docs/pre-lab/env.html)

## 安装 qemu

可以从源码安装，尝试以下命令即为安装成功：

```
yunwei@ubuntu:~/Desktop$ qemu-system-riscv64 --version
QEMU emulator version 5.0.0
Copyright (c) 2003-2020 Fabrice Bellard and the QEMU Project developers
```

此处使用的版本是 5.0.0

## 安装 Rust 工具链

安装完成后，使用 `rustc --version` 确认当前版本:

```
yunwei@ubuntu:~/Desktop$ rustc --version
rustc 1.49.0-nightly (91a79fb29 2020-10-07)
```

此处需要使用 `nightly` 版本的 rust，如果和上面不相同，可以尝试：

```
rustup install nightly
rustup default nightly
```

切换到 `nightly` 版本。

由于官方不保证 nightly 版本的 ABI 稳定性，也就意味着今天写的代码用未来的 nightly 可能无法编译通过，因此我们会使用 rust-toolchain 文件锁定 nightly 版本。

## 添加编译依赖

- 增加RISC-V三元组:

    ```
    rustup target add riscv64imac-unknown-none-elf
    ```

- 增加需要的 cargo-binutils:

    ```
    cargo install cargo-binutils
    rustup component add llvm-tools-preview
    ```

