pub mod article;

use toml;
use rocket_contrib::Template;
use rocket::Route;
use std::io::Read;
use std::path::Path;
use std::fs::File;

pub fn routers () -> Vec<Route> {
    routes![index, get_books]
}


#[derive(Serialize, Debug)]
struct Ctx {
}
#[get("/")]
pub fn index () -> Option<Template> {
    let context = Ctx{};
	Some(Template::render("index", &context))
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
pub fn get_books () -> Option<Template> {
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
	        Some(Template::render("books", c))
        },
        None => None,
    }
}