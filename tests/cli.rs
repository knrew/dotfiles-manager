use std::process::{Command, Output};

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_dotkoke"))
        .args(args)
        .output()
        .expect("dotkoke should run")
}

fn assert_stdout_snapshot(args: &[&str], expected: &str) {
    let output = run(args);

    assert!(output.status.success());
    assert_eq!(output.stdout, expected.as_bytes());
    assert!(output.stderr.is_empty());
}

fn assert_stderr_snapshot(args: &[&str], expected: &str) {
    let output = run(args);

    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert_eq!(output.stderr, expected.as_bytes());
}

#[test]
fn help_snapshots_match_the_cli_contract() {
    let cases: &[(&[&str], &str)] = &[
        (
            &["--help"],
            "\
Manage dotfiles safely

Usage: dotkoke <COMMAND>

Commands:
  init     Initialize the configuration and source tree
  install  Install all managed files
  add      Add files to the source tree
  remove   Remove files from management
  status   Show the status of managed files

Options:
  -h, --help     Print help
  -V, --version  Print version
",
        ),
        (
            &["init", "--help"],
            "\
Initialize the configuration and source tree

Usage: dotkoke init [OPTIONS]

Options:
      --dry-run  Show the plan without changing the file system
      --print    Print the fallback configuration
  -h, --help     Print help
",
        ),
        (
            &["install", "--help"],
            "\
Install all managed files

Usage: dotkoke install [OPTIONS]

Options:
      --config <PATH>  Use a specific configuration file
      --dry-run        Show the plan without changing the file system
  -h, --help           Print help
",
        ),
        (
            &["add", "--help"],
            "\
Add files to the source tree

Usage: dotkoke add [OPTIONS] <PATH>...

Arguments:
  <PATH>...  Paths to add

Options:
      --config <PATH>  Use a specific configuration file
      --dry-run        Show the plan without changing the file system
      --install        Install the added files
      --update         Update existing managed files
  -h, --help           Print help
",
        ),
        (
            &["remove", "--help"],
            "\
Remove files from management

Usage: dotkoke remove [OPTIONS] <PATH>...

Arguments:
  <PATH>...  Paths to remove

Options:
      --config <PATH>  Use a specific configuration file
      --dry-run        Show the plan without changing the file system
  -h, --help           Print help
",
        ),
        (
            &["status", "--help"],
            "\
Show the status of managed files

Usage: dotkoke status [OPTIONS]

Options:
      --config <PATH>  Use a specific configuration file
  -h, --help           Print help
",
        ),
    ];

    for (args, expected) in cases {
        assert_stdout_snapshot(args, expected);
    }
}

#[test]
fn version_is_written_to_stdout_and_succeeds() {
    assert_stdout_snapshot(&["--version"], "dotkoke 0.1.0\n");
}

#[test]
fn conflicting_options_are_usage_errors() {
    let cases: &[(&[&str], &str)] = &[
        (
            &["init", "--print", "--dry-run"],
            "\
error: the argument '--print' cannot be used with '--dry-run'

Usage: dotkoke init --print

For more information, try '--help'.
",
        ),
        (
            &["add", "--install", "--update", "path"],
            "\
error: the argument '--install' cannot be used with '--update'

Usage: dotkoke add --install <PATH>...

For more information, try '--help'.
",
        ),
    ];

    for (args, expected) in cases {
        assert_stderr_snapshot(args, expected);
    }
}

#[test]
fn add_and_remove_require_at_least_one_path() {
    let cases: &[(&[&str], &str)] = &[
        (
            &["add"],
            "\
error: the following required arguments were not provided:
  <PATH>...

Usage: dotkoke add <PATH>...

For more information, try '--help'.
",
        ),
        (
            &["remove"],
            "\
error: the following required arguments were not provided:
  <PATH>...

Usage: dotkoke remove <PATH>...

For more information, try '--help'.
",
        ),
    ];

    for (args, expected) in cases {
        assert_stderr_snapshot(args, expected);
    }
}

#[test]
fn commands_outside_the_spec_are_rejected() {
    let cases: &[(&[&str], &str)] = &[
        (
            &["list"],
            "\
error: unrecognized subcommand 'list'

  tip: a similar subcommand exists: 'install'

Usage: dotkoke <COMMAND>

For more information, try '--help'.
",
        ),
        (
            &["clean"],
            "\
error: unrecognized subcommand 'clean'

Usage: dotkoke <COMMAND>

For more information, try '--help'.
",
        ),
        (
            &["help"],
            "\
error: unrecognized subcommand 'help'

Usage: dotkoke <COMMAND>

For more information, try '--help'.
",
        ),
    ];

    for (args, expected) in cases {
        assert_stderr_snapshot(args, expected);
    }
}

#[test]
fn valid_commands_reach_the_unimplemented_stubs() {
    let cases: &[(&[&str], &str)] = &[
        (&["init"], "init"),
        (
            &["install", "--config", "config.toml", "--dry-run"],
            "install",
        ),
        (
            &["add", "--config", "config.toml", "--dry-run", "a", "b"],
            "add",
        ),
        (
            &["remove", "--config", "config.toml", "--dry-run", "a", "b"],
            "remove",
        ),
        (&["status", "--config", "config.toml"], "status"),
    ];

    for (args, command) in cases {
        assert_stderr_snapshot(
            args,
            &format!("Error: the '{command}' command is not implemented yet\n"),
        );
    }
}
