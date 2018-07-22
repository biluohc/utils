extern crate statx;

use std::env;

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();
    if args.is_empty() {
        let help = r#"
        statx       : Print Help Message.
        statx <path>: Print the path Stat.
        statx <path> <any>: Print the path Stat but don't follow symlink.
        "#;
        println!("{}", help);
        return;
    }

    let rest = if args.len() == 1 {
        statx::statx(args[0].clone(), true)
    } else {
        statx::statx(args[0].clone(), false)
    };
    println!("{:?}", rest);

    // std();
}

#[allow(dead_code)]
fn std() {
    use std::fs;

    let data = fs::metadata(env::args().nth(1).unwrap());
    let ctime = data.and_then(|m| m.created());
    println!("{:?}", ctime);
    assert!(ctime.is_err());
}
