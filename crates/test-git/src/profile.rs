use vstd::{contrib::exec_spec::ExecSpecEq, prelude::*};

verus! {
pub struct Bindings {
    pub bound: &'static str,
    pub ambient: &'static str,
    pub trace: &'static str,
    pub mode: &'static str,
    pub registration: bool,
}

pub open spec fn uses_bindings(b: Bindings, bound: Seq<char>, ambient: Seq<char>,
                               trace: Seq<char>, mode: Seq<char>, registration: bool) -> bool {
    b.bound@ == bound && b.ambient@ == ambient && b.trace@ == trace
        && b.mode@ == mode && b.registration == registration
}

pub fn profile_bindings(as_git: bool, name: &str) -> (choice: Result<Bindings, i32>)
    ensures
        choice.is_ok() == (!as_git && (name@ == "event"@ || name@ == "scoped"@ || name@ == "registration"@)),
        match choice {
            Err(code) => code == if as_git { 83i32 } else { 84i32 },
            Ok(b) => if name@ == "event"@ {
                uses_bindings(b, "BOUND_EVENT_VALUE"@, "CHRONO_EVENT_AMBIENT"@, "EVENT_TRACE"@, "EVENT_GIT_MODE"@, false)
            } else if name@ == "scoped"@ {
                uses_bindings(b, "BOUND_SCOPED_VALUE"@, "SCOPED_AMBIENT"@, "SCOPED_TRACE"@, "SCOPED_GIT_MODE"@, false)
            } else if name@ == "registration"@ {
                uses_bindings(b, "BOUND_FACTS_VALUE"@, "CHRONO_AMBIENT_PROBE"@, "CHRONO_FACTS_TRACE"@, "FACTS_GIT_MODE"@, true)
            } else { false },
        },
{
    if as_git {
        Err(83)
    } else if <&str>::exec_eq(name, "event") {
        Ok(Bindings { bound: "BOUND_EVENT_VALUE", ambient: "CHRONO_EVENT_AMBIENT", trace: "EVENT_TRACE", mode: "EVENT_GIT_MODE", registration: false })
    } else if <&str>::exec_eq(name, "scoped") {
        Ok(Bindings { bound: "BOUND_SCOPED_VALUE", ambient: "SCOPED_AMBIENT", trace: "SCOPED_TRACE", mode: "SCOPED_GIT_MODE", registration: false })
    } else if <&str>::exec_eq(name, "registration") {
        Ok(Bindings { bound: "BOUND_FACTS_VALUE", ambient: "CHRONO_AMBIENT_PROBE", trace: "CHRONO_FACTS_TRACE", mode: "FACTS_GIT_MODE", registration: true })
    } else {
        Err(84)
    }
}

/// Admission precedes trace creation in each registered Git profile.
pub fn environment_exit(bound: &str, ambient_bytes: usize) -> (exit: Option<i32>)
    ensures exit == if bound@ != "declared"@ { Some(81i32) }
                    else if ambient_bytes != 0 { Some(82i32) }
                    else { None },
{
    if !<&str>::exec_eq(bound, "declared") {
        Some(81)
    } else if ambient_bytes > 0 {
        Some(82)
    } else {
        None
    }
}
}

#[cfg(not(verus_keep_ghost))]
use std::{
    env,
    ffi::OsStr,
    fs::OpenOptions,
    io::{self, BufRead, Write},
    os::unix::ffi::OsStrExt,
    path::Path,
    process::{self},
};

#[cfg(not(verus_keep_ghost))]
fn failure(bytes: &[u8], code: i32) -> ! {
    io::stderr().write_all(bytes).unwrap();
    process::exit(code);
}

