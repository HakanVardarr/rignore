use super::List;
use std::fmt::format;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug)]
pub struct Filee {
    pub name: String,
    pub is_valid: bool,
    pub list: List,
    path: String,
}

fn is_valid(name: &&String, list: &List) -> bool {
    list.lang_list.contains(name)
}

impl Filee {
    pub async fn new(name: &String, list: List) -> Result<Self, String> {
        let is_valid = is_valid(&name, &list);

        if is_valid == false {
            eprintln!("ERROR: {name} is not supported.");
            std::process::exit(1);
        };

        Ok(Self {
            name: name.to_lowercase(),
            is_valid,
            list,
            path: String::from(format!(
                "{}/rignore-cache",
                dirs::cache_dir().unwrap().to_string_lossy()
            )),
        })
    }

    pub async fn get_file(&self) -> Result<(), reqwest::Error> {
        let mut string_f = String::new();

        create_dir_all(self.path.clone());

        let mut file = match File::open(format!("{}/rignore-cache-{}", self.path, self.name)) {
            Ok(mut file) => {
                file.read_to_string(&mut string_f);
            }
            Err(e) => {
                let mut new_file =
                    File::create(format!("{}/rignore-cache-{}", self.path, self.name)).unwrap();
                let url = format!(
                    "https://www.toptal.com/developers/gitignore/api/{}",
                    self.name
                );
                let result = match reqwest::get(url).await {
                    Ok(res) => res.text().await.unwrap(),
                    Err(e) => return Err(e),
                };

                new_file.write_all(result.as_bytes());
                string_f = result;
            }
        };

        let mut gitignore = File::create(".gitignore").unwrap();
        gitignore.write_all(string_f.as_bytes());

        Ok(())
    }
}
