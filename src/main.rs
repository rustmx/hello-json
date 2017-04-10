extern crate rand;
extern crate iron;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use rand::Rng;
use rustc_serialize::json;

const MY_URL: &'static str = "localhost:3009";

#[derive(RustcDecodable, RustcEncodable)]
struct JsonResponse{
    response: String
}

fn pick_response() -> String {
    let num = rand::thread_rng().gen_range(1,6);

    let response = match num {
        1 => "Hola Mundo",
        2 => "¿Viste el programa añoche?",
        3 => "Buen clima.",
        4 => "á é í ó ú ñ",
        5 => "Más letras especiales ü ö ç $ &",
        _ => ""
    };

    response.to_string()
}

fn main() {
    println!("Listening on http://{}", MY_URL);
    Iron::new(|_: &mut Request| {
        let content_type = "application/json;charset=utf-8".parse::<Mime>().unwrap();
        let response = JsonResponse { response: pick_response() };
        let out = json::encode(&response).unwrap();
        Ok(Response::with((content_type, status::Ok, out)))
    }).http(MY_URL).unwrap();
}
