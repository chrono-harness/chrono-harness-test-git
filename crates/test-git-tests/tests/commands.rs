#![allow(unexpected_cfgs)]
#[cfg(verus_keep_ghost)]
#[path = "../../test-git/src/lib.rs"]
mod git;
#[cfg(not(verus_keep_ghost))]
use chrono_test_git as git;
#[cfg(not(verus_keep_ghost))]
use vstd::prelude::verus;
#[cfg(verus_keep_ghost)]
use vstd::{prelude::*, vpanic as panic};

verus! {
fn expect(actual:bool) requires actual, { if !actual {panic!();} }
#[cfg_attr(not(verus_keep_ghost), test)]
fn ordinary_model_errors_preserve_the_callers_exit_status() {
    proof {
        reveal_strlit("fixture hook exit:"); vstd::string::is_ascii_spec_bytes("fixture hook exit:");
        reveal_strlit("fixture hook exit:42"); vstd::string::is_ascii_spec_bytes("fixture hook exit:42");
        reveal_strlit("fixture hook exit"); vstd::string::is_ascii_spec_bytes("fixture hook exit");
        reveal_strlit("fixture hook exitx42"); vstd::string::is_ascii_spec_bytes("fixture hook exitx42");
        reveal_strlit("fixture apply conflict");
        reveal_strlit("fixture remote lease rejected");
        reveal_strlit("missing fixture reference");
        reveal_strlit("not a fixture ancestor");
        reveal_strlit("not a symbolic fixture ref");
        reveal_strlit("missing remote fixture reference");
        reveal_strlit("fixture hook exit:oops");
        reveal_strlit("missing fixture reference ");
        reveal_strlit("unhandled");
        assert("fixture remote lease rejected"@.len() != "fixture apply conflict"@.len());
        assert("missing fixture reference"@.len() != "fixture apply conflict"@.len());
        assert("missing fixture reference"@.len() != "fixture remote lease rejected"@.len());
        assert("not a fixture ancestor"@[0] != "fixture apply conflict"@[0]);
        assert("not a fixture ancestor"@.len() != "fixture remote lease rejected"@.len());
        assert("not a fixture ancestor"@.len() != "missing fixture reference"@.len());
        assert("not a symbolic fixture ref"@.len() != "fixture apply conflict"@.len());
        assert("not a symbolic fixture ref"@.len() != "fixture remote lease rejected"@.len());
        assert("not a symbolic fixture ref"@.len() != "missing fixture reference"@.len());
        assert("not a symbolic fixture ref"@.len() != "not a fixture ancestor"@.len());
        assert("missing remote fixture reference"@.len() != "fixture apply conflict"@.len());
        assert("missing remote fixture reference"@.len() != "fixture remote lease rejected"@.len());
        assert("missing remote fixture reference"@.len() != "missing fixture reference"@.len());
        assert("missing remote fixture reference"@.len() != "not a fixture ancestor"@.len());
        assert("missing remote fixture reference"@.len() != "not a symbolic fixture ref"@.len());
        assert("fixture hook exit:oops"@[8] != "fixture apply conflict"@[8]);
        assert("fixture hook exit:oops"@.len() != "fixture remote lease rejected"@.len());
        assert("fixture hook exit:oops"@.len() != "missing fixture reference"@.len());
        assert("fixture hook exit:oops"@[0] != "not a fixture ancestor"@[0]);
        assert("fixture hook exit:oops"@.len() != "not a symbolic fixture ref"@.len());
        assert("fixture hook exit:oops"@.len() != "missing remote fixture reference"@.len());
        assert("missing fixture reference "@.len() != "fixture apply conflict"@.len());
        assert("missing fixture reference "@.len() != "fixture remote lease rejected"@.len());
        assert("missing fixture reference "@.len() != "missing fixture reference"@.len());
        assert("missing fixture reference "@.len() != "not a fixture ancestor"@.len());
        assert("missing fixture reference "@[0] != "not a symbolic fixture ref"@[0]);
        assert("missing fixture reference "@.len() != "missing remote fixture reference"@.len());
        assert("unhandled"@.len() != "fixture apply conflict"@.len());
        assert("unhandled"@.len() != "fixture remote lease rejected"@.len());
        assert("unhandled"@.len() != "missing fixture reference"@.len());
        assert("unhandled"@.len() != "not a fixture ancestor"@.len());
        assert("unhandled"@.len() != "not a symbolic fixture ref"@.len());
        assert("unhandled"@.len() != "missing remote fixture reference"@.len());
    }
    let full = "fixture hook exit:42".as_bytes();
    proof {
        use vstd::string::StringSliceAdditionalSpecFns;
        assert(full@.take(18) =~= "fixture hook exit:".spec_bytes());
        assert("fixture hook exitx42".spec_bytes()[17] != "fixture hook exit:".spec_bytes()[17]);
    }
    let candidate = git::hook_exit_suffix(full);
    match candidate { Some(bytes) => { expect(bytes.len() == 2 && bytes[0] == 52 && bytes[1] == 50); }, None => { expect(false); }, }
    expect(git::hook_exit_suffix("fixture hook exit".as_bytes()).is_none());
    expect(git::hook_exit_suffix("fixture hook exitx42".as_bytes()).is_none());
    expect(git::model_error_exit("fixture apply conflict", None) == 1);
    expect(git::model_error_exit("fixture remote lease rejected", None) == 1);
    expect(git::model_error_exit("missing fixture reference", None) == 1);
    expect(git::model_error_exit("not a fixture ancestor", None) == 1);
    expect(git::model_error_exit("not a symbolic fixture ref", None) == 1);
    expect(git::model_error_exit("missing remote fixture reference", None) == 2);
    expect(git::model_error_exit("fixture hook exit:oops", None) == 93);
    expect(git::model_error_exit("missing fixture reference ", None) == 93);
    expect(git::model_error_exit("unhandled", None) == 93);
    expect(git::model_error_exit("fixture hook exit:42", Some(42)) == 42);
    expect(git::model_error_exit("fixture hook exit:0", Some(0)) == 0);
    expect(git::model_error_exit("fixture hook exit:-1", Some(-1)) == -1);
}

#[cfg_attr(not(verus_keep_ghost), test)]
fn known_git_command_independent_boundaries() {
    proof {
        reveal_strlit("tag"); reveal_strlit("apply");
        reveal_strlit("--version"); reveal_strlit("check-ref-format"); reveal_strlit("init"); reveal_strlit("clone"); reveal_strlit("add"); reveal_strlit("commit"); reveal_strlit("commit-tree"); reveal_strlit("rev-parse"); reveal_strlit("show"); reveal_strlit("ls-tree"); reveal_strlit("cat-file"); reveal_strlit("ls-files"); reveal_strlit("config"); reveal_strlit("diff"); reveal_strlit("update-index"); reveal_strlit("update-ref"); reveal_strlit("reset"); reveal_strlit("restore"); reveal_strlit("checkout"); reveal_strlit("switch"); reveal_strlit("status"); reveal_strlit("symbolic-ref"); reveal_strlit("branch"); reveal_strlit("show-ref"); reveal_strlit("for-each-ref"); reveal_strlit("remote"); reveal_strlit("ls-remote"); reveal_strlit("push"); reveal_strlit("fetch"); reveal_strlit("merge-base"); reveal_strlit("rev-list"); reveal_strlit("merge"); reveal_strlit("write-tree"); reveal_strlit("read-tree"); reveal_strlit("mv"); reveal_strlit("rm"); reveal_strlit("replace"); reveal_strlit("reflog"); reveal_strlit("worktree"); reveal_strlit(""); reveal_strlit("unknown command"); reveal_strlit("BUILD");
        assert(""@.len() == 0);
        assert("BUILD"@.len() != "clone"@.len() || "BUILD"@[0] != "clone"@[0]);
        assert("BUILD"@.len() != "reset"@.len() || "BUILD"@[0] != "reset"@[0]);
        assert("BUILD"@.len() != "merge"@.len() || "BUILD"@[0] != "merge"@[0]);
        assert("BUILD"@.len() != "fetch"@.len() || "BUILD"@[0] != "fetch"@[0]);
        assert("unknown command"@.len() != "--version"@.len() || "unknown command"@[0] != "--version"@[0]);
        assert("unknown command"@.len() != "check-ref-format"@.len() || "unknown command"@[0] != "check-ref-format"@[0]);
        assert("unknown command"@.len() != "init"@.len() || "unknown command"@[0] != "init"@[0]);
        assert("unknown command"@.len() != "clone"@.len() || "unknown command"@[0] != "clone"@[0]);
        assert("unknown command"@.len() != "add"@.len() || "unknown command"@[0] != "add"@[0]);
        assert("unknown command"@.len() != "commit"@.len() || "unknown command"@[0] != "commit"@[0]);
        assert("unknown command"@.len() != "commit-tree"@.len() || "unknown command"@[0] != "commit-tree"@[0]);
        assert("unknown command"@.len() != "rev-parse"@.len() || "unknown command"@[0] != "rev-parse"@[0]);
        assert("unknown command"@.len() != "show"@.len() || "unknown command"@[0] != "show"@[0]);
        assert("unknown command"@.len() != "ls-tree"@.len() || "unknown command"@[0] != "ls-tree"@[0]);
        assert("unknown command"@.len() != "cat-file"@.len() || "unknown command"@[0] != "cat-file"@[0]);
        assert("unknown command"@.len() != "ls-files"@.len() || "unknown command"@[0] != "ls-files"@[0]);
        assert("unknown command"@.len() != "config"@.len() || "unknown command"@[0] != "config"@[0]);
        assert("unknown command"@.len() != "diff"@.len() || "unknown command"@[0] != "diff"@[0]);
        assert("unknown command"@.len() != "update-index"@.len() || "unknown command"@[0] != "update-index"@[0]);
        assert("unknown command"@.len() != "update-ref"@.len() || "unknown command"@[0] != "update-ref"@[0]);
        assert("unknown command"@.len() != "reset"@.len() || "unknown command"@[0] != "reset"@[0]);
        assert("unknown command"@.len() != "restore"@.len() || "unknown command"@[0] != "restore"@[0]);
        assert("unknown command"@.len() != "checkout"@.len() || "unknown command"@[0] != "checkout"@[0]);
        assert("unknown command"@.len() != "switch"@.len() || "unknown command"@[0] != "switch"@[0]);
        assert("unknown command"@.len() != "status"@.len() || "unknown command"@[0] != "status"@[0]);
        assert("unknown command"@.len() != "symbolic-ref"@.len() || "unknown command"@[0] != "symbolic-ref"@[0]);
        assert("unknown command"@.len() != "branch"@.len() || "unknown command"@[0] != "branch"@[0]);
        assert("unknown command"@.len() != "show-ref"@.len() || "unknown command"@[0] != "show-ref"@[0]);
        assert("unknown command"@.len() != "for-each-ref"@.len() || "unknown command"@[0] != "for-each-ref"@[0]);
        assert("unknown command"@.len() != "remote"@.len() || "unknown command"@[0] != "remote"@[0]);
        assert("unknown command"@.len() != "ls-remote"@.len() || "unknown command"@[0] != "ls-remote"@[0]);
        assert("unknown command"@.len() != "push"@.len() || "unknown command"@[0] != "push"@[0]);
        assert("unknown command"@.len() != "fetch"@.len() || "unknown command"@[0] != "fetch"@[0]);
        assert("unknown command"@.len() != "merge-base"@.len() || "unknown command"@[0] != "merge-base"@[0]);
        assert("unknown command"@.len() != "rev-list"@.len() || "unknown command"@[0] != "rev-list"@[0]);
        assert("unknown command"@.len() != "merge"@.len() || "unknown command"@[0] != "merge"@[0]);
        assert("unknown command"@.len() != "write-tree"@.len() || "unknown command"@[0] != "write-tree"@[0]);
        assert("unknown command"@.len() != "read-tree"@.len() || "unknown command"@[0] != "read-tree"@[0]);
        assert("unknown command"@.len() != "mv"@.len() || "unknown command"@[0] != "mv"@[0]);
        assert("unknown command"@.len() != "rm"@.len() || "unknown command"@[0] != "rm"@[0]);
        assert("unknown command"@.len() != "replace"@.len() || "unknown command"@[0] != "replace"@[0]);
        assert("unknown command"@.len() != "reflog"@.len() || "unknown command"@[0] != "reflog"@[0]);
        assert("unknown command"@.len() != "worktree"@.len() || "unknown command"@[0] != "worktree"@[0]);
        assert("BUILD"@[0] != "--version"@[0] && "BUILD"@[0] != "check-ref-format"@[0]
            && "BUILD"@[0] != "init"@[0] && "BUILD"@[0] != "clone"@[0]
            && "BUILD"@[0] != "add"@[0] && "BUILD"@[0] != "commit"@[0]
            && "BUILD"@[0] != "commit-tree"@[0] && "BUILD"@[0] != "rev-parse"@[0]
            && "BUILD"@[0] != "show"@[0] && "BUILD"@[0] != "ls-tree"@[0]
            && "BUILD"@[0] != "cat-file"@[0] && "BUILD"@[0] != "ls-files"@[0]
            && "BUILD"@[0] != "config"@[0] && "BUILD"@[0] != "diff"@[0]
            && "BUILD"@[0] != "update-index"@[0] && "BUILD"@[0] != "update-ref"@[0]
            && "BUILD"@[0] != "restore"@[0] && "BUILD"@[0] != "reset"@[0] && "BUILD"@[0] != "checkout"@[0]
            && "BUILD"@[0] != "switch"@[0] && "BUILD"@[0] != "status"@[0]
            && "BUILD"@[0] != "symbolic-ref"@[0] && "BUILD"@[0] != "branch"@[0]
            && "BUILD"@[0] != "show-ref"@[0] && "BUILD"@[0] != "for-each-ref"@[0]
            && "BUILD"@[0] != "remote"@[0] && "BUILD"@[0] != "ls-remote"@[0]
            && "BUILD"@[0] != "push"@[0] && "BUILD"@[0] != "fetch"@[0]
            && "BUILD"@[0] != "merge-base"@[0] && "BUILD"@[0] != "rev-list"@[0]
            && "BUILD"@[0] != "merge"@[0] && "BUILD"@[0] != "write-tree"@[0]
            && "BUILD"@[0] != "read-tree"@[0] && "BUILD"@[0] != "mv"@[0]
            && "BUILD"@[0] != "rm"@[0] && "BUILD"@[0] != "replace"@[0]
            && "BUILD"@[0] != "reflog"@[0] && "BUILD"@[0] != "worktree"@[0]);
    }
    expect(git::known_git_command("--version"));
    expect(git::known_git_command("worktree"));
    expect(git::known_git_command("tag"));
    expect(git::known_git_command("apply"));
    expect(git::known_git_command("restore"));
    expect(!git::known_git_command(""));
    expect(!git::known_git_command("unknown command"));
    expect(!git::known_git_command("BUILD"));
}
// "#x\n\n//a/\r\n*.c\nb\r" includes a CRLF and a final literal CR.
const RULES: [u8; 16] = [35, 120, 10, 10, 47, 47, 97, 47, 13, 10, 42, 46, 99, 10, 98, 13];
const INSIDE: [u8; 3] = [97, 47, 120];
const EXACT: [u8; 1] = [97];
const EXTENSION: [u8; 3] = [113, 46, 99];
const COMMENT: [u8; 1] = [120];
const FINAL_CR: [u8; 2] = [98, 13];
const BARE: [u8; 1] = [98];

#[cfg_attr(not(verus_keep_ghost), test)]
fn finite_ignore_patterns_and_line_boundaries() {
    proof {
        assert(git::ignored_from(RULES@, INSIDE@, 0)) by (compute);
        assert(git::ignored_from(RULES@, EXACT@, 0)) by (compute);
        assert(git::ignored_from(RULES@, EXTENSION@, 0)) by (compute);
        assert(!git::ignored_from(RULES@, COMMENT@, 0)) by (compute);
        assert(git::ignored_from(RULES@, FINAL_CR@, 0)) by (compute);
        assert(!git::ignored_from(RULES@, BARE@, 0)) by (compute);
    }
    expect(git::ignored_text(&RULES, &INSIDE));
    expect(git::ignored_text(&RULES, &EXACT));
    expect(git::ignored_text(&RULES, &EXTENSION));
    expect(!git::ignored_text(&RULES, &COMMENT));
    expect(git::ignored_text(&RULES, &FINAL_CR));
    expect(!git::ignored_text(&RULES, &BARE));
}

#[cfg_attr(not(verus_keep_ghost), test)]
fn diff_reads_the_index_even_when_the_worktree_is_hidden() {
    use git::DiffSource::*;
    proof { reveal_strlit("h"); reveal_strlit("S"); reveal_strlit("H");
        assert("H"@[0] != "h"@[0] && "H"@[0] != "S"@[0]);
    }
    expect(matches!(git::diff_source(true, Some("h")), Index));
    expect(matches!(git::diff_source(true, Some("S")), Index));
    expect(matches!(git::diff_source(false, Some("h")), Hidden));
    expect(matches!(git::diff_source(false, Some("S")), Hidden));
    expect(matches!(git::diff_source(false, Some("H")), Worktree));
    expect(matches!(git::diff_source(false, None), Worktree));
}

#[cfg_attr(not(verus_keep_ghost), test)]
fn diff_compares_contents_types_and_presence() {
    proof {
        reveal_strlit("120000"); reveal_strlit("100644"); reveal_strlit("100755");
        reveal_strlit("x"); reveal_strlit("y");
        assert("x"@[0] != "y"@[0]);
        assert("120000"@[1] != "100644"@[1] && "120000"@[1] != "100755"@[1]);
        assert("100644"@[3] != "100755"@[3]);
    }
    let link = Some(("120000", "x"));
    let file = Some(("100644", "x"));
    let executable = Some(("100755", "x"));
    let edited = Some(("100644", "y"));
    expect(!git::diff_entry_changed(None, None, false, false, false));
    expect(git::diff_entry_changed(file, None, false, false, false));
    expect(git::diff_entry_changed(None, file, true, false, false));
    expect(!git::diff_entry_changed(file, file, true, true, true));
    expect(git::diff_entry_changed(file, edited, false, false, false));
    expect(git::diff_entry_changed(link, edited, false, false, true));
    expect(!git::diff_entry_changed(link, file, false, false, true));
    expect(git::diff_entry_changed(link, file, true, false, true));
    expect(git::diff_entry_changed(link, file, false, true, true));
    expect(git::diff_entry_changed(file, link, false, false, true));
    expect(git::diff_entry_changed(link, executable, false, false, true));
    expect(git::diff_entry_changed(file, executable, false, false, true));
    expect(!git::diff_entry_changed(file, executable, false, true, false));
}

const MIXED_BYTES: [u8; 9] = [13, 13, 10, 0, 255, 10, 13, 10, 13];
const NORMALIZED: [u8; 7] = [13, 10, 0, 255, 10, 10, 13];

#[cfg_attr(not(verus_keep_ghost), test)]
fn text_diff_preserves_every_byte_except_a_cr_before_lf() {
    proof {
        assert(git::normalized_prefix(MIXED_BYTES@, 9) == NORMALIZED@) by (compute);
    }
    let result = git::normalize_text(&MIXED_BYTES);
    expect(result.len() == 7);
    let mut i = 0;
    while i < result.len()
        invariant result@ == NORMALIZED@, i <= result.len(),
        decreases result.len() - i,
    {
        expect(result[i] == NORMALIZED[i]);
        i += 1;
    }
    expect(git::normalize_text(&[]).is_empty());
}

}

