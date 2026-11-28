#![allow(unexpected_cfgs)]
#[cfg(verus_keep_ghost)]
#[path = "../../test-git/src/profile_failure.rs"]
mod policy;
#[cfg(not(verus_keep_ghost))]
use chrono_test_git::profile_failure as policy;
#[cfg(verus_keep_ghost)]
use vstd::vpanic as panic;
use vstd::{contrib::exec_spec::ExecSpecEq, prelude::*};

verus! {

/// The deployed check is an input of this proof and its native consumer.
pub fn native_check_script() -> &'static str {
    include_str!("../../../check.sh")
}
fn expect(value: bool) requires value, { if !value { panic!(); } }

#[cfg_attr(not(verus_keep_ghost), test)]
fn profile_command_observations_use_original_argument_bytes() {
    let args = ["ls-tree".as_bytes(), "--version".as_bytes(), "--batch-check".as_bytes(),
        "fetch".as_bytes(), "rev-parse".as_bytes(), &[255u8]];
    let observed = policy::observe_arguments(&args);
    proof {
        use vstd::string::StringSliceAdditionalSpecFns;
        reveal_strlit("--version"); reveal_strlit("ls-tree");
        assert(args[0]@ == "ls-tree".spec_bytes());
        assert(args[2]@.take(args[2].len() as int) =~= "--batch-check".spec_bytes());
        assert(args[3]@ == "fetch".spec_bytes());
        assert(args[4]@ == "rev-parse".spec_bytes());
    }
    expect(observed.tree && observed.fetch && observed.probe && observed.revision);
    let empty = policy::observe_arguments(&[]);
    expect(!empty.version && !empty.tree && !empty.fetch && !empty.probe && !empty.revision);
    let version = policy::observe_arguments(&["--version".as_bytes()]);
    expect(version.version);
}

#[cfg_attr(not(verus_keep_ghost), test)]
fn nonterminal_modes_select_their_registered_action_or_delegate() {
    use policy::Action::*;
    let on = policy::Arguments { version: true, tree: true, fetch: true, probe: true, revision: true };
    let off = policy::Arguments { version: false, tree: false, fetch: false, probe: false, revision: false };
    proof {
        reveal_strlit("pass");
        reveal_strlit("version-timeout");
        reveal_strlit("read-timeout");
        reveal_strlit("read-limit");
        reveal_strlit("read-drift");
        reveal_strlit("malformed-probe");
        reveal_strlit("wrong-type-probe");
        reveal_strlit("extra-probe-output");
        reveal_strlit("read-failure");
        reveal_strlit("version-exit");
        reveal_strlit("probe-failure");
        reveal_strlit("full-probe-failure");
        reveal_strlit("full-fetch-failure");
        reveal_strlit("scope-probe-failure");
        reveal_strlit("unknown");
        assert("version-timeout"@[0] != "malformed-probe"@[0]);
        assert("read-timeout"@[5] != "read-failure"@[5]);
        assert("read-timeout"@[0] != "version-exit"@[0]);
        assert("read-limit"@[5] != "read-drift"@[5]);
        assert("extra-probe-output"@[0] != "full-probe-failure"@[0]);
        assert("extra-probe-output"@[0] != "full-fetch-failure"@[0]);
        assert("read-failure"@[0] != "version-exit"@[0]);
        assert("full-probe-failure"@[5] != "full-fetch-failure"@[5]);
        assert("pass"@.len() == 4);
        assert("version-timeout"@.len() == 15);
        assert("read-timeout"@.len() == 12);
        assert("read-limit"@.len() == 10);
        assert("read-drift"@.len() == 10);
        assert("malformed-probe"@.len() == 15);
        assert("wrong-type-probe"@.len() == 16);
        assert("extra-probe-output"@.len() == 18);
        assert("read-failure"@.len() == 12);
        assert("version-exit"@.len() == 12);
        assert("probe-failure"@.len() == 13);
        assert("full-probe-failure"@.len() == 18);
        assert("full-fetch-failure"@.len() == 18);
        assert("scope-probe-failure"@.len() == 19);
        assert("unknown"@.len() == 7);
    }
    expect(matches!(policy::nonterminal_action("pass", false, on), Delegate));
    expect(matches!(policy::nonterminal_action("version-timeout", true, on), VersionTimeout { prefix: true, seconds: 10 }));
    expect(matches!(policy::nonterminal_action("version-timeout", false, on), VersionTimeout { prefix: false, seconds: 60 }));
    expect(matches!(policy::nonterminal_action("read-timeout", false, on), ReadTimeout));
    expect(matches!(policy::nonterminal_action("read-limit", true, on), BoundedOutput));
    expect(matches!(policy::nonterminal_action("read-limit", false, on), ContinuousOutput));
    expect(matches!(policy::nonterminal_action("read-drift", false, on), ReadDrift));
    expect(matches!(policy::nonterminal_action("malformed-probe", false, on), MalformedProbe));
    expect(matches!(policy::nonterminal_action("wrong-type-probe", true, on), ProbeReply { extra: false }));
    expect(matches!(policy::nonterminal_action("extra-probe-output", true, on), ProbeReply { extra: true }));
    expect(matches!(policy::nonterminal_action("read-limit", true, off), Delegate));
    expect(matches!(policy::nonterminal_action("full-fetch-failure", true, off), Delegate));
    expect(matches!(policy::nonterminal_action("unknown", true, on), Unknown));
}

