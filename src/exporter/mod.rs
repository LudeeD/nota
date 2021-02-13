// External crates
use anyhow::Result;
use chrono::{TimeZone, Utc};
use handlebars::Handlebars;

// STD imports
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::Write;
use std::path::{PathBuf, Path};
use std::time::SystemTime;

// NOTA imports
use crate::index::list::IndexEntry;
use crate::parser;
use crate::util;

pub fn init(export_folder: String, template_folder: String) -> Result<()> {
    info!("Creating export folder {:?}", export_folder);
    info!("Searching templates in {:?}", template_folder);

    util::envs::set_export_folder(&export_folder);
    if ! Path::new(&export_folder).exists() {
        util::filesystem::create_folder(&export_folder)?;
    }
    util::envs::set_template_folder(&template_folder);

    Ok(())
}

pub fn export_registered(list: &[IndexEntry]) -> Result<()> {
    debug!("list {:?}", list);
    let mut templates_nota = PathBuf::from(util::envs::template_folder());
    templates_nota.push("nota.html");

    let export_folder = util::envs::export_folder();
    let mut data = BTreeMap::new();

    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("entry", templates_nota)
        .expect("damn");

    for item in list {
        debug!("Item {:?}", item);
        let item_path = item.path.clone();
        let original_file = File::open(&item_path)?;
        let metadata = original_file.metadata()?;
        let mut out_file = PathBuf::from(&export_folder);

        let name = item_path.file_name().unwrap().to_str().unwrap();

        out_file.push(name);
        out_file.set_extension("html");

        debug!("export file: {:?} to {:?}", &item.path, &out_file);

        let a = parser::parse_to_html(item_path)?;

        data.insert("body".to_string(), a);

        let dt = Utc.timestamp(
            metadata
                .modified()?
                .duration_since(SystemTime::UNIX_EPOCH)?
                .as_secs()
                .try_into()
                .unwrap(),
            0,
        );

        data.insert("lastmodified".to_string(), dt.to_rfc2822());

        let mut output_file = File::create(out_file).unwrap();

        output_file
            .write_all(handlebars.render("entry", &data).unwrap().as_bytes())
            .expect("TODO remove expects");
    }

    Ok(())
}