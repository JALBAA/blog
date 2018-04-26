use std::path::Path;
use std::io;
use std::fs::File;
use std::fs::{self, DirEntry};
use comrak;
use comrak::{ComrakOptions};
use std::io::Read;
use rocket_contrib::Template;
use rocket::Route;
use std::fs::read_dir;

/*
struct Months {
    months: Vec<>
}
struct Year {
    years: Vec<Months>,
}

*/
use std::collections::HashMap;
#[derive(Serialize, Debug)]
struct Article {
    cat: Option<String>,
    year: Option<String>,
    month: Option<String>,
    name: Option<String>,
}
impl Article {
    fn new () -> Article {
        Article {
            cat: None,
            year: None,
            month: None,
            name: None,
        }
    }
    
}
// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, files: &mut Vec<Article>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, files)?;
            } else {
                let mut article = Article::new();
                let path = path.to_str().unwrap();
                let mut paths: Vec<&str> = path.split("\\").collect();
                paths.remove(0);
                let mut iter = paths.iter().rev();
                if let Some(name) = iter.next() {
                    article.name = Some(String::from(*name));
                }
                if let Some(month) = iter.next() {
                    article.month = Some(String::from(*month));
                }
                if let Some(year) = iter.next() {
                    article.year = Some(String::from(*year));
                }
                if let Some(cat) = iter.next() {
                    article.cat = Some(String::from(*cat));
                }
                let file_path = String::from(path);
                let file_path = file_path.replace(".md", "");
                files.push(article);
            }
        }
    }
    Ok(())
}


#[derive(Serialize, Debug)]
struct Name {
	markdown_body: String,
    articles: Vec<Article>,
}

pub fn routers () -> Vec<Route> {
    routes![get_article, fuck]
}

#[get("/fuck")]
pub fn fuck () -> String{
    get_file_list();
    String::from("123")
}

fn get_file_list () -> Option<Vec<Article>> {
    let dir = Path::new("article");
    let mut files = vec![];
    visit_dirs(&dir, &mut files).unwrap();
    Some(files)
}

#[get("/<cat>/<year>/<month>/<name>")]
pub fn get_article(cat: String, year: String, month: String, name: String) -> Option<Template> {
    println!("{}", &format!("article/{}/{}/{}/{}", cat, year, month, name));
    let mut file = match File::open(Path::new(&format!("article/{}/{}/{}/{}.md", cat, year, month, name))) {
        Ok(f) => f,
        Err(_) => {
            return None;
        }
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(f) => f,
        Err(_) => {
            return None;
        }
    };
    let r = comrak::markdown_to_html(&contents[..], &ComrakOptions::default());
    let context = Name {
		markdown_body: r,
        articles: get_file_list().unwrap(),
	};
	Some(Template::render("index", &context))
}

