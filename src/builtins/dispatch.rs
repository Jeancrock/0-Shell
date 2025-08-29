use crate::builtins::job_control::jobs_manager::JobManager;
use crate::builtins::*;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn dispatch(
    args: &[String],
    history: Arc<Mutex<Vec<String>>>,
    background: bool,
    job_manager: Arc<Mutex<JobManager>>,
) -> Result<bool, String> {
    let cmd_name = args[0].clone();

    if background {
        let args_vec = args.to_vec();
        let job_manager_clone = Arc::clone(&job_manager);
        let history_clone = Arc::clone(&history);

        let handle = thread::spawn(move || {
            let _ = dispatch(&args_vec, history_clone, false, job_manager_clone);
        });

        let mut jm = job_manager.lock().unwrap();
        let job_id = jm.add_job(cmd_name.clone(), handle);

        // Affichage à la Bash : [job_id] PID simulé
        let pid_sim = job_id + 500000; // juste pour simuler un PID
        println!("[{}] {}", job_id, pid_sim);

        Ok(true)
    } else {
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
            "jobs" => {
                job_manager.lock().unwrap().list_jobs();
                Ok(true)
            }
            "history" => {
                let hist = history.lock().unwrap();
                println!("{:?}", *hist);
                Ok(true)
            }
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
                let _ = ls::cmd_ls(args, background);
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
            "touch" => {
                let _ = touch::cmd_touch(args);
                Ok(true)
            }
            "clear" => {
                let _ = clear::cmd_clear();
                Ok(true)
            }
            _ => Err(format!("Command '{}' not found", args[0])),
        }
    }
}
