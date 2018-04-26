pub mod article;
use std::path::Path;
use std::path::PathBuf;
use rocket::response::NamedFile;

#[get("/<file..>")]
pub fn get_file (file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}