//! Terminal Git profile responses consumed by runner process diagnostics.
#[cfg(verus_keep_ghost)]
use vstd::string::StringSliceAdditionalSpecFns;
use vstd::{contrib::exec_spec::ExecSpecEq, prelude::*};

#[cfg(verus_keep_ghost)]
#[path = "profile.rs"]
mod profile;
#[cfg(not(verus_keep_ghost))]
pub use crate::profile::{Bindings, environment_exit, profile_bindings};
#[cfg(verus_keep_ghost)]
pub use profile::{Bindings, environment_exit, profile_bindings};

verus! {

#[derive(Clone, Copy)]
pub struct Arguments {
    pub tree: bool,
    pub version: bool,
    pub probe: bool,
    pub fetch: bool,
    pub revision: bool,
}

fn matches_argument(bytes: &[u8], expected: &[u8], prefix: bool) -> (matched: bool)
    ensures matched == ((prefix || bytes.len() == expected.len()) && expected.len() <= bytes.len()
        && bytes@.take(expected.len() as int) == expected@),
        !prefix ==> matched == (bytes@ == expected@),
{
    if bytes.len() < expected.len() || (!prefix && bytes.len() != expected.len()) { return false; }
    let mut i = 0usize;
    while i < expected.len()
        invariant i <= expected.len() <= bytes.len(),
            forall|j:int| 0 <= j < i ==> bytes@[j] == expected@[j],
        decreases expected.len() - i,
    {
        if bytes[i] != expected[i] { return false; }
        i += 1;
    }
    proof {
        assert(bytes@.take(expected.len() as int) =~= expected@);
        if !prefix { assert(bytes@ =~= expected@); }
    }
    true
}

pub fn observe_arguments(args: &[&[u8]]) -> (flags: Arguments)
    ensures flags.version == (args.len() > 0 && args@[0]@ == "--version".spec_bytes()),
        flags.tree == (exists|i:int| #![auto] 0 <= i < args.len() && args@[i]@ == "ls-tree".spec_bytes()),
        flags.fetch == (exists|i:int| #![auto] 0 <= i < args.len() && args@[i]@ == "fetch".spec_bytes()),
        flags.revision == (exists|i:int| #![auto] 0 <= i < args.len() && args@[i]@ == "rev-parse".spec_bytes()),
        flags.probe == (exists|i:int| #![auto] 0 <= i < args.len() && args@[i].len() >= "--batch-check".spec_bytes().len()
            && args@[i]@.take("--batch-check".spec_bytes().len() as int) == "--batch-check".spec_bytes()),
{
    let mut flags = Arguments {
        version: args.len() > 0 && matches_argument(args[0], "--version".as_bytes(), false),
        tree: false, fetch: false, revision: false, probe: false,
    };
    let mut i = 0usize;
    while i < args.len()
        invariant i <= args.len(),
            flags.version == (args.len() > 0 && args@[0]@ == "--version".spec_bytes()),
            flags.tree == (exists|j:int| #![auto] 0 <= j < i && args@[j]@ == "ls-tree".spec_bytes()),
            flags.fetch == (exists|j:int| #![auto] 0 <= j < i && args@[j]@ == "fetch".spec_bytes()),
            flags.revision == (exists|j:int| #![auto] 0 <= j < i && args@[j]@ == "rev-parse".spec_bytes()),
            flags.probe == (exists|j:int| #![auto] 0 <= j < i && args@[j].len() >= "--batch-check".spec_bytes().len()
                && args@[j]@.take("--batch-check".spec_bytes().len() as int) == "--batch-check".spec_bytes()),
        decreases args.len() - i,
    {
        let bytes = args[i];
        flags.tree = flags.tree || matches_argument(bytes, "ls-tree".as_bytes(), false);
        flags.fetch = flags.fetch || matches_argument(bytes, "fetch".as_bytes(), false);
        flags.revision = flags.revision || matches_argument(bytes, "rev-parse".as_bytes(), false);
        flags.probe = flags.probe || matches_argument(bytes, "--batch-check".as_bytes(), true);
        i += 1;
    }
    flags
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Delegate, VersionTimeout { prefix: bool, seconds: u64 }, ReadTimeout,
    BoundedOutput, ContinuousOutput, ReadDrift, MalformedProbe, ProbeReply { extra: bool }, Unknown,
}

pub open spec fn selected_action(mode: Seq<char>, registration: bool, args: Arguments) -> Action {
    use Action::*;
    if mode == "pass"@ { Delegate }
    else if mode == "version-timeout"@ { if args.version { VersionTimeout { prefix: registration, seconds: if registration { 10 } else { 60 } } } else { Delegate } }
    else if mode == "read-timeout"@ { if args.tree { ReadTimeout } else { Delegate } }
    else if mode == "read-limit"@ { if !args.tree { Delegate } else if registration { BoundedOutput } else { ContinuousOutput } }
    else if mode == "read-drift"@ { if args.revision { ReadDrift } else { Delegate } }
    else if mode == "malformed-probe"@ { if args.probe { MalformedProbe } else { Delegate } }
    else if mode == "wrong-type-probe"@ { if args.probe { ProbeReply { extra: false } } else { Delegate } }
    else if mode == "extra-probe-output"@ { if args.probe { ProbeReply { extra: true } } else { Delegate } }
    else if mode == "read-failure"@
        || mode == "version-exit"@
        || mode == "probe-failure"@
        || mode == "full-probe-failure"@
        || mode == "full-fetch-failure"@
        || mode == "scope-probe-failure"@ { Delegate }
    else { Unknown }
}

pub fn nonterminal_action(mode: &str, registration: bool, args: Arguments) -> (action: Action)
    ensures action == selected_action(mode@, registration, args),
{
    use Action::*;
    if <&str>::exec_eq(mode, "pass") { Delegate }
    else if <&str>::exec_eq(mode, "version-timeout") { if args.version { VersionTimeout { prefix: registration, seconds: if registration { 10 } else { 60 } } } else { Delegate } }
    else if <&str>::exec_eq(mode, "read-timeout") { if args.tree { ReadTimeout } else { Delegate } }
    else if <&str>::exec_eq(mode, "read-limit") { if !args.tree { Delegate } else if registration { BoundedOutput } else { ContinuousOutput } }
    else if <&str>::exec_eq(mode, "read-drift") { if args.revision { ReadDrift } else { Delegate } }
    else if <&str>::exec_eq(mode, "malformed-probe") { if args.probe { MalformedProbe } else { Delegate } }
    else if <&str>::exec_eq(mode, "wrong-type-probe") { if args.probe { ProbeReply { extra: false } } else { Delegate } }
    else if <&str>::exec_eq(mode, "extra-probe-output") { if args.probe { ProbeReply { extra: true } } else { Delegate } }
    else if <&str>::exec_eq(mode, "read-failure")
        || <&str>::exec_eq(mode, "version-exit")
        || <&str>::exec_eq(mode, "probe-failure")
        || <&str>::exec_eq(mode, "full-probe-failure")
        || <&str>::exec_eq(mode, "full-fetch-failure")
        || <&str>::exec_eq(mode, "scope-probe-failure") { Delegate }
    else { Unknown }
}

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
