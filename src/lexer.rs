use logos::Logos;

#[derive(Logos, Debug, PartialEq)]

// macro para especificar que o analisador deve pular comentários e espaços em branco
#[logos(skip r"#.*|[ \r\t\n\f]+")]

pub enum Token {
    #[regex(r"_[A-Z][a-zA-Z]{2,}")]
    FunctionId,

    #[regex(r"[A-Z][a-zA-Z]{2,}")]
    Identifier,

    #[regex(r"[1-9][0-9]*")]
    Integer,

    #[regex(r"elgio|inteiro|zero|comp|enquanto|se|entao|senao|inicio|fim|maior|menor|igual|diferente")]
    Reserved,

    #[regex(r"[\.()]")]
    Punctuation,

    #[regex(r"[\+\-x=\/]")]
    Operator,
}
