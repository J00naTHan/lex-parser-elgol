// importa para o escopo da main o código do módulo lexer (arquivo lexer.rs)
pub mod lexer;

use logos::Logos;
use std::io::stdin;
use std::io::Read;
use std::fs::File;

fn main() {
    println!("Analisador Léxico Sintático\n");

    // permite ao usuário selecionar o nome do arquivo com o código fonte Elgol :P
    println!("Informe o nome do arquivo com código Elgol:");
    let mut file_name = String::new();

    // lê a entrada do usuário, a associa à variável mutável definida acima e trata possíveis erros
    stdin()
        .read_line(&mut file_name)
        .expect("\nNão foi possível ler a entrada");
    file_name = file_name.trim().to_string();

    let mut haystack = String::new();
    let mut file_result = File::open(file_name).unwrap();
    file_result
        .read_to_string(&mut haystack)
        .unwrap();

    // println!("\n{:?}", haystack);

    let lexer = lexer::Token::lexer(&haystack);

    for result in lexer {
        match result {
            Ok(token) => println!("{:#?}", token),
            Err(e) => continue,
        }
    }
}
