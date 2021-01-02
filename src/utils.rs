use std::process::Command;
use std::time::Instant;

#[derive(Debug)]
pub struct CommandOutput {
  pub stdout: String,
  pub stderr: String,
}

impl PartialEq for CommandOutput {
  fn eq(&self, other: &Self) -> bool {
    self.stdout == other.stdout && self.stderr == other.stderr
  }
}

/// Execute a command and return the output on stdout and stderr if successful
#[cfg(not(test))]
pub fn exec_cmd(cmd: &str, args: &[&str]) -> Option<CommandOutput> {
  internal_exec_cmd(&cmd, &args)
}

#[cfg(test)]
pub fn exec_cmd(cmd: &str, args: &[&str]) -> Option<CommandOutput> {
  let command = match args.len() {
    0 => String::from(cmd),
    _ => format!("{} {}", cmd, args.join(" ")),
  };
  match command.as_str() {
        "crystal --version" => Some(CommandOutput {
            stdout: String::from(
                "\
Crystal 0.35.1 (2020-06-19)

LLVM: 10.0.0
Default target: x86_64-apple-macosx\n",
            ),
            stderr: String::default(),
        }),
        "dart --version" => Some(CommandOutput {
            stdout: String::default(),
            stderr: String::from(
                "Dart VM version: 2.8.4 (stable) (Wed Jun 3 12:26:04 2020 +0200) on \"macos_x64\"",
            ),
        }),
        "dummy_command" => Some(CommandOutput {
            stdout: String::from("stdout ok!\n"),
            stderr: String::from("stderr ok!\n"),
        }),
        "elixir --version" => Some(CommandOutput {
            stdout: String::from(
                "\
Erlang/OTP 22 [erts-10.6.4] [source] [64-bit] [smp:8:8] [ds:8:8:10] [async-threads:1] [hipe]

Elixir 1.10 (compiled with Erlang/OTP 22)\n",
            ),
            stderr: String::default(),
        }),
        "elm --version" => Some(CommandOutput {
            stdout: String::from("0.19.1\n"),
            stderr: String::default(),
        }),
        "go version" => Some(CommandOutput {
            stdout: String::from("go version go1.12.1 linux/amd64\n"),
            stderr: String::default(),
        }),
        "helm version --short --client" => Some(CommandOutput {
            stdout: String::from("v3.1.1+gafe7058\n"),
            stderr: String::default(),
        }),
        s if s.ends_with("java -Xinternalversion") => Some(CommandOutput {
            stdout: String::from("OpenJDK 64-Bit Server VM (13.0.2+8) for bsd-amd64 JRE (13.0.2+8), built on Feb  6 2020 02:07:52 by \"brew\" with clang 4.2.1 Compatible Apple LLVM 11.0.0 (clang-1100.0.33.17)"),
            stderr: String::default(),
        }),
        "julia --version" => Some(CommandOutput {
            stdout: String::from("julia version 1.4.0\n"),
            stderr: String::default(),
        }),
        "kotlin -version" => Some(CommandOutput {
            stdout: String::from("Kotlin version 1.4.21-release-411 (JRE 14.0.1+7)\n"),
            stderr: String::default(),
        }),
        "kotlinc -version" => Some(CommandOutput {
            stdout: String::from("info: kotlinc-jvm 1.4.21 (JRE 14.0.1+7)\n"),
            stderr: String::default(),
        }),
        "lua -v" => Some(CommandOutput{
            stdout: String::from("Lua 5.4.0  Copyright (C) 1994-2020 Lua.org, PUC-Rio\n"),
            stderr: String::default(),
        }),
        "luajit -v" => Some(CommandOutput{
            stdout: String::from("LuaJIT 2.0.5 -- Copyright (C) 2005-2017 Mike Pall. http://luajit.org/\n"),
            stderr: String::default(),
        }),
        "nim --version" => Some(CommandOutput {
            stdout: String::from(
                "\
Nim Compiler Version 1.2.0 [Linux: amd64]
Compiled at 2020-04-03
Copyright (c) 2006-2020 by Andreas Rumpf
git hash: 7e83adff84be5d0c401a213eccb61e321a3fb1ff
active boot switches: -d:release\n",
            ),
            stderr: String::default(),
        }),
        "node --version" => Some(CommandOutput {
            stdout: String::from("v12.0.0\n"),
            stderr: String::default(),
        }),
        "ocaml -vnum" => Some(CommandOutput {
            stdout: String::from("4.10.0\n"),
            stderr: String::default(),
        }),
        "esy ocaml -vnum" => Some(CommandOutput {
            stdout: String::from("4.08.1\n"),
            stderr: String::default(),
        }),
        "perl -e printf q#%vd#,$^V;" => Some(CommandOutput {
            stdout: String::from("5.26.1"),
            stderr: String::default(),
        }),
        "php -nr echo PHP_MAJOR_VERSION.\".\".PHP_MINOR_VERSION.\".\".PHP_RELEASE_VERSION;" => {
            Some(CommandOutput {
                stdout: String::from("7.3.8"),
                stderr: String::default(),
            })
        }
        "purs --version" => Some(CommandOutput {
            stdout: String::from("0.13.5\n"),
            stderr: String::default(),
        }),
        "pyenv version-name" => Some(CommandOutput {
            stdout: String::from("system\n"),
            stderr: String::default(),
        }),
        "python --version" => None,
        "python2 --version" => Some(CommandOutput {
            stdout: String::default(),
            stderr: String::from("Python 2.7.17\n"),
        }),
        "python3 --version" => Some(CommandOutput {
            stdout: String::from("Python 3.8.0\n"),
            stderr: String::default(),
        }),
        "ruby -v" => Some(CommandOutput {
            stdout: String::from("ruby 2.5.1p57 (2018-03-29 revision 63029) [x86_64-linux-gnu]\n"),
            stderr: String::default(),
        }),
        "swift --version" => Some(CommandOutput {
            stdout: String::from(
                "\
Apple Swift version 5.2.2 (swiftlang-1103.0.32.6 clang-1103.0.32.51)
Target: x86_64-apple-darwin19.4.0\n",
            ),
            stderr: String::default(),
        }),
        "zig version" => Some(CommandOutput {
            stdout: String::from("0.6.0\n"),
            stderr: String::default(),
        }),
        "cmake --version" => Some(CommandOutput {
            stdout: String::from(
                "\
cmake version 3.17.3

CMake suite maintained and supported by Kitware (kitware.com/cmake).\n",
            ),
            stderr: String::default(),
        }),
        "dotnet --version" => Some(CommandOutput {
            stdout: String::from("3.1.103"),
            stderr: String::default(),
        }),
        "dotnet --list-sdks" => Some(CommandOutput {
            stdout: String::from("3.1.103 [/usr/share/dotnet/sdk]"),
            stderr: String::default(),
        }),
        "terraform version" => Some(CommandOutput {
            stdout: String::from("Terraform v0.12.14\n"),
            stderr: String::default(),
        }),
        s if s.starts_with("erl -noshell -eval") => Some(CommandOutput {
            stdout: String::from("22.1.3\n"),
            stderr: String::default(),
        }),
        // If we don't have a mocked command fall back to executing the command
        _ => internal_exec_cmd(&cmd, &args),
    }
}

