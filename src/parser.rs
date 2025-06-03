use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "elgol.pest"]
pub struct ElgolParser;

pub fn parse(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let parsed = ElgolParser::parse(Rule::programa, input)?;
    
    // Processar a árvore de parsing aqui
    for pair in parsed {
        match pair.as_rule() {
            Rule::funcao => println!("Função: {:?}", pair.as_str()),
            Rule::expressao => println!("Comando: {:?}", pair.as_str()),
            _ => {}
        }
    }
    
    Ok(())
}

pub fn execute_parser() {
    let code = r#"
        inteiro _Soma(inteiro a, inteiro b).
        inicio.
            elgio = a + b.
        fim.

        inicio.
            inteiro x.
            x = _Soma(5, 10).
        fim.
    "#;

    parse(code).unwrap();
}