mod git_status_io {
    use std::{fs, process::Command};

    #[test]
    fn selected_git_mock_status_skips_ignored_subtrees_but_checks_tracked_files() {
        use std::os::unix::fs::PermissionsExt;

        let directory = tempfile::tempdir().unwrap();
        let root = directory.path();
        let call = |args: &[&str]| {
            Command::new(env!("CARGO_BIN_EXE_chrono-test-git"))
                .current_dir(root)
                .args(args)
                .output()
                .unwrap()
        };
        let ok = |args: &[&str]| {
            let output = call(args);
            assert!(output.status.success(), "{args:?}: {output:?}");
            assert!(output.stderr.is_empty(), "{args:?}: {output:?}");
            output.stdout
        };
        ok(&["init", "-q", "-b", "dev"]);
        fs::write(root.join(".gitignore"), "ignored/\n").unwrap();
        fs::write(root.join(".git/info/exclude"), "excluded/\n").unwrap();
        fs::create_dir(root.join("ignored")).unwrap();
        fs::write(root.join("ignored/tracked"), b"before").unwrap();
        ok(&["add", ".gitignore"]);
        ok(&["add", "-f", "ignored/tracked"]);
        ok(&["commit", "-qm", "base"]);
        fs::write(root.join("ignored/tracked"), b"after").unwrap();
        fs::write(root.join("visible"), b"new").unwrap();
        fs::create_dir(root.join("excluded")).unwrap();
        for subtree in ["ignored", "excluded"] {
            let private = root.join(subtree).join("private");
            fs::create_dir(&private).unwrap();
            fs::write(private.join("payload"), b"private").unwrap();
            fs::set_permissions(private, fs::Permissions::from_mode(0o000)).unwrap();
        }
        let ordinary = call(&["status", "--porcelain", "-z"]);
        for subtree in ["ignored", "excluded"] {
            fs::set_permissions(
                root.join(subtree).join("private"),
                fs::Permissions::from_mode(0o700),
            )
            .unwrap();
        }
        assert!(ordinary.status.success(), "{ordinary:?}");
        assert!(ordinary.stderr.is_empty(), "{ordinary:?}");
        assert_eq!(ordinary.stdout, b" M ignored/tracked\0?? visible\0");
        fs::remove_file(root.join("ignored/tracked")).unwrap();
        assert_eq!(
            ok(&["status", "--porcelain", "-z"]),
            b" D ignored/tracked\0?? visible\0"
        );
        fs::create_dir(root.join("ignored/tracked")).unwrap();
        assert_eq!(
            ok(&["status", "--porcelain", "-z"]),
            b" D ignored/tracked\0?? visible\0"
        );
        fs::remove_dir(root.join("ignored/tracked")).unwrap();
        fs::write(root.join("ignored/tracked"), b"after").unwrap();
        let output = ok(&["status", "--porcelain", "--ignored", "-z"]);
        let mut records: Vec<_> = output.split(|byte| *byte == 0).collect();
        records.sort();
        assert_eq!(
            records,
            [
                b"".as_slice(),
                b" M ignored/tracked",
                b"!! excluded/private/payload",
                b"!! ignored/private/payload",
                b"?? visible",
            ]
        );
    }

    #[test]
    fn selected_git_mock_status_keeps_visible_and_explicit_ignored_io_errors() {
        use std::os::unix::fs::PermissionsExt;

        let directory = tempfile::tempdir().unwrap();
        let root = directory.path();
        let call = |args: &[&str]| {
            Command::new(env!("CARGO_BIN_EXE_chrono-test-git"))
                .current_dir(root)
                .args(args)
                .output()
                .unwrap()
        };
        for args in [
            vec!["init", "-q", "-b", "dev"],
            vec!["commit", "--allow-empty", "-qm", "base"],
        ] {
            let output = call(&args);
            assert!(output.status.success(), "{args:?}: {output:?}");
        }
        fs::write(root.join(".gitignore"), "blocked/\n").unwrap();
        let blocked = root.join("blocked");
        fs::create_dir(&blocked).unwrap();
        fs::write(blocked.join("payload"), b"private").unwrap();
        fs::set_permissions(&blocked, fs::Permissions::from_mode(0o000)).unwrap();
        let denial = fs::read_dir(&blocked).err();
        let ordinary = call(&["status", "--porcelain", "-z"]);
        let explicit = call(&["status", "--porcelain", "--ignored", "-z"]);
        fs::write(root.join(".gitignore"), "").unwrap();
        let visible = call(&["status", "--porcelain", "-z"]);
        // Restore access before inspecting results so failed assertions retain a
        // removable private fixture. Privileged hosts cannot exercise this denial.
        fs::set_permissions(&blocked, fs::Permissions::from_mode(0o700)).unwrap();
        let Some(denial) = denial else {
            eprintln!("permission denial is unavailable for this process identity");
            return;
        };
        assert!(ordinary.status.success(), "{ordinary:?}");
        assert_eq!(ordinary.stdout, b"?? .gitignore\0");
        assert!(ordinary.stderr.is_empty(), "{ordinary:?}");
        for output in [explicit, visible] {
            assert_eq!(output.status.code(), Some(93), "{output:?}");
            assert!(output.stdout.is_empty(), "{output:?}");
            let error = String::from_utf8(output.stderr).unwrap();
            assert!(error.contains(blocked.to_str().unwrap()), "{error}");
            assert!(error.contains(&denial.to_string()), "{error}");
        }
        fs::write(root.join(".gitignore"), "blocked/\n").unwrap();
        for args in [
            vec!["add", "-f", "blocked/payload"],
            vec!["commit", "-qm", "tracked ignored payload"],
        ] {
            let output = call(&args);
            assert!(output.status.success(), "{args:?}: {output:?}");
        }
        fs::set_permissions(&blocked, fs::Permissions::from_mode(0o000)).unwrap();
        let tracked = call(&["status", "--porcelain", "-z"]);
        fs::set_permissions(&blocked, fs::Permissions::from_mode(0o700)).unwrap();
        assert_eq!(tracked.status.code(), Some(93), "{tracked:?}");
        assert!(tracked.stdout.is_empty(), "{tracked:?}");
        let error = String::from_utf8(tracked.stderr).unwrap();
        assert!(error.contains("blocked/payload"), "{error}");
        assert!(error.contains(&denial.to_string()), "{error}");
    }
}
