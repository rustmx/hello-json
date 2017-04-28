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
LaEstructura debe poder ser codificable o "encodable".
Es por ello que le precedemos con un derive RustcEncodable.
*/
#[derive(RustcDecodable, RustcEncodable)]
struct LaEstructura{
    mensaje: String
}


/*
La función pick_response() crea un respuesta aleatoria de tipo String.
*/
fn elegir_frase() -> String {
    // Creación de un número aleatorio.
    let num = rand::thread_rng().gen_range(1,6);

    // Elección de una frase a partir del número aleatorio.
    let frase_elegida = match num {
        1 => "Hola JSON",
        2 => "Estoy usando el crate Iron.",
        3 => "Estoy usando también rustc_serialize.",
        4 => "á é í ó ú ñ",
        5 => "Más letras especiales ü ö ç $ &",
        _ => ""
    };

    frase_elegida.to_string()
}

fn main() {
    println!("Listening on http://{}", MY_URL);
    Iron::new(|_: &mut Request| {
        let tipo_de_contenido = "application/json;charset=utf-8".parse::<Mime>().unwrap();
        let respuesta = LaEstructura { mensaje: elegir_frase() };
        let salida_json = json::encode(&respuesta).unwrap();
        Ok(Response::with((tipo_de_contenido, status::Ok, salida_json)))
    }).http(MY_URL).unwrap();
}
