#[macro_use]
extern crate log;
extern crate simple_logger;

#[macro_use]
extern crate serde;
use serde::Serialize;
use serde_json::json;

extern crate bincode;

use warp::Filter;


mod application;
mod service;
mod utility;

extern crate handlebars;
use handlebars::{
    to_json, Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError,
};


use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};


use std::sync::Arc;

struct WithTemplate<T: Serialize> {
    name: &'static str,
    value: T,
}

fn render<T>(template: WithTemplate<T>, hbs: Arc<Handlebars>) -> impl warp::Reply
where
    T: Serialize,
{
    let render = hbs
        .render(template.name, &template.value)
        .unwrap_or_else(|err| err.to_string());
    warp::reply::html(render)
}

#[tokio::main]
async fn main() {

    simple_logger::init_with_level(log::Level::Warn).unwrap();

    let mut hb = Handlebars::new();

    hb.register_template_file("head", &"./frontend/templates/head.hbs")
        .unwrap();

    hb.register_template_file("navbar", &"./frontend/templates/navbar.hbs")
        .unwrap();

    hb.register_template_file("new_nota", &"./frontend/templates/new_nota.hbs")
        .unwrap();

    hb.register_template_file("list_nota", &"./frontend/templates/list_nota.hbs")
        .unwrap();

    // Turn Handlebars instance into a Filter so we can combine it
    // easily with others...
    let hb = Arc::new(hb);

    // Create a reusable closure to render template
    let handlebars = move |with_template| render(with_template, hb.clone());

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let assets = warp::path("assets")
        .and(warp::fs::dir("./frontend/assets"));

    //GET /
    let new_nota_route = warp::path!("new")
        .and(warp::path::end())
        .map(|| WithTemplate {
            name: "new_nota",
            value: json!({"user" : "demo"}),
        })
        .map(handlebars.clone());

    let list_nota_route = warp::path!("notas") 
        .and(warp::path::end())
        .map(|| WithTemplate {
            name: "list_nota",
            value: json!({"notas" : {"blocks": {"uid": "1", "title":"demo demo", "timestamp" : "1234"}} }),
        })
        .map(handlebars.clone());

    let app = warp::get().and(hello.or(assets).or(new_nota_route).or(list_nota_route));

    warp::serve(app).run(([127, 0, 0, 1], 3030)).await;
}
