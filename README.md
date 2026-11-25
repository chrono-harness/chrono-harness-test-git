# chrono-harness-test-git

Rust 测试使用的 Git 命令替身。共享的模型覆盖工作树、索引、对象和引用的测试场景。可通过库的 `run` 接口提供指定的参数及输入，也可将二进制放在测试子进程独占的 PATH 中。

源码与边界用例来自 [runner 测试的固定提交](https://github.com/chrono-harness/chrono-harness-runner/tree/5c9364711a47ab45242195f11ca350e5d2b858cf/crates/runner-tests/tests)。运行 `mise trust` 后，`./check.sh` 按本次 Git 变更执行登记的检查。修改 Git 替身会选中命令匹配及独立预期的 Verus 证明和原生用例，涵盖命令分派、工作树状态及 registration 的错误输出。

从本仓发行下载对应平台的 `chrono-test-git`、`chrono-test-git-profile` 和 `SHA256SUMS`。按资产文件名核对 SHA-256 后，将需要的可执行文件安装在工作树外。升级和回退使用对应标签的发行文件。

`chrono-test-git` 在测试进程私有的 PATH 中充当 Git，接受 Git 参数、标准输入并维护测试仓库。用 `--version` 可读到 `git version fixture`。`chrono-test-git-profile` 供需要追踪参数和注入错误的测试使用。用 `CHRONO_TEST_GIT_PROFILE` 选择 `registration`、`event` 或 `scoped`，相应的环境变量与退出行为见 [profile 实现](crates/test-git/src/profile.rs)。

终止错误的证明根覆盖 read、version、probe 和 fetch 的适用条件，以及退出码、原始输出、错误前缀和是否读完 stdin。原生适配器执行实际 IO；runner 的登记 Git 用例直接读取子进程结果。超时、输出上限和文件替换另由相应行为用例验证。

## 本地检查产物

本仓的单元和汇总检查在各自的业务目录发布原件。判官读回本次 Verus 编译库及依赖文件后，成功的操作会释放这两份文件。检查报告的保留窗口为 24 小时，需要长期保存的报告及其引用原件须在窗口内按相对路径转存，并在检出外读回。

开发机可将下列命令按实际安装路径加入用户 crontab，每小时调用本仓的模板维护任务。已到期的成功报告在读取和交付义务结束后释放，后续维护轮次回收已失去引用的原件。独立维护依赖机器在线及已安装的定时任务。

```cron
0 * * * * cd '/absolute/path/to/repository' && '/absolute/path/to/mise' run harness-maintain-checks >/dev/null
```