#[cfg_attr(not(verus_keep_ghost), test)]
fn environment_errors_precede_trace_creation() {
    proof {
        reveal_strlit("declared");
        reveal_strlit("");
        reveal_strlit("wrong");
        assert(""@.len() != "declared"@.len());
        assert("wrong"@.len() != "declared"@.len());
    }
    expect(policy::environment_exit("", 0).unwrap_or(0) == 81);
    expect(policy::environment_exit("wrong", 8).unwrap_or(0) == 81);
    expect(policy::environment_exit("declared", 1).unwrap_or(0) == 82);
    expect(policy::environment_exit("declared", usize::MAX).unwrap_or(0) == 82);
    expect(policy::environment_exit("declared", 0).is_none());
}

fn check_binding(choice: Result<policy::Bindings, i32>, bound: &str, ambient: &str,
                 trace: &str, mode: &str, registration: bool)
    requires match choice {
        Ok(b) => b.bound@ == bound@ && b.ambient@ == ambient@ && b.trace@ == trace@
            && b.mode@ == mode@ && b.registration == registration,
        Err(_) => false,
    },
{
    match choice {
        Ok(b) => {
            expect(<&str>::exec_eq(b.bound, bound));
            expect(<&str>::exec_eq(b.ambient, ambient));
            expect(<&str>::exec_eq(b.trace, trace));
            expect(<&str>::exec_eq(b.mode, mode));
            expect(b.registration == registration);
        },
        Err(_) => expect(false),
    }
}

#[cfg_attr(not(verus_keep_ghost), test)]
fn profile_selects_registered_environment_or_first_error() {
    proof {
        reveal_strlit("event");
        reveal_strlit("scoped");
        reveal_strlit("registration");
        reveal_strlit("");
        reveal_strlit("other");
        assert("other"@[0] != "event"@[0]);
        assert("other"@ != "event"@);
        assert("event"@.len() == 5 && "scoped"@.len() == 6 && "registration"@.len() == 12);
        assert(""@.len() == 0 && "other"@.len() == 5);
        assert("event"@ != "scoped"@ && "event"@ != "registration"@ && "scoped"@ != "registration"@);
        assert(""@ != "event"@ && ""@ != "scoped"@ && ""@ != "registration"@);
        assert("other"@ != "scoped"@ && "other"@ != "registration"@);
    }
    check_binding(policy::profile_bindings(false, "event"),
        "BOUND_EVENT_VALUE", "CHRONO_EVENT_AMBIENT", "EVENT_TRACE", "EVENT_GIT_MODE", false);
    check_binding(policy::profile_bindings(false, "scoped"),
        "BOUND_SCOPED_VALUE", "SCOPED_AMBIENT", "SCOPED_TRACE", "SCOPED_GIT_MODE", false);
    check_binding(policy::profile_bindings(false, "registration"),
        "BOUND_FACTS_VALUE", "CHRONO_AMBIENT_PROBE", "CHRONO_FACTS_TRACE", "FACTS_GIT_MODE", true);
    expect(policy::profile_bindings(true, "event").err().unwrap_or(0) == 83);
    expect(policy::profile_bindings(true, "").err().unwrap_or(0) == 83);
    expect(policy::profile_bindings(false, "").err().unwrap_or(0) == 84);
    expect(policy::profile_bindings(false, "other").err().unwrap_or(0) == 84);
}

