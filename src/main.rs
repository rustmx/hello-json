// rand nos va a ayudar a generar números aleatorios.
extern crate rand;
// iron es un web framework flexible orientado a middleware.
extern crate iron;
// rustc_serialize nos va a ayudar a serializar una struct a JSON.              
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use rand::Rng;
use rustc_serialize::json;

/*
Declaración de un constante con el servidor y puerto de nuestro servicio.
*/
const MY_URL: &'static str = "localhost:3009";

/*
La siguiente estructura, JsonResponse, debe poder ser codificable o "encodable".
Es por ello que le antecedemos un derive con RustcEncodable.
*/
#[derive(RustcDecodable, RustcEncodable)]
struct JsonResponse{
    response: String
}


/*
La función pick_response() crea un respuesta aleatoria de tipo String.
*/
fn pick_response() -> String {
    // Creación de un número aleatorio.
    let num = rand::thread_rng().gen_range(1,6);

    // Elección de un respuesta a partir del número aleatorio.
    let response = match num {
        1 => "Hola JSON",
        2 => "Estoy usando el crate Iron.",
        3 => "Estoy usando también rustc_serialize.",
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
