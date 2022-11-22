#![allow(unused)]

mod file;
mod list;

use file::Filee;
use list::List;
use std::env;
use std::process;

pub async fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let list = List::new().await;

    if args.len() == 2 {
        if &args[1] == &String::from("help") {
            println!("INFO:");
            println!("     -> rignore help == prints commands");
            println!("     -> rignore list == lists supported langs");
            println!(
                "     -> rignore <supported_lang(example = 'rust')> == creates a .gitignore file"
            );
        } else if &args[1] == &String::from("list") {
            for lang in list.lang_list {
                println!("{lang}");
            }
        } else if &args[1] == &String::from("clear") {
            List::clear();
        } else {
            let file = Filee::new(&args[1], list).await?;
            file.get_file().await;
        }
    } else {
        eprintln!("ERROR: You need to give only 1 argument.");
        process::exit(1);
    }

    Ok(())
}
