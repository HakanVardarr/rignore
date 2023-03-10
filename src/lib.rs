#![allow(unused)]

mod file;
mod list;

use file::File;
use list::List;
use std::env;
use std::process;

pub async fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        if args[1] == String::from("help") {
            println!("INFO:");
            println!("     -> rignore help == prints commands");
            println!("     -> rignore list == lists supported langs");
            println!("     -> rignore clear == clear cache");
            println!(
                "     -> rignore <supported_lang(example = 'rust')> == creates a .gitignore file"
            );
        } else if args[1] == String::from("list") {
            let list = List::new().await;
            for lang in list.lang_list {
                println!("{lang}");
            }
        } else if args[1] == String::from("clear") {
            List::clear();
        } else {
            let list = List::new().await;
            let file = File::new(&args[1], list)?;
            file.get_file().await;
        }
    } else {
        return Err("You need to give only 1 argument.".to_owned());
    }

    Ok(())
}
