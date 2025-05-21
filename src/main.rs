// importa o código do modulo lexer (arquivo lexer.rs) para o escopo da main
pub mod lexer;

// importa a struct Logos da crate logos no escopo da main
use logos::Logos;

// usa as implementações da crate std (standard) do Rust, que é inclusa naturalmente na linguagem
use std::io::stdin;
use std::io::Read;
use std::fs::File;

fn main() {
    println!("Analisador Léxico Sintático\n");

    // lê a entrada do usuário contendo o caminho do código fonte Elgol, associa essa entrada à variável mutável definida abaixo e trata possíveis erros
    println!("Informe o nome do arquivo com código Elgol:");
    let mut file_name = String::new();
    stdin()
        .read_line(&mut file_name)
        .expect("\nNão foi possível ler a entrada");
    file_name = file_name.trim().to_string();

    // abre o arquivo com o caminho fornecido pelo usuário, associa o arquivo à variável mutável definida abaixo e lida com possíveis erros
    let mut haystack = String::new();
    let mut file_result = File::open(file_name).unwrap();
    file_result
        .read_to_string(&mut haystack)
        .unwrap();

    // teste para visualização do arquivo com código-fonte Elgol
    // println!("\n{:?}", haystack);

    // instancia uma struct de analisador léxico do Logos passando o texto que deve ser analisado
    let mut lexer = lexer::Token::lexer(&haystack);

    // exibe cada token gerado pelo analisador léxico, lidando com possíveis erros ao exibir os tokens
    while let Some(token) = lexer.next() {
        if let Ok(lexer::Token::FunctionId((line, column, ref word))) = token {
            println!("<FunctionId: {}, {}:{}>", word, line, column);
        }
        if let Ok(lexer::Token::Identifier((line, column, ref word))) = token {
            println!("<Identifier: {}, {}:{}>", word, line, column);
        }
        if let Ok(lexer::Token::Integer((line, column, ref word))) = token {
            println!("<Integer: {}, {}:{}>", word, line, column);
        }
        if let Ok(lexer::Token::Reserved((line, column, ref word))) = token {
            println!("<Reserved: {}, {}:{}>", word, line, column);
        }
        if let Ok(lexer::Token::Punctuation((line, column, ref word))) = token {
            println!("<Punctuation: {}, {}:{}>", word, line, column);
        }
        if let Ok(lexer::Token::Operator((line, column, ref word))) = token {
            println!("<Operator: {}, {}:{}>", word, line, column);
        }
    }
}
