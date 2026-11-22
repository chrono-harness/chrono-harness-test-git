# chrono-harness-test-git

Rust 测试使用的 Git 命令替身。共享的模型覆盖工作树、索引、对象和引用的测试场景。可通过库的 `run` 接口提供指定的参数及输入，也可将二进制放在测试子进程独占的 PATH 中。

源码与边界用例来自 [runner 测试的固定提交](https://github.com/chrono-harness/chrono-harness-runner/tree/5c9364711a47ab45242195f11ca350e5d2b858cf/crates/runner-tests/tests)。使用 `VERUS=/path/to/verus ./check.sh` 检查。
