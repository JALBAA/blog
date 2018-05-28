pub mod article;

use toml;
use rocket_contrib::Template;
use rocket::Route;
use std::io::Read;
use std::path::Path;
use std::fs::File;
use tera::Context;
use global;
use std::path::PathBuf;
pub fn routers () -> Vec<Route> {
    routes![index, get_books, count2, nav]
}
use rocket::State;
use rocket::http::uri::URI;
use HitCount;
use std::sync::atomic::{Ordering};
use std::sync::{Mutex, Arc};
use global::NavInfo;
use global::NavItem;

use rocket::Request;

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
    set_nav_info(&mut context, uri_info, nav);
	Some(Template::render("index", context))
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
	production: Option<ConfigEnv>,
	development: Option<ConfigEnv>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ConfigEnv {
	menu_path: String,
    books: Vec<Book>,
}
#[derive(Debug, Deserialize, Serialize)]
struct Book {
    name: String,
    path: String,
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