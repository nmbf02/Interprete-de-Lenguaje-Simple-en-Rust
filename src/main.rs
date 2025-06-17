mod lexer;
mod parser;

use lexer::Lexer;
use parser::{Context, Parser, Statement};
use std::fs;
use std::path::Path;

/// Función principal que ejecuta el intérprete de tu lenguaje.
fn main() {
    // Ruta del archivo de entrada (puedes cambiarlo)
    let path = Path::new("programa.txt");

    // Leer el contenido del archivo como un string
    let input = fs::read_to_string(path)
        .expect("No se pudo leer el archivo fuente (programa.txt)");

    // Crear el lexer y parser
    let lexer = Lexer::new(&input);
    let mut parser = Parser::new(lexer);

    // Parsear todas las instrucciones del archivo
    let statements: Vec<Statement> = parser.parse_statements();

    // Crear un contexto vacío para almacenar las variables
    let mut ctx = Context::new();

    // Ejecutar cada instrucción
    for stmt in statements {
        if let Err(e) = stmt.execute(&mut ctx) {
            eprintln!("Error en tiempo de ejecución: {}", e);
            break;
        }
    }
}
