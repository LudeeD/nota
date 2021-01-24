use anyhow::Result;
use pulldown_cmark::{
    html, Event, Options, Parser,
    Tag::{Heading, Link},
};

use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ParsedNota {
    pub title: String,
    pub contents_digest: String,
    pub links: Vec<String>,
}

pub fn parse_to_html(in_path: PathBuf) -> Result<String> {
    debug!("parse to HTML file {:?}", in_path);

    let buffer: String = fs::read_to_string(&in_path)?
        .parse()
        .expect("TODO remove expects");

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(&buffer, options);

    let mut html_output = String::new();

    html::push_html(&mut html_output, parser);

    Ok(html_output)
}

pub fn parse(in_file: &PathBuf) -> Result<Box<ParsedNota>> {
    debug!("Parsing...");

    //let mut link_range = None;

    let mut title: Option<String> = None;

    let mut links: Vec<String> = Vec::new();

    let mut last_event = None;

    let mut buffer = String::new();

    let mut in_file = File::open(in_file)?;

    in_file.read_to_string(&mut buffer).expect("#TODO change");

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let callback = |a: &str, _b: &str| {
        // For some reason we get the same value in both variables
        debug!("Found Link in File");

        Some((String::from(a), String::from("")))
    };

    let parser = Parser::new_with_broken_link_callback(&buffer, options, Some(&callback));

    for element in parser {
        debug!("Elem: {:?}", &element);

        match &element {
            Event::Start(Heading(1)) => {
                last_event = Some(element);
            }
            Event::Start(Heading(3)) => {
                last_event = Some(element);
            }
            Event::Start(Link(_, _, _)) => {
                last_event = Some(element);
            }
            Event::Text(t) => {
                if let Some(Event::Start(Heading(1))) = last_event {
                    if title.is_none() {
                        title = Some(t.to_string());
                        last_event = None;
                    }
                }
                if let Some(Event::Start(Heading(3))) = last_event {
                    // Parse
                    last_event = None
                }
                if let Some(Event::Start(Link(_, dest, _title))) = last_event {
                    debug!("Link -> {} To -> {}", t, dest);
                    // let dest = dest.to_string();
                    // let uid = Path::new(&dest);
                    // let uid: u32 = uid.file_stem().unwrap().to_str().unwrap().to_string().parse().unwrap();
                    // let link = LinkInfo {details: None , link_to_text: Some(t.to_string()), link_to_uid: Some(uid)};
                    links.push(dest.to_string());
                    last_event = None
                }
            }
            _ => (),
        }
    }

    // let d = String::from("Ai Ai Ai");

    //let demo = vec![Event::Start(Heading(3)), Event::Text(d.try_into().unwrap()), Event::End(Heading(3))];

    //let vecs = iter.chain(demo.into_iter());

    //let buffer = String::new();

    //cmark(vecs, &mut buffer, None);

    //let byte_pos = link_range.unwrap().start;

    //in_file.seek(SeekFrom::Start(byte_pos.try_into().unwrap())).expect("Hum");

    //let text_to_write = format!("{} {}", crate::REVERSE_LINKS_HEADING_LEVEL, crate::REVERSE_LINKS_TEXT);

    //let bytes_written = in_file.write(text_to_write.as_bytes()).expect("Hum...");

    //let new_end_pos = byte_pos + bytes_written;

    //in_file.set_len(new_end_pos.try_into().unwrap()).expect("Hum");

    //in_file.flush().expect("Hum...");

    let title = title.unwrap();

    let contents_digest = String::from("demodigest");

    let info = ParsedNota {
        title,
        contents_digest,
        links,
    };

    debug!("Parsing... Done: {:?}", info);

    Ok(Box::new(info))
}
