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
fn expect(value: bool) requires value, { if !value { panic!(); } }

#[cfg_attr(not(verus_keep_ghost), test)]
fn environment_errors_precede_trace_creation() {
    proof {
        reveal_strlit("declared");
        reveal_strlit("");
        reveal_strlit("wrong");
    }
    expect(policy::environment_exit("", 0).unwrap_or(0) == 81);
    expect(policy::environment_exit("wrong", 8).unwrap_or(0) == 81);
    expect(policy::environment_exit("declared", 1).unwrap_or(0) == 82);
    expect(policy::environment_exit("declared", usize::MAX).unwrap_or(0) == 82);
    expect(policy::environment_exit("declared", 0).is_none());
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
