
use crate::cli::args::git_parse_args; // 引入命令行参数解析模块
use crate::commands::init::git_init; 
use crate::commands::add::git_add;
use crate::commands::rm::git_remove;
use crate::commands::commit::git_commit;
use crate::commands::branch::git_branch;
use crate::commands::checkout::git_checkout;
use crate::commands::merge::git_merge;
use crate::commands::fetch::git_fetch;
use crate::commands::pull::git_pull;
use crate::commands::push::git_push;

pub fn git_execute(){
    // 解析命令行参数
    let matches = git_parse_args();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let files: Vec<&str> = sub_matches
               .get_many::<String>("file")
               .unwrap_or_default()
               .map(|s| s.as_str())
               .collect();
            git_add();
        }
        Some(("branch", sub_matches)) => {
            let branch_name = sub_matches.get_one::<String>("branch_name").unwrap();
            let delete = sub_matches.get_flag("delete");
            git_branch();
        }
        Some(("checkout", sub_matches)) => {
            let target = sub_matches.get_one::<String>("target").unwrap();
            git_checkout();
        }
        Some(("commit", sub_matches)) => {
            let message = sub_matches.get_one::<String>("message").unwrap();
            git_commit();
        }
        Some(("fetch", sub_matches)) => {
            let remote_url = sub_matches.get_one::<String>("remote_url").unwrap();
            git_fetch();
        }
        Some(("init",sub_matches)) => {
            let init_dir = sub_matches.get_one::<String>("init_dir").unwrap();
            git_init(init_dir);
        }
        Some(("merge", sub_matches)) => {
            let branch_name = sub_matches.get_one::<String>("branch_name").unwrap();
            git_merge();
        }
        Some(("pull", sub_matches)) => {
            let remote_url = sub_matches.get_one::<String>("remote_url").unwrap();
            git_pull();
        }
        Some(("push", sub_matches)) => {
            let remote_url = sub_matches.get_one::<String>("remote_url").unwrap();
            git_push();
        }
        Some(("rm", sub_matches)) => {
            let file = sub_matches.get_one::<String>("file").unwrap();
            let force = sub_matches.get_flag("force");
            git_remove();
        }
        _ => {}
    }

}