fn exact(result: Option<policy::Failure>, code: i32, stdout: &str, prefix: Option<u8>, stderr: &str, drain: bool)
    requires match result { Some(r) => policy::describes(r, code as int, stdout@, prefix, stderr@, drain), None => false },
{
    match result {
        Some(r) => {
            expect(r.code == code);
            expect(<&str>::exec_eq(r.stdout, stdout));
            let same_prefix = match (r.stderr_prefix, prefix) {
                (Some(a), Some(b)) => a == b, (None, None) => true, _ => false,
            };
            expect(same_prefix);
            expect(<&str>::exec_eq(r.stderr, stderr));
            expect(r.drain_stdin == drain);
        },
        None => expect(false),
    }
}

#[cfg_attr(not(verus_keep_ghost), test)]
fn profile_terminal_failures_preserve_process_responses_and_delegation() {
    proof {
        reveal_strlit("full-probe-failure"); reveal_strlit("scope-probe-failure");
        reveal_strlit("probe-failure"); reveal_strlit("full-fetch-failure");
        reveal_strlit("read-failure"); reveal_strlit("version-exit"); reveal_strlit("pass");
        assert("pass"@.len() != "read-failure"@.len() &&
            "pass"@.len() != "version-exit"@.len() &&
            "pass"@.len() != "probe-failure"@.len() &&
            "pass"@.len() != "full-probe-failure"@.len() &&
            "pass"@.len() != "scope-probe-failure"@.len() &&
            "pass"@.len() != "full-fetch-failure"@.len());
        assert("full-probe-failure"@.len() != "probe-failure"@.len());
        assert("scope-probe-failure"@.len() != "probe-failure"@.len());
        assert("scope-probe-failure"@.len() != "full-probe-failure"@.len());
    }
    exact(policy::terminal_failure("read-failure", true, true, false, false, false), 17, "a\0b", None, "selected-facts-failure", false);
    exact(policy::terminal_failure("read-failure", false, true, false, false, false), 17, "a\0b", None, "chosen-failure", false);
    exact(policy::terminal_failure("version-exit", false, false, true, false, false), 19, "a\0b", None, "version-failed", false);
    exact(policy::terminal_failure("probe-failure", false, false, false, true, false), 73, "", Some(255), "original-probe\n", true);
    exact(policy::terminal_failure("full-probe-failure", false, false, false, true, false), 29, "", Some(255), "broken", false);
    exact(policy::terminal_failure("full-fetch-failure", false, false, false, false, true), 31, "", Some(254), "fetch-failure", false);
    exact(policy::terminal_failure("scope-probe-failure", false, false, false, true, false), 29, "", Some(255), "scope-probe-failure", false);
    expect(policy::terminal_failure("pass", true, true, true, true, true).is_none());
    expect(policy::terminal_failure("read-failure", true, false, false, false, false).is_none());
    expect(policy::terminal_failure("full-fetch-failure", false, false, false, false, false).is_none());
}
}

#[cfg(not(verus_keep_ghost))]
#[test]
fn registered_environments_preserve_raw_inputs_and_first_error() {
    use std::os::unix::ffi::OsStrExt;
    use std::{ffi::OsStr, fs, process::Command};

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

#[cfg(not(verus_keep_ghost))]
#[test]
fn native_check_matches_the_compiled_host_input() {
    let deployed = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../check.sh");
    assert_eq!(
        std::fs::read(&deployed)
            .unwrap_or_else(|error| panic!("cannot read {}: {error}", deployed.display())),
        native_check_script().as_bytes()
    );
}
