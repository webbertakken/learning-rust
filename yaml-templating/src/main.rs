use walkdir::{WalkDir};
use serde_yaml;
use serde_derive;
use std::fs;
use std::collections::{HashMap, BTreeMap};
use std::path::Path;
use regex::Regex;
use serde_derive::Deserialize;
use serde_derive::Serialize;


#[derive(Debug, Serialize, Deserialize)]
struct Config {
    languages: Vec<String>
}

type Variables = BTreeMap<String, BTreeMap<String, String>>;

#[derive(Debug, Serialize, Deserialize)]
struct VariableFile {
    variables: Variables,
}

#[derive(Debug, Serialize, Deserialize)]
struct DefinitionsFile {
    column1: BTreeMap<String, BTreeMap<String, String>>,
    column2: BTreeMap<String, BTreeMap<String, String>>,
}

fn main() {
    let variables = get_consolidated_variables();
    let countries = get_countries_data();
    let languages = get_config().languages;

    for (country, country_data) in countries {
        for language in &languages {
            let mut contents = country_data.clone();
            for (variable, value) in &variables {
                let replacement = if value.contains_key(language) {
                    value.get(language).unwrap()
                } else {
                    value.get("en").unwrap()
                };

                let replacer = Regex::new( format!(r"\{{\{{\s?{}\s?\}}\}}", variable).as_str()).unwrap();

                contents = replacer.replace_all(contents.as_str(), replacement).to_string();
            }

            // todo: parse into definition file, process specific rules like "only", then serialise again before saving

            let output_path = format!("dist/{}-{}.yaml", country, language);
            fs::write(&output_path, &contents).expect(format!("Unable to write to {}", output_path).as_str());
            println!("{} - {}", output_path, contents)
        }
    }
}

fn get_countries_data() -> HashMap<String, String> {
    get_data_from_files_in_path("input/countries")
}

fn get_consolidated_variables() -> Variables {
    get_data_from_files_in_path("input/languages")
        .into_iter()
        .map(|(name, data) | (name, deserialize_data::<VariableFile>(data)))
        .flat_map(|(_, file) | file.variables)
        .collect::<Variables>()
}

fn get_config() -> Config {
    let contents = get_data("config.yaml".as_ref());

    deserialize_data::<Config>(contents)
}

fn deserialize_data<T: for<'de> serde::Deserialize<'de>>(data: String) ->  T {
    serde_yaml::from_str(data.as_str()).unwrap()
}

fn get_data_from_files_in_path(path: &str) -> HashMap<String, String> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| !entry.file_type().is_dir())
        .map(|entry| {
            let name = Path::file_stem(entry.path()).unwrap().to_string_lossy().to_string();
            let contents = get_data(entry.path());

            (name, contents)
        })
        .collect::<HashMap<String, String>>()
}

fn get_data(path: &Path) -> String {
    fs::read_to_string(path).unwrap()
}