#[cfg(not(verus_keep_ghost))]
pub fn main() -> io::Result<()> {
    let mut arguments = env::args_os();
    let program = arguments.next().unwrap();
    let as_git = Path::new(&program).file_name() == Some(OsStr::new("git"));
    let requested = env::var("CHRONO_TEST_GIT_PROFILE").unwrap_or_default();
    let bindings = match profile_bindings(as_git, &requested) {
        Ok(bindings) => bindings,
        Err(83) => failure(b"ambient-git\n", 83),
        Err(code) => failure(b"unknown registered Git fixture profile\n", code),
    };
    let bound_value = env::var(bindings.bound).unwrap_or_default();
    let ambient_bytes = env::var_os(bindings.ambient).map_or(0, |value| value.as_bytes().len());
    if let Some(code) = environment_exit(&bound_value, ambient_bytes) {
        process::exit(code);
    }
    let arguments: Vec<_> = arguments.collect();
    let mut trace = OpenOptions::new()
        .create(true)
        .append(true)
        .open(env::var_os(bindings.trace).expect("registered trace path"))?;
    for (index, argument) in arguments.iter().enumerate() {
        if index != 0 {
            trace.write_all(b" ")?;
        }
        trace.write_all(argument.as_bytes())?;
    }
    trace.write_all(b"\n")?;
    drop(trace);

    let mode = env::var(bindings.mode).expect("registered Git fixture mode");
    let probe = arguments
        .iter()
        .any(|argument| argument.as_bytes().starts_with(b"--batch-check"));
    let fetch = arguments.iter().any(|argument| argument == "fetch");
    let version = arguments
        .first()
        .is_some_and(|argument| argument == "--version");
    let tree = arguments.iter().any(|argument| argument == "ls-tree");
    let registration = bindings.registration;
    if let Some(result) =
        crate::profile_failure::terminal_failure(&mode, registration, tree, version, probe, fetch)
    {
        if result.drain_stdin {
            io::copy(&mut io::stdin(), &mut io::sink())?;
        }
        if !result.stdout.is_empty() {
            io::stdout().write_all(result.stdout.as_bytes())?;
            io::stdout().flush()?;
        }
        let mut stderr = io::stderr().lock();
        if let Some(byte) = result.stderr_prefix {
            stderr.write_all(&[byte])?;
        }
        stderr.write_all(result.stderr.as_bytes())?;
        process::exit(result.code);
    }
    match mode.as_str() {
        "pass" => {}
        "version-timeout" if version => {
            if registration {
                io::stdout().write_all(b"before-timeout")?;
                io::stdout().flush()?;
            }
            std::thread::sleep(std::time::Duration::from_secs(if registration {
                10
            } else {
                60
            }));
            return Ok(());
        }
        "read-timeout" if tree => {
            io::stdout().write_all(b"before-timeout")?;
            io::stdout().flush()?;
            std::thread::sleep(std::time::Duration::from_secs(60));
        }
        "read-limit" if tree && registration => {
            for _ in 0..400 {
                io::stdout().write_all(b"0123456789012345678901234567890123456789")?;
            }
            io::stdout().flush()?;
        }
        "read-limit" if tree => loop {
            io::stdout().write_all(b"0123456789abcdef")?;
        },
        "read-drift" if arguments.iter().any(|argument| argument == "rev-parse") => {
            let output = crate::run(
                env::current_dir()?,
                &arguments
                    .iter()
                    .map(|a| a.to_str().unwrap().to_owned())
                    .collect::<Vec<_>>(),
                &[],
            );
            if let Ok(bytes) = &output {
                io::stdout().write_all(bytes)?;
            }
            if output.is_ok() {
                // Replace this test's private copied image after Git exits.
                // Reuse the transport fixture's permission-preserving rename
                // method on platforms that forbid writing an executing image.
                let executable = env::current_exe()?;
                let mut bytes = std::fs::read(&executable)?;
                bytes.extend_from_slice(b"\n# persistent drift\n");
                let replacement = executable.with_extension("replacement");
                std::fs::write(&replacement, bytes)?;
                std::fs::set_permissions(
                    &replacement,
                    std::fs::metadata(&executable)?.permissions(),
                )?;
                std::fs::rename(replacement, executable)?;
            }
            if let Err(error) = output {
                failure(error.as_bytes(), 93);
            }
            return Ok(());
        }
        "malformed-probe" if probe => {
            io::copy(&mut io::stdin(), &mut io::sink())?;
            io::stdout().write_all(b"not-the-request missing\n")?;
            return Ok(());
        }
        "wrong-type-probe" | "extra-probe-output" if probe => {
            let mut oid = Vec::new();
            io::stdin().lock().read_until(b'\n', &mut oid)?;
            if oid.last() == Some(&b'\n') {
                oid.pop();
            }
            io::stdout().write_all(&oid)?;
            io::stdout().write_all(if mode == "wrong-type-probe" {
                b" blob\n"
            } else {
                b" commit\nextra\n"
            })?;
            return Ok(());
        }
        "probe-failure"
        | "malformed-probe"
        | "wrong-type-probe"
        | "extra-probe-output"
        | "full-probe-failure"
        | "full-fetch-failure"
        | "scope-probe-failure"
        | "read-failure"
        | "read-drift"
        | "read-timeout"
        | "version-exit"
        | "version-timeout"
        | "read-limit" => {}
        _ => failure(b"unknown registered Git fixture mode\n", 84),
    }
    crate::main();
    Ok(())
}
