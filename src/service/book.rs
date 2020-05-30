use std::path::Path;
use std::fs;
use std::ffi::{OsStr};
use std::collections::BTreeMap;


use comrak::{markdown_to_html, ComrakOptions};

use handlebars::{
    to_json, Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError,
};

pub fn generate() {
    let mut data = BTreeMap::new();
    data.insert("title".to_string(), "QUE MERRRDA".to_string());

    let mut handlebars = Handlebars::new();
    let mut source_template = fs::File::open("/home/ludee/Documents/nota/book/templates/nota.hbs").unwrap();
    let mut output_file = fs::File::create("/home/ludee/Documents/nota/book/demo.html").unwrap();
    handlebars.render_template_source_to_write(&mut source_template, &data, &mut output_file).unwrap();

    //let mut source_template = fs::File::open(&"/home/ludee/Documents/nota/book/templates/nota.hbs").expect("No template File");

    //let nota_dir = "/home/ludee/Documents/nota";

    // https://rust-lang-nursery.github.io/rust-cookbook/file/dir.html
    // see also for metadata like modified time
    //for entry in fs::read_dir(nota_dir).expect("No nota dir!") {
    //    let entry = entry.unwrap();
    //    let path = entry.path();
    //    let md_extension = OsStr::new("md");

    //    match path.extension() {
    //        Some( ext ) => {
    //                if ext == "md" {
    //                    info!("{:?}", path);
    //                    //let string_html = markdown_to_html(&fs::read_to_string(path).expect("Bad nota"),  &ComrakOptions::default());
    //                    //handlebars.register_template_string("page", string_html).is_ok();
    //                    info!("Should be creating things");
    //                } 
    //        },
    //        _ => {}
    //    }

    //}

    
}