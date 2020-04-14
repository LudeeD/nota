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

    hb.register_template_file("title", &"./frontend/title.hbs");

    hb.register_template_file("template.html", &"./frontend/index.hbs")
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
    let route = warp::get()
        .and(warp::path::end())
        .map(|| WithTemplate {
            name: "template.html",
            value: json!({"user" : "demo"}),
        })
        .map(handlebars);

    let app = hello.or(assets).or(route);

    warp::serve(app).run(([127, 0, 0, 1], 3030)).await;
}