fn internal_exec_cmd(cmd: &str, args: &[&str]) -> Option<CommandOutput> {
  log::trace!("Executing command {:?} with args {:?}", cmd, args);

  let full_path = match which::which(cmd) {
    Ok(full_path) => {
      log::trace!("Using {:?} as {:?}", full_path, cmd);
      full_path
    }
    Err(e) => {
      log::trace!("Unable to find {:?} in PATH, {:?}", cmd, e);
      return None;
    }
  };

  let start = Instant::now();
  match Command::new(full_path).args(args).output() {
    Ok(output) => {
      let stdout_string = String::from_utf8(output.stdout).unwrap();
      let stderr_string = String::from_utf8(output.stderr).unwrap();

      log::trace!(
        "stdout: {:?}, stderr: {:?}, exit code: \"{:?}\", took {:?}",
        stdout_string,
        stderr_string,
        output.status.code(),
        start.elapsed()
      );

      if !output.status.success() {
        return None;
      }

      Some(CommandOutput {
        stdout: stdout_string,
        stderr: stderr_string,
      })
    }
    Err(error) => {
      log::info!("Executing command {:?} failed by: {:?}", cmd, error);
      None
    }
  }
}

#[cfg(test)]
#[cfg(not(windows))] // While the exec_cmd should work on Windows these tests assume a Unix-like environment.
mod tests {
  use super::*;

  #[test]
  fn exec_mocked_command() {
    let result = exec_cmd("dummy_command", &[]);
    let expected = Some(CommandOutput {
      stdout: String::from("stdout ok!\n"),
      stderr: String::from("stderr ok!\n"),
    });

    assert_eq!(result, expected)
  }

  #[test]
  fn exec_no_output() {
    let result = internal_exec_cmd("true", &[]);
    let expected = Some(CommandOutput {
      stdout: String::from(""),
      stderr: String::from(""),
    });

    assert_eq!(result, expected)
  }

  #[test]
  fn exec_with_output_stdout() {
    let result = internal_exec_cmd("/bin/sh", &["-c", "echo hello"]);
    let expected = Some(CommandOutput {
      stdout: String::from("hello\n"),
      stderr: String::from(""),
    });

    assert_eq!(result, expected)
  }

  #[test]
  fn exec_with_output_stderr() {
    let result = internal_exec_cmd("/bin/sh", &["-c", "echo hello >&2"]);
    let expected = Some(CommandOutput {
      stdout: String::from(""),
      stderr: String::from("hello\n"),
    });

    assert_eq!(result, expected)
  }

  #[test]
  fn exec_with_output_both() {
    let result = internal_exec_cmd("/bin/sh", &["-c", "echo hello; echo world >&2"]);
    let expected = Some(CommandOutput {
      stdout: String::from("hello\n"),
      stderr: String::from("world\n"),
    });

    assert_eq!(result, expected)
  }

  #[test]
  fn exec_with_non_zero_exit_code() {
    let result = internal_exec_cmd("false", &[]);
    let expected = None;

    assert_eq!(result, expected)
  }
}
