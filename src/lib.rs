#![allow(unused)]
use clap::{App, Arg, SubCommand};
use std::{
    fs::create_dir_all,
    fs::File,
    io::{Read, Write},
    process,
};

struct Rignore {
    cache: Cache,
}

struct Cache {
    suported_langs: Vec<String>,
}

impl Cache {
    async fn new() -> Self {
        let path = String::from(format!(
            "{}/rignore-cache-list",
            dirs::cache_dir().unwrap().to_string_lossy()
        ));

        let mut suported_langs = String::new();
        let mut suported_langs_vec = Vec::new();
        create_dir_all(path.clone());
        let mut file = match File::open(format!("{}{}", path, "/rignore-cache-list")) {
            Ok(mut file) => {
                let string = file.read_to_string(&mut suported_langs);
                for lang in suported_langs.split("\n") {
                    suported_langs_vec.push(lang.to_owned());
                }
            }
            Err(_) => {
                let mut lang_list =
                    File::create(format!("{}{}", path, "/rignore-cache-list")).unwrap();
                let mut list = Rignore::_list_suported_langs().await.unwrap();

                for line in list.iter_mut() {
                    line.push_str("\n");
                    lang_list.write(line.as_bytes());
                }
                suported_langs_vec = list;
            }
        };

        Cache {
            suported_langs: suported_langs_vec,
        }
    }
    async fn save(&self, lang: &String) -> Result<(), &'static str> {
        let path = String::from(format!(
            "{}/rignore-cache",
            dirs::cache_dir().unwrap().to_string_lossy()
        ));

        create_dir_all(path.clone());
        let mut file = match File::open(format!("{}/rignore-cache-{}", path, lang)) {
            Ok(mut file) => (),
            Err(e) => {
                let mut new_file =
                    File::create(format!("{}/rignore-cache-{}", path, lang)).unwrap();
                let url = format!("https://www.toptal.com/developers/gitignore/api/{}", lang);
                let result = match reqwest::get(url).await {
                    Ok(res) => res.text().await.unwrap(),
                    Err(_) => return Err("Cannot send request to server"),
                };

                new_file.write_all(result.as_bytes());
            }
        };

        Ok(())
    }
    fn clear() {
        let path = String::from(format!(
            "{}/rignore-cache",
            dirs::cache_dir().unwrap().to_string_lossy()
        ));
        std::fs::remove_dir_all(path);
    }
}

impl Rignore {
    async fn new() -> Self {
        Self {
            cache: Cache::new().await,
        }
    }
    async fn get_gitignore_file(&self, language: &String) -> Result<String, &'static str> {
        self.cache.save(language).await;
        let mut result = String::new();
        let path = String::from(format!(
            "{}/rignore-cache",
            dirs::cache_dir().unwrap().to_string_lossy()
        ));
        let mut file = File::open(format!("{}/rignore-cache-{}", path, language)).unwrap();
        file.read_to_string(&mut result);

        Ok(result)
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
        .subcommand(SubCommand::with_name("clear").about("Clears the cache"))
        .get_matches();

    let language: Option<&String> = app.get_one("LANGUAGE");
    let cli = Rignore::new().await;
    let list = &cli.cache.suported_langs;

    if let Some(lang) = language {
        if list.contains(&lang) {
            let gitignore = match cli.get_gitignore_file(lang).await {
                Ok(file) => file,
                Err(e) => {
                    return Err(e);
                }
            };
            let mut file = File::create(".gitignore").unwrap();
            file.write(gitignore.as_bytes());
        } else {
            return Err("Language is not suported you use list command to see suported languages");
        }
    } else {
        if let Some(_) = app.subcommand_matches("clear") {
        } else {
            for lang in list.iter() {
                if lang == "" {
                } else {
                    println!("-> {}", lang.trim());
                }
            }
        }
    }

    if let Some(_) = app.subcommand_matches("list") {
        for lang in list.iter() {
            if lang == "" {
            } else {
                println!("-> {}", lang);
            }
        }
    }
    if let Some(_) = app.subcommand_matches("clear") {
        Cache::clear();
    }

    Ok(())
}
