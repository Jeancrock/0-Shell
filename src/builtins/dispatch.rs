use crate::builtins::*;

pub fn dispatch(args: &[String], history: &Vec<String>) -> Result<bool, String> {
    match args[0].as_str() {
        "help" => {
            println!(
                r#"Usage:
    Exit:
    $exit

    Display a message:
    $echo "[text]"

    Choose Directory:
    $cd [path]

    Clear the terminal:
    $clear

    Print working directory:
    $pwd

    List directory contents:
    $ls [-a] [-F] [-l] [-r] [-R] [path]

    -a : Include hidden files
    -F : Append indicator (one of */=>@|) to entries
    -l : Use a long listing format
    -r : Reverse order while sorting
    -R : List subdirectories recursively

    Display file contents:
    $cat [path/file]

    Copy files and directories:
    $cp [dir or file to copy] [path of the destination]

    Remove files or directories:
    $rm [-r] [path/file or dir]

    -r : Remove directories and their contents recursively

    Move files and directories:
    $mv [dir or file to move] [path of the destination]

    Create directory(ies):
    $mkdir [-p] [path/dir]

    -p : Create multiple directories

    Show command history:
    $history

    Create an empty file or update the access and modification times of a file:
    $touch [path/file]"#
            );
            Ok(true)
        }

        "exit" => Ok(false),
        "echo" => {
            let _ = echo::cmd_echo(args);
            Ok(true)
        }
        "cd" => {
            cd::cmd_cd(args);
            Ok(true)
        }
        "pwd" => {
            let _ = pwd::cmd_pwd(args);
            Ok(true)
        }
        "ls" => {
            let _ = ls::cmd_ls(args);
            Ok(true)
        }
        "cat" => {
            let _ = cat::cmd_cat(args);
            Ok(true)
        }
        "cp" => {
            let _ = cp::cmd_cp(args);
            Ok(true)
        }
        "rm" => {
            let _ = rm::cmd_rm(args);
            Ok(true)
        }
        "mv" => {
            let _ = mv::cmd_mv(args);
            Ok(true)
        }
        "mkdir" => {
            let _ = mkdir::cmd_mkdir(args);
            Ok(true)
        }
        "history" => {
            println!("{:?}", history);
            Ok(true)
        }
        "clear" => {
            clear::cmd_clear()?;
            Ok(true)
        }
        "touch" => {
            if let Err(e) = touch::cmd_touch(args) {
                eprintln!("touch: {}", e);
            }
            Ok(true)
        }
        "write" => {
            if let Err(e) = write::cmd_write(args) {
                eprintln!("write: {}", e);
            }
            Ok(true)
        }

        _ => Err(format!("Command '{}' not found", args[0])),
    }
}
