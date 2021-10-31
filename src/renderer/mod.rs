use log::{debug, error, info};
use handlebars::Handlebars;
use serde_json::json;
use pulldown_cmark::{Parser, Options, html};
use std::fs::{self, File};
use std::io::prelude::*;

use super::NotaBuilder;
use super::NotaIndex;

mod theme;

use theme::{ TEMPLATE, CSS };

pub struct Renderer<'a> {
    hbs: Handlebars<'a> ,
}

impl<'a> Renderer<'a> {

    pub fn new() -> Renderer<'a> {
        let mut hbs = Handlebars::new();

        hbs.register_template_string("demo", String::from_utf8(TEMPLATE.to_vec()).expect("TODO")).expect("TODO");

        Renderer { hbs }
    }

    pub fn render(&self, builder: &NotaBuilder, index: &NotaIndex) {

        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);

        // Write to String buffer.

        for nota in &index.nota_store {
            let mut html_output = String::new();

            let markdown = fs::read_to_string(&nota.path).expect("Something went wrong reading the file");

            let parser = Parser::new_ext(&markdown, options);

            html::push_html(&mut html_output, parser);

            let render = self.hbs.render("demo", &json!({"content": html_output})).expect("TODO");

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


    }

}