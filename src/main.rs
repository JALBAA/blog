#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate markdown;

use std::path::Path;
use rocket::response::content;

#[get("/article/<cat>/<year>/<month>/<name>")]
fn article(cat: String, year: String, month: String, name: String) -> Result<content::Html<String>, String> {
    println!("{}", &format!("article/{}/{}/{}/{}", cat, year, month, name));
    let r = markdown::file_to_html(Path::new(&format!("article/{}/{}/{}/{}.md", cat, year, month, name))).unwrap();
    Ok(content::Html(r))
}

fn main() {
     rocket::ignite()
     .mount("/", routes![article])
     .launch();
}