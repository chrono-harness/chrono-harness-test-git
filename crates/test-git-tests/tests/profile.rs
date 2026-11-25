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

#[test]
fn registered_environments_preserve_raw_inputs_and_first_error() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    for (profile, bound, ambient, trace_key, mode) in [
        (
            "event",
            "BOUND_EVENT_VALUE",
            "CHRONO_EVENT_AMBIENT",
            "EVENT_TRACE",
            "EVENT_GIT_MODE",
        ),
        (
            "scoped",
            "BOUND_SCOPED_VALUE",
            "SCOPED_AMBIENT",
            "SCOPED_TRACE",
            "SCOPED_GIT_MODE",
        ),
        (
            "registration",
            "BOUND_FACTS_VALUE",
            "CHRONO_AMBIENT_PROBE",
            "CHRONO_FACTS_TRACE",
            "FACTS_GIT_MODE",
        ),
    ] {
        let declared = Some(OsStr::new("declared"));
        let empty = Some(OsStr::new(""));
        let raw = Some(OsStr::from_bytes(&[255]));
        for (value, inherited, code) in [
            (None, None, 81),
            (empty, empty, 81),
            (Some(OsStr::new("wrong")), raw, 81),
            (raw, None, 81),
            (declared, raw, 82),
            (declared, None, 0),
            (declared, empty, 0),
        ] {
            let directory = tempfile::tempdir().unwrap();
            let trace = directory.path().join("git.trace");
            let mut child = Command::new(env!("CHRONO_TEST_GIT_PROFILE_BIN"));
            child
                .env_clear()
                .env("CHRONO_TEST_GIT_PROFILE", profile)
                .env(trace_key, &trace)
                .env(mode, "pass")
                .arg("--version");
            if let Some(v) = value {
                child.env(bound, v);
            }
            if let Some(v) = inherited {
                child.env(ambient, v);
            }
            let result = child.output().unwrap();
            assert_eq!(result.status.code(), Some(code), "{profile}");
            assert!(result.stderr.is_empty());
            if code == 0 {
                assert_eq!(result.stdout, b"git version fixture\n");
                assert_eq!(fs::read(trace).unwrap(), b"--version\n");
            } else {
                assert!(result.stdout.is_empty());
                assert!(!trace.exists());
            }
        }
    }
}
