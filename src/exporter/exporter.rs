use anyhow::{Context, Result};
use std::path::PathBuf;

use std::collections::BTreeMap;
use handlebars::Handlebars;

use crate::util;
use crate::parser;
use crate::exporter::templates;

use std::fs;
use std::io::Write;
use std::fs::File;

use walkdir::WalkDir;
//
//use std::path::{ PathBuf};
//
//
//use comrak::{parse_document, format_html, Arena, ComrakOptions};
//
//use walkdir::WalkDir;
//
//use crate::service::structure;
//
// pub fn generate() -> Result<()>{
// 
//     let nota_path = util::envs::main_folder();
//     let book_path = PathBuf::from(util::envs::export_folder());
// 
//     let mut handlebars = Handlebars::new();
// 
//     for entry in WalkDir::new(nota_path).follow_links(false).into_iter().filter_map(|e| e.ok()) {
//    
//         let fname = String::from(entry.file_name().to_string_lossy());
// 
//         if !(fname.ends_with(".md")) {
//             continue;
//         }
//         
//         let input = std::fs::read_to_string(entry.into_path()).unwrap();
// 
//         let root = parse_document(&arena, &input, &ComrakOptions::default());
// 
//         let mut html = vec![];
// 
//         format_html(root, &ComrakOptions::default(), &mut html).unwrap();
//             
//         let mut html_path = book_path.clone();
//         html_path.push(fname);
//         html_path.set_extension("html");
// 
//         let mut output_file = fs::File::create(html_path).unwrap();
// 
// 
//         output_file.write_all(top_of_file.as_bytes())?;
//         output_file.write_all(top_of_file2.as_bytes())?;
//         output_file.write_all(&html)?;
//         output_file.write_all(bottom_of_file.as_bytes())?;
//     }
//     
//     return Ok(());
// }

pub fn init() -> Result<()> {

    let export_folder = util::envs::export_folder();
    debug!("creating exporter folder in {:?}", export_folder);
    util::filesystem::create_folder(&export_folder);

    Ok(())
}

pub fn export(file_path: Option<PathBuf>) -> Result<()> {

    let mut handlebars = Handlebars::new();

    handlebars.register_template_string("entry", templates::entry)?;
    handlebars.register_template_string("index", templates::index)?;

    match file_path {
        Some(f) => export_single_file(f, &handlebars),
        None => export_all_folder(&handlebars) 
    }?;

    export_index(&handlebars)

    // let mut a = PathBuf::from("C:\\Users\\Luís Silva\\Desktop\\NOTA\\1.md");

    // export_single_file(a, handlebars)
}

#[derive(Serialize)]
struct Person {
  link: String,
  title: String,
}

fn export_index(handlebars: & Handlebars) -> Result<()> { 

    let index = crate::index::list::load().expect("TODO remove expects | load index");

    let mut data_ext = BTreeMap::new();

    let mut list = vec![];

    for entry in index.into_iter() {
        let mut link = entry.file_path;
        link.set_extension("html");
        let link = String::from(link.file_name().unwrap().to_str().unwrap());
        let title = match entry.original_title {
            Some(s) => s,
            None => String::from("No title")
        };
        list.push(Person{link, title})
    }

    data_ext.insert("nav", list);

    let export_folder = util::envs::export_folder();
    let mut index_file = PathBuf::from(export_folder);
    index_file.push("index");
    index_file.set_extension("html");

    let mut index_file = File::create(index_file).unwrap();

    index_file.write_all(handlebars.render("index", &data_ext).unwrap().as_bytes()).expect("TODO remove expects");

    Ok(())
}

fn export_all_folder( handlebars: & Handlebars ) -> Result<()> {
    debug!("exporting all folder");

    let nota_path = util::envs::main_folder();

    for entry in WalkDir::new(nota_path).follow_links(false).into_iter().filter_map(|e| e.ok()) {
        let fname = String::from(entry.file_name().to_string_lossy());
 
         if !(fname.ends_with(".md")) {
             continue;
         }

         export_single_file(entry.into_path(), handlebars);
    }

    Ok(())
}

fn export_single_file(mut file_path: PathBuf, handlebars: & Handlebars) -> Result<()> {
    debug!("exporting file {:?}", file_path);

    let mut data = BTreeMap::new();

    let mut out_file = PathBuf::from(file_path.clone());

    let name = file_path.file_name().expect("No file name").to_str().expect("Failed conversion to str");

    let name = String::from(name);

    out_file.pop();
    out_file.push("export");
    out_file.push(name);
    out_file.set_extension("html");

    debug!("export file: {:?} to {:?}", &file_path, &out_file);

    let a = parser::parse_to_html(file_path)?;

    debug!("{:?}", &a);

    data.insert("body".to_string(), a);

    let mut output_file = File::create(out_file).unwrap();

    output_file.write_all(handlebars.render("entry", &data).unwrap().as_bytes()).expect("TODO remove expects");

    Ok(())
}