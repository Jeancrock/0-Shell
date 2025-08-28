use crate::builtins::cat;
use crate::builtins::cd;
use crate::builtins::clear;
use crate::builtins::cp;
use crate::builtins::echo;
use crate::builtins::ls;
use crate::builtins::mkdir;
use crate::builtins::mv;
use crate::builtins::pwd;
use crate::builtins::rm;
use crate::builtins::touch;
use crate::builtins::write;

pub fn dispatch(args: &[String], history: &Vec<String>) -> Result<bool, String> {
    match args[0].as_str() {
        "help" => {
            println!(
                r#"Usage:
    exit
    echo [text]
    cd [path]
    pwd
    ls [-l] [-a] [-F] [path]
    cat <file>
    cp <src> <dst>
    rm [-r] <path>
    mv <src> <dst>
    mkdir <dir>"#
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
