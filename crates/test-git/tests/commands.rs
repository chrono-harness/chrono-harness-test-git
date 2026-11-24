#![allow(unexpected_cfgs)]
#[cfg(verus_keep_ghost)]
#[path = "../src/lib.rs"]
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
