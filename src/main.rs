use std::io::LineWriter;

//agregamos los use statements
use serde_json::{Result, Value};
//Indicamos que vamos a utilizar la estructura Value el cual 
// es utilizado para crear un objeto de memory-safe para que
//Rust pueda interpretar el JSON quel e pasemos

fn main() -> Result<()> {
    //! con r# indicamos qeu es un raw string
    //! si no establecemos el JSON como raw string las "" de las valores pueden generar problemas
    let libro = r#"{
        "titulo": "The Pragmatic Programmer",
        "autores": [
            "David Thomas",
            "Andrew Hunt"
        ],
        "total_paginas": 352,
        "generos": [
            "programacion",
            "ingenieria",
            "educacion"
        ],
        "precios": [
            {
            "tipo": "digital",
            "precio": 15.00,
            "moneda": "USD"
            },
            {
            "tipo": "tapa dura",
            "precio": 35.50,
            "moneda": "USD"
            }
        ]
    }"#;
    println!("Libro sin rpocesar: {}", libro);
    //convertimos el string que representa el JSON a un objeto de tipo Value, permitiendonos procesar sus valores 
    let libro_parseado: Value = serde_json::from_str(libro)?;

    println!("Libro Procesado por Rust");
    println!("Titulo del libro: {}", libro_parseado["titulo"]);
    println!("Titulo del libro sin comillas {}", libro_parseado["titulo"].as_str().unwrap()); //indicamos que si puede, convierta el valor a str
    println!("Cantidad de paginas: {}", libro_parseado["total_paginas"]);
    println!("Primer genero: {}", libro_parseado["generos"][0]);
    println!("Precio digital: {}", libro_parseado["precios"][0]["precio"]);

    const DESCUENTO: f64 = 10.00;

    println!("-------------------------------------------------------------------------");

    for precio in libro_parseado["precios"].as_array().unwrap(){
        println!("==========================================================");
        let precio_regular = precio["precio"].as_f64().unwrap();
        let precio_descuento = precio_regular - (precio_regular * (DESCUENTO / 100.00));

        println!("Tipo: {}", precio["tipo"].as_str().unwrap());
        println!("Precio regular: {} {}", precio_regular,  precio["moneda"].as_str().unwrap());
        println!("Precio con 10% de descuento: {} {}", precio_descuento, precio["moneda"].as_str().unwrap());
    }
    
    Ok(())
}


