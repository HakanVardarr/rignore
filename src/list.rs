use std::fs::create_dir_all;
use std::fs::File;
use std::io::BufWriter;
use std::io::{Read, Write};

use crate::file;

#[derive(Debug)]
pub struct List {
    pub lang_list: Vec<String>,
}

impl List {
    pub async fn new() -> Self {
        Self {
            lang_list: List::get_list().await.unwrap(),
        }
    }
    async fn get_list() -> Result<Vec<String>, reqwest::Error> {
        let path = String::from(format!(
            "{}/rignore-cache",
            dirs::cache_dir().unwrap().to_string_lossy()
        ));
        let mut string_list = String::new();
        let mut language_list = Vec::new();

        create_dir_all(path.clone());

        let mut file = match File::open(format!("{}{}", path, "/rignore-cache-list")) {
            Ok(mut file_content) => {
                file_content.read_to_string(&mut string_list);
                let lines = string_list
                    .lines()
                    .map(|value| value.to_owned())
                    .collect::<Vec<String>>();
                for line in lines {
                    for lang in line.split(" ") {
                        language_list.push(lang.to_string());
                    }
                }
            }
            Err(_) => {
                let mut lang_list =
                    File::create(format!("{}{}", path, "/rignore-cache-list")).unwrap();
                let mut list = List::_get_list().await?;
                let mut writer = BufWriter::new(lang_list);

                for line in list.iter_mut() {
                    line.push_str(" ");
                    writer
                        .write_all(line.as_bytes())
                        .expect("ERROR: unable to write data");
                }

                list = list.iter().map(|s| s.trim().to_string()).collect();
                language_list = list;
            }
        };

        Ok(language_list)
    }

    async fn _get_list() -> Result<Vec<String>, reqwest::Error> {
        let mut lang_list: Vec<String> = Vec::new();
        let list = reqwest::get("https://toptal.com/developers/gitignore/api/list")
            .await?
            .text()
            .await?;

        let lines = list.lines();
        for line in lines {
            let line = line
                .split(",")
                .map(|lang| lang.to_owned())
                .collect::<Vec<String>>();

            for lang in line {
                lang_list.push(lang);
            }
        }

        Ok((lang_list))
    }
    pub fn clear() {
        let path = String::from(format!(
            "{}/rignore-cache",
            dirs::cache_dir().unwrap().to_string_lossy()
        ));

        std::fs::remove_dir_all(&path);
    }
}
