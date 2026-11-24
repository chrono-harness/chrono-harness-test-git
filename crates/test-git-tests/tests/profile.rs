use std::{fs, process::Command};

#[test]
fn registration_tree_failure_preserves_bytes_exit_and_arguments() {
    let dir = tempfile::tempdir().unwrap();
    let trace = dir.path().join("git.trace");
    let result = Command::new(env!("CHRONO_TEST_GIT_PROFILE_BIN"))
        .env_clear()
        .env("CHRONO_TEST_GIT_PROFILE", "registration")
        .env("BOUND_FACTS_VALUE", "declared")
        .env("FACTS_GIT_MODE", "read-failure")
        .env("CHRONO_FACTS_TRACE", &trace)
        .args(["ls-tree", "-z", "HEAD"])
        .output()
        .unwrap();

    assert_eq!(result.status.code(), Some(17));
    assert_eq!(result.stdout, b"a\0b");
    assert_eq!(result.stderr, b"selected-facts-failure");
    assert_eq!(fs::read(trace).unwrap(), b"ls-tree -z HEAD\n");
}