#[cfg(not(verus_keep_ghost))]
#[test]
fn dispatch_rejects_unknown_commands_before_opening_a_repository() {
    let directory = tempfile::tempdir().unwrap();
    let run = |command: &str| git::run(directory.path().to_path_buf(), &[command.to_owned()], &[]);
    assert_eq!(run("--version").unwrap(), b"git version fixture\n");
    assert_eq!(
        run("BUILD").unwrap_err(),
        "unhandled Git fixture command: [\"BUILD\"]"
    );
}

#[cfg(not(verus_keep_ghost))]
#[test]
fn diff_reads_staged_entries_even_when_worktree_comparisons_are_hidden() {
    use std::{fs, os::unix::fs::symlink, process::Command};

    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    let run = |args: &[&str]| {
        let output = Command::new(env!("CHRONO_TEST_GIT_BIN"))
            .current_dir(root)
            .args(args)
            .output()
            .unwrap();
        assert!(output.status.success(), "{args:?}: {output:?}");
        assert!(output.stderr.is_empty(), "{args:?}: {output:?}");
        output.stdout
    };
    run(&["init", "-q", "-b", "dev"]);
    symlink("target", root.join("alias")).unwrap();
    fs::write(root.join("payload"), [13, 13, 10, 0, 255, 13]).unwrap();
    fs::write(root.join(".gitattributes"), "payload text\n").unwrap();
    run(&["add", "--all"]);
    run(&["commit", "-qm", "base"]);
    fs::remove_file(root.join("alias")).unwrap();
    fs::write(root.join("alias"), "target").unwrap();
    fs::write(root.join("payload"), [13, 10, 0, 255, 13]).unwrap();
    let worktree = ["diff", "--name-only", "HEAD", "--"];
    let staged = ["diff", "--cached", "--name-only"];
    assert_eq!(run(&worktree), b"alias\npayload\n");
    run(&["config", "core.symlinks", "false"]);
    fs::write(root.join("payload"), [13, 13, 10, 0, 255, 13]).unwrap();
    assert!(run(&worktree).is_empty());
    fs::write(root.join("payload"), "staged edit").unwrap();
    run(&["add", "payload"]);
    run(&["add", "alias"]);
    for flag in ["--skip-worktree", "--assume-unchanged"] {
        run(&["update-index", flag, "payload"]);
        assert!(run(&worktree).is_empty());
        assert_eq!(run(&staged), b"alias\npayload\n");
    }
    run(&["update-index", "--no-assume-unchanged", "payload"]);
    assert_eq!(run(&worktree), b"payload\n");
    fs::write(root.join(".gitattributes"), "payload filter=hide\n").unwrap();
    assert_eq!(run(&worktree), b".gitattributes\n");
    assert_eq!(run(&staged), b"alias\npayload\n");
}
