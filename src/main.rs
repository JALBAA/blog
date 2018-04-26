#![feature(plugin)]
#![plugin(rocket_codegen)]


extern crate rocket;
extern crate markdown;
extern crate comrak;
extern crate websocket;
extern crate rocket_contrib;
#[macro_use] 
extern crate serde_derive;

mod router;

use router::article;

use rocket_contrib::Template;


fn main() {
	rocket::ignite()
	.mount("/article", article::routers())
	.mount("/static", routes![router::get_file])
	.attach(Template::fairing())
	.launch();
}