pub mod echo;
pub mod cd;
pub mod pwd;
pub mod ls;
pub mod cat;
pub mod cp;
pub mod rm;
pub mod mv;
pub mod mkdir;

pub fn dispatch(args: &[String]) -> Result<(), String> {
    match args[0].as_str() {
        "exit" => std::process::exit(0),
        "echo" => echo::cmd_echo(args),
        "cd"   => Ok(cd::cmd_cd(args)),
        "pwd"  => pwd::cmd_pwd(args),
        "ls"   => ls::cmd_ls(args),
        "cat"  => cat::cmd_cat(args),
        "cp"   => cp::cmd_cp(args),
        "rm"   => rm::cmd_rm(args),
        "mv"   => mv::cmd_mv(args),
        "mkdir"=> mkdir::cmd_mkdir(args),
        _ => Err(format!("Command '{}' not found", args[0])),
    }
}
