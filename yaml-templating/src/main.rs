use walkdir::{WalkDir};
use serde_yaml;
use serde_derive;
use std::fs;
use std::collections::{HashMap, BTreeMap};
use std::path::Path;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, Serialize, Deserialize)]
struct Languages {
    nl: Option<String>,
    en: Option<String>,
    de: Option<String>,
}

type Variables = BTreeMap<String, Languages>;

#[derive(Debug, Serialize, Deserialize)]
struct VariableFile {
    variables: Variables,
}

#[derive(Debug, Serialize, Deserialize)]
struct SourceFile {
    column1: BTreeMap<String, BTreeMap<String, String>>,
    column2: BTreeMap<String, BTreeMap<String, String>>,
}

fn main() {
    let variables = get_all_variables();


    for variable in variables {
        println!("{:?}", variable)
    }

    let _countries = get_countries();
    // for (country_name, data) in countries {
    //     // Todo - substitute variables and output 1 file per language per country
    // }
}

fn get_countries() -> HashMap<String, SourceFile> {
    get_data::<SourceFile>("input/countries")
}

fn get_all_variables() -> Variables {
    get_data::<VariableFile>("input/languages")
        .into_iter()
        .flat_map(|(_, file) | file.variables)
        .collect::<Variables>()
}

fn get_data<T: for<'de> serde::Deserialize<'de>>(path: &str) -> HashMap<String, T> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| !entry.file_type().is_dir())
        .map(|entry| {
            let language = Path::file_stem(entry.path()).unwrap().to_string_lossy().to_string();
            let contents = fs::read_to_string(entry.path()).unwrap();
            let data: T = serde_yaml::from_str(contents.as_str()).unwrap();

            (language, data)
        })
        .collect::<HashMap<String, T>>()
}


