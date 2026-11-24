# chrono-harness-test-git

Rust 测试使用的 Git 命令替身。共享的模型覆盖工作树、索引、对象和引用的测试场景。可通过库的 `run` 接口提供指定的参数及输入，也可将二进制放在测试子进程独占的 PATH 中。

源码与边界用例来自 [runner 测试的固定提交](https://github.com/chrono-harness/chrono-harness-runner/tree/5c9364711a47ab45242195f11ca350e5d2b858cf/crates/runner-tests/tests)。运行 `mise trust` 后，`./check.sh` 按本次 Git 变更执行登记的检查。修改 Git 替身会选中命令匹配及独立预期的 Verus 证明和原生用例，涵盖命令分派、工作树状态及 registration 的错误输出。

从本仓发行下载对应平台的 `chrono-test-git`、`chrono-test-git-profile` 和 `SHA256SUMS`。按资产文件名核对 SHA-256 后，将需要的可执行文件安装在工作树外。升级和回退使用对应标签的发行文件。

`chrono-test-git` 在测试进程私有的 PATH 中充当 Git，接受 Git 参数、标准输入并维护测试仓库。用 `--version` 可读到 `git version fixture`。`chrono-test-git-profile` 供需要追踪参数和注入错误的测试使用。用 `CHRONO_TEST_GIT_PROFILE` 选择 `registration`、`event` 或 `scoped`，相应的环境变量与退出行为见 [profile 实现](crates/test-git/src/profile.rs)。

终止错误的证明根覆盖 read、version、probe 和 fetch 的适用条件，以及退出码、原始输出、错误前缀和是否读完 stdin。原生适配器执行实际 IO；runner 的登记 Git 用例直接读取子进程结果。超时、输出上限和文件替换另由相应行为用例验证。
