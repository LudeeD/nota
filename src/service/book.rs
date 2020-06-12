use crate::utility::error::Upsie;

use std::fs;

use std::path::{ PathBuf};

use std::io::Write;

use comrak::{parse_document, format_html, Arena, ComrakOptions};

use walkdir::WalkDir;

use crate::service::structure;

pub fn generate() -> Result<(), Upsie>{

    let nota_path = structure::main_folder()?; 
    let book_path = PathBuf::from(structure::book_folder()?);

    let top_of_file ="<!DOCTYPE html><html><head><meta charset=\"utf-8\"><title>";
    let top_of_file2="</title><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><link href=\"css/style.css\" rel=\"stylesheet\"></head><body>";
    let bottom_of_file="</body></html>";

    let arena = Arena::new();

    for entry in WalkDir::new(nota_path).follow_links(false).into_iter().filter_map(|e| e.ok()) {
   
        let fname = String::from(entry.file_name().to_string_lossy());

        if !(fname.ends_with(".md")) {
            continue;
        }
        
        let input = std::fs::read_to_string(entry.into_path()).unwrap();

        let root = parse_document(&arena, &input, &ComrakOptions::default());

        let mut html = vec![];

        format_html(root, &ComrakOptions::default(), &mut html).unwrap();
            
        let mut html_path = book_path.clone();
        html_path.push(fname);
        html_path.set_extension("html");

        let mut output_file = fs::File::create(html_path).unwrap();


        output_file.write_all(top_of_file.as_bytes())?;
        output_file.write_all(top_of_file2.as_bytes())?;
        output_file.write_all(&html)?;
        output_file.write_all(bottom_of_file.as_bytes())?;
    }
    
    return Ok(());
}