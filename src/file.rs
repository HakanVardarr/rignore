use super::List;
use std::fmt::format;
use std::fs::create_dir_all;
use std::io::{BufReader, BufWriter, Read, Write};

pub struct File {
    pub name: String,
    pub list: List,
    path: String,
}

impl File {
    pub fn new(name: &String, list: List) -> Result<Self, String> {
        if list.lang_list.contains(name) == false {
            return Err(format!("{name} is not supported."));
        };

        Ok(Self {
            name: name.to_lowercase(),
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
        let mut file =
            match std::fs::File::open(format!("{}/rignore-cache-{}", self.path, self.name)) {
                Ok(mut file) => {
                    let mut file = BufReader::new(file);
                    file.read_to_string(&mut string_f);
                }
                Err(e) => {
                    let mut file = BufWriter::new(
                        std::fs::File::create(format!("{}/rignore-cache-{}", self.path, self.name))
                            .unwrap(),
                    );

                    let url = format!(
                        "https://www.toptal.com/developers/gitignore/api/{}",
                        self.name
                    );
                    let result = match reqwest::get(url).await {
                        Ok(res) => res.text().await.unwrap(),
                        Err(e) => return Err(e),
                    };

                    file.write_all(result.as_bytes());
                    string_f = result;
                }
            };

        let mut gitignore = BufWriter::new(std::fs::File::create(".gitignore").unwrap());
        gitignore.write_all(string_f.as_bytes());

        Ok(())
    }
}
