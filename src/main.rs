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
Definimos una struct llamada LaEstructura con diversos tipos de datos.
LaEstructura debe poder ser codificable o "encodable".
Es por ello que le precedemos con un atributo derive RustcEncodable.
El dato "mensaje" será aleatorio.
*/
#[derive(RustcDecodable, RustcEncodable)]
struct LaEstructura{
    mensaje: String,
    numero: i32,
    punto_flotante: f32,
    booleano: bool
}

/*
La función elegir_frase() devuelve una frase aleatoria.
*/
fn elegir_frase() -> String {
    // Creación de un número aleatorio con rand.
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
    // Mostramos un mensaje para que el usuario sepa la URL y el puerto del servidor
    println!("Listening on http://{}", MY_URL);

    // Levantamos a Iron para que responda a solicitudes:
    Iron::new(|_: &mut Request| {
        // Establecemos el tipo de contenido que vamos a devolver a JSON con charset=utf-8
        let tipo_de_contenido = "application/json;charset=utf-8".parse::<Mime>().unwrap();
        // Poblamos de valores LaEstructura y la pasamos a la variable respuesta
        let respuesta = LaEstructura { mensaje: elegir_frase(), numero: 100, punto_flotante: 7.1, booleano: true };
        // Codificamos la respuesta a JSON ...
        let salida_json = json::encode(&respuesta).unwrap();
        // Armamos una respuesta y la enviamos a la dirección definidad en My_URL.
        Ok(Response::with((tipo_de_contenido, status::Ok, salida_json)))
    }).http(MY_URL).unwrap();
}
