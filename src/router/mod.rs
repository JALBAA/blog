pub mod article;

use toml;
use rocket_contrib::Template;
use rocket::Route;
use std::io::Read;
use std::path::Path;
use std::fs::File;
use tera::Context;

use rocket::State;
use rocket::http::uri::URI;
use std::sync::{Mutex, Arc};
use global::NavInfo;

#[derive(Debug, Deserialize, Serialize)]
struct Config {
	production: Option<ConfigEnv>,
	development: Option<ConfigEnv>,
}
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
struct ConfigEnv {
	menu_path: String,
    books: Vec<Book>,
	articles: Vec<Article>
}
#[derive(Debug, Deserialize, Serialize)]
struct Article {
	tag: String,
	years: Vec<Year>,
}
#[derive(Debug, Deserialize, Serialize)]
struct Year {
	year: String,
	months: Vec<Month>,
}
#[derive(Debug, Deserialize, Serialize)]
struct Month {
    month: String,
	articles: Vec<ArticleData>,
}
#[derive(Debug, Deserialize, Serialize)]
struct ArticleData {
    name: String,
    portrait: String,
    intro: String,
}
#[derive(Debug, Deserialize, Serialize)]
struct Book {
    name: String,
    path: String,
}
use std::io::prelude::*;

pub fn routers () -> Vec<Route> {
    routes![index, get_books, count2, nav]
}

#[get("/nav")]
pub fn nav<'r> (uri: &'r URI) -> String {
    println!("{}", &uri.as_str());
    // String::from("ss")
    String::from("a")
}



#[get("/count2")]
pub fn count2 (count: State<Arc<Mutex<i32>>>) -> String {
    let mut a = count.lock().unwrap();
    *a += 1;
    println!("{:?}", *a);
    format!("{}", *a)
}

fn set_nav_info<'a> (context:&'a mut Context, uri_info: &URI, nav:State<Arc<Mutex<NavInfo>>> ) -> &'a mut Context {
    context.add("uri", &uri_info.as_str());
    context.add("nav_info", &*nav.lock().unwrap());
    context
}

#[get("/")]
pub fn index (uri_info: &URI , nav: State<Arc<Mutex<NavInfo>>>) -> Option<Template> {
    let mut context: Context = Context::new();
    
    let mut f = File::open("./Config.toml").unwrap();
	let mut contents = String::new();

	f.read_to_string(&mut contents);
    // println!("With text:\n{}", contents);
	let c:Config = toml::from_str(&contents[..]).unwrap();

	match c.production {
		Some (c) => {
            context.add("articles", &c.articles)
		},
		None => ()
	};
    set_nav_info(&mut context, uri_info, nav);
	Some(Template::render("index", context))
}

#[get("/books")]
pub fn get_books (uri_info: &URI, nav: State<Arc<Mutex<NavInfo>>>) -> Option<Template> {
	let path = Path::new("Config.toml");
	let mut file = match File::open(&path) {
		Ok(f) => f,
		Err(_) => {
			return None
		},
	};
	let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(f) => f,
        Err(_) => {
            return None
        },
    };
	let config: Config = toml::from_str(&contents[..]).unwrap();
    // println!("ffffffffffffffffffff{:?}",config.production);
    match config.production {
        Some(c) => {
            let mut context: Context = Context::new();
            context.add("content", &c);
            set_nav_info(&mut context, uri_info, nav);
	        Some(Template::render("books", context))
        },
        None => None,
    }
}