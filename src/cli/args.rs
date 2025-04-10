use clap::{Arg, ArgMatches, Command};

pub fn git_parse_args() -> ArgMatches{

    Command::new("rust-git")
       .version("0.1.0")
       .author("Your Name <your.email@example.com>")
       .about("A simple Git implementation in Rust")
        // 定义子命令：提交更改
       .subcommand(
            Command::new("commit")
               .about("Record changes to the repository")
               .arg(
                    Arg::new("message")
                       .help("Commit message")
                       .required(true),
                ),
        )
        .subcommand(
            Command::new("add")
               .arg(
                    Arg::new("file")
                       .help("File to add")
                       .required(true)
                       .num_args(1..) // 允许一个或多个参数
                       .action(clap::ArgAction::Append),
                ),
        )
        // 定义子命令：分支管理
       .subcommand(
            Command::new("branch")
               .about("List, create, or delete branches")
               .arg(
                    Arg::new("branch_name")
                       .help("Branch name")
                       .required(false),
                )
               .arg(
                    Arg::new("delete")
                       .help("Delete branch")
                       .short('d')
                       .long("delete").action(clap::ArgAction::SetTrue),
                ),
        )
        // 定义子命令：切换分支或恢复工作区文件
       .subcommand(
            Command::new("checkout")
               .about("Switch branches or restore working tree files")
               .arg(
                    Arg::new("target")
                       .help("Branch or commit to checkout")
                       .required(true),
                ),
        )
        // 定义子命令：合并分支
       .subcommand(
            Command::new("merge")
               .about("Join two or more development histories together")
               .arg(
                    Arg::new("branch_name")
                       .help("Branch to merge")
                       .required(true),
                ),
        )
        .subcommand(
            Command::new("fetch")
               .about("Download objects and refs from another repository")
               .arg(
                    Arg::new("remote_url")
                       .help("Remote repository URL")
                       .required(true),
                ),
        )
        .subcommand(
            Command::new("init")
               .arg(
                    Arg::new("init_dir")
                       .help("Init")
                       .required(false)
                ),
        )
        // 定义子命令：拉取并合并
       .subcommand(
            Command::new("pull")
               .about("Fetch from and integrate with another repository or a local branch")
               .arg(
                    Arg::new("remote_url")
                       .help("Remote repository URL")
                       .required(true),
                ),
        )
        // 定义子命令：推送更改
       .subcommand(
            Command::new("push")
               .about("Update remote refs along with associated objects")
               .arg(
                    Arg::new("remote_url")
                       .help("Remote repository URL")
                       .required(true),
                ),
        )
        // 定义子命令：rm
       .subcommand(
            Command::new("rm")
               .arg(
                    Arg::new("file")
                       .help("File to remove")
                       .required(true),
                )
               .arg(
                    Arg::new("force")
                       .help("Force removal")
                       .long("force")
                       .action(clap::ArgAction::SetTrue),
                ),
        )
       .get_matches()
}