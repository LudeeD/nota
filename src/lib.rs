#[macro_use]
extern crate log;
extern crate serde;
extern crate handlebars;
extern crate pulldown_cmark;
extern crate walkdir;

use std::fs;
use std::fs::File;
use std::path::{ PathBuf};
use std::time::SystemTime;

use anyhow::Result;

use serde::{Deserialize, Serialize};

use std::env;

const ENVS_ERROR : &str = "envs error";
const OTHER_ERROR : &str = "error";

use pulldown_cmark::{Parser, Event, Tag::Link, html};
use std::io::Read;

use handlebars::{to_json, Handlebars};
use std::io::prelude::*;
use std::collections::BTreeMap;

pub fn generate_nota(handlebars: &handlebars::Handlebars, db: &HashMap<PathBuf, Vec<PathBuf>>, nota: &PathBuf) {

    let nota_folder =  PathBuf::from(env::var("NOTA_HOME").expect(ENVS_ERROR)).canonicalize().expect(OTHER_ERROR);

    let filename = nota.file_name().unwrap();
    let mut output_file = PathBuf::from(env::var("NOTA_OUTPUT_FOLDER").expect(ENVS_ERROR));
    output_file.push(filename);
    output_file.set_extension("html");


    info!("Parsing {:?} to HTML", nota);

    let buffer: String = fs::read_to_string(&nota).expect("io error");
    let parser = Parser::new(&buffer);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);


    let mut data = BTreeMap::new();
    data.insert("body".to_string(), to_json(html_output));
    let dt = SystemTime::now();
    data.insert("lastmodified".to_string(), to_json(format!("{:?}",dt)));

    let links = db.get(nota);

    let mut refs = Vec::new();
    
    if links.is_some() {
        let links = links.unwrap();

        refs = links.iter().map(|value| {
            let mut href = value.clone();
            href.set_extension("html");
            let href = href.strip_prefix(&nota_folder).expect(OTHER_ERROR);
            format!("<a href={:?}>{:?}</a>", href, value)
        }).collect();
    }

    data.insert("links".to_string(), to_json(refs));

    let mut output_file = File::create(output_file).unwrap();

    output_file.write_all(handlebars.render("entry", &data).unwrap().as_bytes()).expect(OTHER_ERROR);
}

pub fn index_nota(path: &PathBuf, db: &mut HashMap<PathBuf,Vec<PathBuf>>) {

    info!("Parsing for links {:?}", &path);
    let mut f = File::open(path).expect("io error");
    let mut buffer = String::new();

    f.read_to_string(&mut buffer).expect("io error");
    let parser = Parser::new(&buffer);

    let _parser = parser.for_each(|event| 
        if let Event::Start(Link(_link_type, destination, _title)) = event {
            let dest = PathBuf::from(destination.as_ref());

            match dest.canonicalize() {
                Ok(dest_canon) => {
                   info!("{:?} -> {:?} [{:?}]", path, &dest, &dest_canon);

                    let new_value = match db.get(&dest_canon) {
                        Some(v) => {
                            let mut new_v = v.clone();
                            new_v.sort();
                            if let Err(_) = v.binary_search(path) {
                                new_v.push(path.clone());
                            };
                            new_v
                        },
                        None => {
                            vec![path.clone()]
                        }
                    };

                    db.insert(dest_canon, new_value);
                },
                Err(_) => ()
            }
        }
    );
}


#[derive(Deserialize, Serialize)]
struct Config {
    output: Output,
}

#[derive(Deserialize, Serialize)]
struct Output {
    folder: PathBuf,
    template: PathBuf,
    exported: Option<SystemTime>,
}

pub fn create_default_config_file() {
    let config = Config {
        output : Output {
            folder : PathBuf::from("output"),
            template : PathBuf::from("template.html"), 
            exported  : Some(SystemTime::now())
        }
    };

    let toml = toml::to_string(&config).unwrap();
    let mut file = File::create("nota.toml").expect("damn");
    file.write_all(toml.as_bytes());
}

use std::collections::HashMap;

pub fn generate() {

    let mut path = env::current_dir().expect(ENVS_ERROR);
    path.canonicalize().expect(OTHER_ERROR);

    env::set_var("NOTA_HOME", path.to_str().unwrap());

    let mut config_path = path.clone();
    config_path.push("nota.toml");

    if ! config_path.exists() {
        create_default_config_file();
    }

    let config = fs::read_to_string(config_path).expect(OTHER_ERROR);
    let config: Config = toml::from_str(&config).expect(OTHER_ERROR);

    env::set_var("NOTA_OUTPUT_FOLDER", config.output.folder);

    let folder = fs::read_dir(&path).expect(OTHER_ERROR);
    let entries : Vec<PathBuf> = folder
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| { 
            entry.path().is_file() && 
            entry.path().extension().is_some() && 
            entry.path().extension().unwrap() == "md" })
        .map(|entry| entry.path().canonicalize())
        .filter_map(Result::ok)
        .collect();
    
    let mut notas: HashMap<PathBuf, Vec<PathBuf>>= HashMap::new();

    for entry_path in &entries {
        index_nota(entry_path, &mut notas);
    }

    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("entry", &config.output.template)
        .expect("damn");

    for entry_path in &entries {
        generate_nota(&handlebars, &notas, entry_path);
    };
}