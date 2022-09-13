#![allow(unused)]
use clap::{App, Arg, SubCommand};
use std::{io::Write, process};

pub struct Rignore<'a> {
    language: Option<&'a String>,
}

impl<'a> Rignore<'a> {
    pub fn new(language: Option<&'a String>) -> Self {
        Self { language }
    }
    pub async fn list_suported_langs(&self) -> Vec<String> {
        let list = match _list_suported_langs().await {
            Ok(list) => list,
            Err(e) => {
                eprintln!("ERROR: {e}");
                process::exit(1);
            }
        };
        list
    }
    pub async fn get_gitignore_file(&self) -> Result<String, &'static str> {
        let url = format!(
            "https://www.toptal.com/developers/gitignore/api/{}",
            self.language.unwrap()
        );
        let result = match reqwest::get(url).await {
            Ok(res) => res.text().await.unwrap(),
            Err(_) => return Err("Cannot send request to server"),
        };

        Ok(result)
    }
}

async fn _list_suported_langs() -> Result<Vec<String>, &'static str> {
    let url = "https://www.toptal.com/developers/gitignore/api/list";
    let result = match reqwest::get(url).await {
        Ok(res) => res.text().await.unwrap(),
        Err(_) => return Err("Cannot send request to server"),
    };

    let list_of_langs: Vec<String> = result.split(",").map(|value| value.to_owned()).collect();

    Ok(list_of_langs)
}

pub async fn run() -> Result<(), &'static str> {
    let app = App::new("rignore")
        .about("Simple gitignore generator")
        .version("1.0")
        .author("Hakan Vardar <hakovardar@gmail.com>")
        .arg(
            Arg::with_name("LANGUAGE")
                .help("Dowloads the chosen language from the api")
                .required(false),
        )
        .subcommand(SubCommand::with_name("list").about("Lists suported languages"))
        .get_matches();

    let language: Option<&String> = app.get_one("LANGUAGE");
    let cli = Rignore::new(language);
    let list = cli.list_suported_langs().await;

    if let Some(lang) = language {
        if list.contains(&lang) {
            let gitignore = match cli.get_gitignore_file().await {
                Ok(file) => file,
                Err(e) => {
                    return Err(e);
                }
            };
            let mut file = std::fs::File::create(".gitignore").unwrap();
            file.write(gitignore.as_bytes());
        } else {
            return Err("Language is not suported you use list command to see suported languages");
        }
    } else {
        for lang in list.iter() {
            println!("--> {lang}");
        }
    }

    if let Some(_) = app.subcommand_matches("list") {
        for lang in list.iter() {
            println!("--> {lang}");
        }
    }

    Ok(())
}
