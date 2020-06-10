use std::fs;

use std::path::PathBuf;

use std::io::{Error, Write};

use comrak::{parse_document, format_html, Arena, ComrakOptions};

use walkdir::WalkDir;

pub fn generate(nota_path: &PathBuf, book_path: &PathBuf) -> Result<(), Error>{

    let arena = Arena::new();

    let (mut input, mut root, mut output_file);
    let mut html = vec![];

    for entry in WalkDir::new(nota_path).follow_links(false).into_iter().filter_map(|e| e.ok()) {

        let fname = String::from(entry.file_name().to_string_lossy());

        if !(fname.ends_with(".md")) {
            continue;
        }
        
        input = std::fs::read_to_string(entry.into_path()).unwrap();

        root = parse_document(&arena, &input, &ComrakOptions::default());

        html.clear(); 

        format_html(root, &ComrakOptions::default(), &mut html).unwrap();
            
        let mut html_path = book_path.clone();
        html_path.push(fname);
        html_path.set_extension("html");

        output_file = fs::File::create(html_path).unwrap();

        output_file.write_all(&html)?;
    }
    
    return Ok(());
}