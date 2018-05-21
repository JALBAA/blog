#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate markdown;
extern crate comrak;
extern crate websocket;
extern crate rocket_contrib;
#[macro_use] 
extern crate serde_derive;
extern crate toml;



use router::article;
use rocket_contrib::Template;
use std::path::Path;
use std::path::PathBuf;
use rocket::response::NamedFile;
use std::fs::File;


mod router;


#[get("/<file..>")]
pub fn get_file (file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}



fn main() {
	rocket::ignite()
	.mount("/", router::routers())
	.mount("/article", article::routers())
	.mount("/static", routes![get_file])
	.attach(Template::fairing())
	.launch();
}