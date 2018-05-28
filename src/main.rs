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
extern crate tera;


use std::fmt::Display;
use router::article;
use rocket_contrib::Template;
use std::path::Path;
use std::path::PathBuf;
use rocket::response::NamedFile;
use rocket::fairing::AdHoc;
use std::sync::atomic::{AtomicUsize, Ordering};

mod router;
mod global;


#[get("/<file..>")]
pub fn get_file (file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[derive(Debug)]
pub struct HitCount {
    count: AtomicUsize
}
#[derive(Debug)]
pub struct A {
	a: i32
}

use std::sync::{Mutex, Arc};
fn main() {

	let nav_info = Arc::new(Mutex::new(global::get_nav_info()));

	let counter = Arc::new(Mutex::new(1));
	rocket::ignite()
	.manage(counter)
	.manage(nav_info)
	.mount("/", router::routers())
	.mount("/article", article::routers())
	.mount("/static", routes![get_file])
	.attach(Template::fairing())
	.launch();
}