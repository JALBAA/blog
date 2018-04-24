#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate markdown;
extern crate comrak;
extern crate websocket;

use std::error::Error;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;
use std::fs::File;
use rocket::response::content;
use comrak::{markdown_to_html, ComrakOptions};
use std::io;
use std::thread;
use std::io::Read;
use websocket::Message;
use websocket::OwnedMessage;
use websocket::sync::Server;

#[get("/article/<cat>/<year>/<month>/<name>")]
fn article(cat: String, year: String, month: String, name: String) -> Option<content::Html<String>> {
    println!("{}", &format!("article/{}/{}/{}/{}", cat, year, month, name));
    let mut file = match File::open(Path::new(&format!("article/{}/{}/{}/{}.md", cat, year, month, name))) {
        Ok(f) => f,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            // return content::Html(String::from("not found"));
            return None
            // panic!("not found");
        }
        Err(_) => {
            // panic!("=.=");
            return Some(content::Html(String::from("not found")));
        }
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(f) => f,
        Err(_) => {
            // panic!(".=.");
            return Some(content::Html(String::from("not found")));
        }
    };
    let r = comrak::markdown_to_html(&contents[..], &ComrakOptions::default());
    // let r = markdown::file_to_html(Path::new(&format!("article/{}/{}/{}/{}.md", cat, year, month, name))).unwrap();
    Some(content::Html(r))
}

fn main() {
    thread::spawn(move || {
        rocket::ignite()
        .mount("/", routes![article])
        .launch();
    });

    let server = Server::bind("127.0.0.1:1234").unwrap();

	for request in server.filter_map(Result::ok) {
		// Spawn a new thread for each connection.
		thread::spawn(move || {
			if !request.protocols().contains(&"rust-websocket".to_string()) {
				request.reject().unwrap();
				return;
			}

			let mut client = request.use_protocol("rust-websocket").accept().unwrap();

			let ip = client.peer_addr().unwrap();

			println!("Connection from {}", ip);

			let message = OwnedMessage::Text("Hello".to_string());
			client.send_message(&message).unwrap();

			let (mut receiver, mut sender) = client.split().unwrap();

			for message in receiver.incoming_messages() {
				let message = message.unwrap();

				match message {
					OwnedMessage::Close(_) => {
						let message = OwnedMessage::Close(None);
						sender.send_message(&message).unwrap();
						println!("Client {} disconnected", ip);
						return;
					}
					OwnedMessage::Ping(ping) => {
						let message = OwnedMessage::Pong(ping);
						sender.send_message(&message).unwrap();
					}
					_ => sender.send_message(&message).unwrap(),
				}
			}
		});
	}
        // Perform another task. Because we have a non-blocking server,
        // this will execute independent of whether someone tried to
        // establish a connection.
    
}
