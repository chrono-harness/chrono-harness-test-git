//! Terminal Git profile responses consumed by runner process diagnostics.
use vstd::{contrib::exec_spec::ExecSpecEq, prelude::*};

#[cfg(verus_keep_ghost)]
#[path = "profile.rs"]
mod profile;
#[cfg(not(verus_keep_ghost))]
pub use crate::profile::{Bindings, environment_exit, profile_bindings};
#[cfg(verus_keep_ghost)]
pub use profile::{Bindings, environment_exit, profile_bindings};

verus! {

pub struct Failure {
    pub code: i32,
    pub stdout: &'static str,
    pub stderr_prefix: Option<u8>,
    pub stderr: &'static str,
    pub drain_stdin: bool,
}

pub open spec fn describes(r: Failure, code: int, stdout: Seq<char>, prefix: Option<u8>, stderr: Seq<char>, drain: bool) -> bool {
    r.code as int == code && r.stdout@ == stdout && r.stderr_prefix == prefix
        && r.stderr@ == stderr && r.drain_stdin == drain
}

pub fn terminal_failure(mode: &str, registration: bool, tree: bool, version: bool, probe: bool, fetch: bool)
    -> (result: Option<Failure>)
    ensures
        (result.is_some() == ((mode@ == "read-failure"@ && tree)
            || (mode@ == "version-exit"@ && version)
            || ((mode@ == "probe-failure"@ || mode@ == "full-probe-failure"@ || mode@ == "scope-probe-failure"@) && probe)
            || (mode@ == "full-fetch-failure"@ && fetch))),
        match result {
            Some(r) => {
                if mode@ == "read-failure"@ && tree {
                    describes(r, 17, "a\0b"@, None, if registration { "selected-facts-failure"@ } else { "chosen-failure"@ }, false)
                } else if mode@ == "version-exit"@ && version { describes(r, 19, "a\0b"@, None, "version-failed"@, false) }
                else if mode@ == "probe-failure"@ && probe { describes(r, 73, ""@, Some(255u8), "original-probe\n"@, true) }
                else if mode@ == "full-probe-failure"@ && probe { describes(r, 29, ""@, Some(255u8), "broken"@, false) }
                else if mode@ == "full-fetch-failure"@ && fetch { describes(r, 31, ""@, Some(254u8), "fetch-failure"@, false) }
                else { describes(r, 29, ""@, Some(255u8), "scope-probe-failure"@, false) }
            },
            None => true,
        },
{
    let failure = if <&str>::exec_eq(mode, "read-failure") && tree {
        Failure { code: 17, stdout: "a\0b", stderr_prefix: None,
            stderr: if registration { "selected-facts-failure" } else { "chosen-failure" }, drain_stdin: false }
    } else if <&str>::exec_eq(mode, "version-exit") && version {
        Failure { code: 19, stdout: "a\0b", stderr_prefix: None, stderr: "version-failed", drain_stdin: false }
    } else if <&str>::exec_eq(mode, "probe-failure") && probe {
        Failure { code: 73, stdout: "", stderr_prefix: Some(255), stderr: "original-probe\n", drain_stdin: true }
    } else if <&str>::exec_eq(mode, "full-probe-failure") && probe {
        Failure { code: 29, stdout: "", stderr_prefix: Some(255), stderr: "broken", drain_stdin: false }
    } else if <&str>::exec_eq(mode, "full-fetch-failure") && fetch {
        Failure { code: 31, stdout: "", stderr_prefix: Some(254), stderr: "fetch-failure", drain_stdin: false }
    } else if <&str>::exec_eq(mode, "scope-probe-failure") && probe {
        Failure { code: 29, stdout: "", stderr_prefix: Some(255), stderr: "scope-probe-failure", drain_stdin: false }
    } else { return None; };
    Some(failure)
}

}
