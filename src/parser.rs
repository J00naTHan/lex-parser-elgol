use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "elgol.pest"] // assumindo que sua gramática está em elgol.pest
struct ElgolParser;

pub fn main() {
    let source_code = r#"
        inteiro _Soma(inteiro Teste, inteiro Teste).
        inicio.
            elgio = Teste + Teste.
        fim.

        inicio.
            inteiro Xis.
            Xis = 5.
            _Soma(Xis, 10).
        fim.
    "#;

    let parsed = ElgolParser::parse(Rule::programa, source_code);
    match parsed {
        Ok(pairs) => {
            println!("Análise sintática bem-sucedida!");
            // Para ver a estrutura completa:
            println!("{:#?}", pairs);
        },
        Err(e) => println!("Erro de sintaxe: {}", e),
    }
}