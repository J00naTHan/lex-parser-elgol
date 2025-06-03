// importa a crate logos (structs Lexer, Logos e Skip) no escopo do analisador léxico (lexer)
use logos::{Lexer, Logos, Skip};


/// atualiza o contador de linhas e colunas do código-fonte usando caracteres de quebra de linha
// baseado no exemplo fornecido em: https://logos.maciej.codes/extras.html
fn newline_callback(lex: &mut Lexer<Token>) -> Skip {
    lex.extras.0 += 1;
    lex.extras.1 = lex.span().end;
    Skip
}


/// calcula a posição de linha e coluna e retorna a palavra para um token encontrado
// baseado no exemplo fornecido em: https://logos.maciej.codes/extras.html
fn token_callback(lex: &mut Lexer<Token>) -> (usize, usize, String) {
    let line = lex.extras.0;
    let column = lex.span().start - lex.extras.1;
    let word = lex.slice();
    (line + 1, column + 1, word.to_string())
}


/// deriva 3 atributos para o lexer, efetivamente tornando o enum em um lexer Logos
#[derive(Logos, Debug, PartialEq)]

/// atributo para especificar que o analisador deve pular comentários e espaços em branco
#[logos(skip r"#.*|[ \r\t\f]+")]

/// atributo para definir um estado interno ao lexer, esse contém a linha e a coluna correspondente ao token
#[logos(extras = (usize, usize))]

/// enum que define o lexer Logos, em que cada item define um tipo de token reconhecível pelo lexer Logos
pub enum Token {
    #[regex(r"\n", newline_callback, priority = 7)]
    NewLine,

    #[regex(r"_[A-Z][a-zA-Z]{2,}", token_callback, priority = 6)]
    FunctionId((usize, usize, String)),

    #[regex(r"[A-Z][a-zA-Z]{2,}", token_callback, priority = 5)]
    Identifier((usize, usize, String)),

    #[regex(r"[1-9][0-9]*", token_callback, priority = 4)]
    Integer((usize, usize, String)),

    #[regex(r"elgio|inteiro|zero|comp|enquanto|se|entao|senao|inicio|fim|maior|menor|igual|diferente", token_callback, priority = 3)]
    Reserved((usize, usize, String)),

    #[regex(r"[\.(),]", token_callback, priority = 2)]
    Punctuation((usize, usize, String)),

    #[regex(r"[\+\-x=\/]", token_callback, priority = 1)]
    Operator((usize, usize, String)),
}
