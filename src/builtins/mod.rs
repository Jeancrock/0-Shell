use clear::cmd_clear;

pub mod cat;
pub mod cd;
pub mod cp;
pub mod echo;
pub mod ls;
pub mod mkdir;
pub mod mv;
pub mod pwd;
pub mod clear;
pub mod rm;

pub fn dispatch(args: &[String], history: &Vec<String>) -> Result<bool, String> {
    match args[0].as_str() {
        "help" => {
            println!(
                r#"Usage:
          exit
              Exit the shell.
        
          echo [text]
              Prints text to the standard output.
              Example: echo "Hello World"
        
          cd [path]
              Change current directory to PATH.
              Example: cd /home/user
        
          pwd
              Print current working directory.
        
          ls [-l] [-a] [-F] [path]
              List files and directories.
              Options:
                -l  : long listing format
                -a  : include hidden files
                -F  : append indicator (/ for directories)
              Example: ls -laF /home/user
        
          cat <file>
              Display contents of FILE.
              Example: cat file.txt
        
          cp <source> <target>
              Copy file or directory.
              Example: cp file.txt backup.txt
        
          rm [-r] <path>
              Remove file or directory.
              Options:
                -r : recursive remove for directories
              Example: rm -r my_folder
        
          mv <source> <target>
              Move or rename file/directory.
              Example: mv old.txt new.txt
        
          mkdir <directory>
              Create a new directory.
              Example: mkdir new_folder"#
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
            cmd_clear()?;
            Ok(true)
        },
        _ => Err(format!("Command '{}' not found", args[0])),
    }
}
