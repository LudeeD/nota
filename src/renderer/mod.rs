use log::{debug, error, info};
use handlebars::Handlebars;
use serde_json::json;
use pulldown_cmark::{Parser, Options, html};
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::PathBuf;

use super::NotaBuilder;
use super::NotaIndex;

mod theme;

use theme::{ TEMPLATE_NOTA, TEMPLATE_INDEX, CSS };

pub struct Renderer<'a> {
    hbs: Handlebars<'a> ,
}

impl<'a> Renderer<'a> {

    pub fn new() -> Renderer<'a> {
        let mut hbs = Handlebars::new();

        hbs.register_template_string("nota", String::from_utf8(TEMPLATE_NOTA.to_vec()).expect("TODO")).expect("TODO");
        hbs.register_template_string("index", String::from_utf8(TEMPLATE_INDEX.to_vec()).expect("TODO")).expect("TODO");

        Renderer { hbs }
    }

    pub fn render(&self, builder: &NotaBuilder, index: &NotaIndex) {

        // create css folder
        let mut static_folder = builder.output_folder.clone();
        static_folder.push("static");
        fs::create_dir_all(&static_folder).expect("TODO");

        static_folder.push("style.css");


        let mut file = File::create(static_folder).expect("TODO");
        file.write_all(CSS).expect("TODO");


        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);

        // Write to String buffer.

        for nota in &index.nota_store {
            let mut html_output = String::new();

            let markdown = fs::read_to_string(&nota.path).expect("Something went wrong reading the file");

            let parser = Parser::new_ext(&markdown, options);

            html::push_html(&mut html_output, parser);


            let parent = match &nota.rel_path.parent() {
                Some(par) => {
                    if par.as_os_str() != "" { "../" } 
                    else { "" }
                },
                _ => ""
            };

            let render = self.hbs.render("nota", &json!({"content": html_output, "parent": parent})).expect("TODO");

            let file_name = &nota.rel_path.file_name().expect("TODO");
            let mut output = builder.output_folder.clone();
            output.push(&nota.rel_path);
            output.pop();
            fs::create_dir_all(&output).expect("TODO");
            output.push(file_name);
            output.set_extension("html");
            debug!("Writing to {:?}", output);

            let mut file = File::create(output).unwrap();
            file.write_all(&render.as_bytes()).expect("TODO");
        }


        let demo : Vec<PathBuf> = index.nota_store.iter()
            .map(|nota| {
                let mut path = nota.rel_path.clone();
                path.set_extension("html"); 
                path
            }).collect();
        let render = self.hbs.render("index", &json!({"people": demo})).expect("TODO");
        let mut index_file = builder.output_folder.clone();
        index_file.push("index.html");

        let mut file = File::create(index_file).unwrap();
        file.write_all(&render.as_bytes()).expect("TODO");
    }

}