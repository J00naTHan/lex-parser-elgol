use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "elgol.pest"]
struct ElgolParser;

const CODE1: &str = r#"
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

const CODE2: &str = r#"
# Linguagem Elgol
# autor Elgio Schlemer
# sem erros
inteiro _Fazalgo (inteiro Fazum, inteiro Fazdois) .
inicio .
   inteiro Lixo .
   
   Lixo = Fazum x Fazdois + zero .
   elgio = Lixo .
fim .

# programa principal, fora de função
# deve começar também com inicio e fim que seriam os
# { e } do C
inicio .
  inteiro Variavel .
  inteiro Teste .
  
  Variavel = zero .
  Teste = 45 x _Fazalgo (45, Variavel) .  
  Variavel = Variavel + 3 - 5 x 2 / 2 + Teste .
fim.
"#;

const CODE3: &str = r#"
# com erros lexicos
inteiro _Fazalgo2 (inteiro Fazum, inteiro Faz2) .
inicio.
   inteiro Variável .
   Fazum = 0;
   Fazum = 45 * 20 .
fim.
"#;

const CODE4: &str = r#"
# erros sintáticos
inteiro _Fazalgo(inteiro Fazum, inteiro). # falta id depois do inteiro
inicio   						# faltou .
   inteiro Lixo.
   
   Lixo = Fazum - Fazdois + zero x. # operador x precisa de 2 operandos
   elgio = Lixo + _Soma(20,30).     # elgio não permite ter função
fim.

inicio.
  inteiro Variavel, Lixo.   # Não pode ter , aqui
  inteiro Teste.
  
  Variavel = zero.
  
  Teste = 45 x _Fazalgo (50, Variavel).
  
  se Teste maior zero.
  entao.
  inicio.
    Variavel = Teste igual Lixo. # Somente exp mat podem ser atribuídas
  fim.
  # erro semântico (não implementar). Faltou senao
  
  Variavel = Variavel + Nova + 3 - 5 x 2 / 2 + Teste.
  # semântico: Nova não foi declarada
fim.
"#;

const CODE5: &str = r#"
# Linguagem Elgol
# autor Elgio Schlemer
# exemplo 4
inteiro _Fazalgo (inteiro Fazum, inteiro Fazdois).
inicio.
   inteiro Lixo.
   inteiro Lixox.
   
   Lixo = Fazum x Fazdois + zero.
   Fazum = Lixox - Fazdois + 34 - zero x 15 / 3 x 4 - 87 + Fazum.
   elgio = Fazum.
fim.

inicio.
  inteiro Variavel.
  inteiro Teste.
  inteiro Varx.
  
  Variavel = zero. 
  Variaval = Teste + Nada .
  Teste = 45 x _Fazalgo (45, Variavel).
  
  Varx = _Fazalgo (Teste, zero).   # Comentario aqui
  
  se Varx maior Variavel.
  entao.
  inicio.
      Varx = Varx x _Fazalgo(Variavel, zero).
      Teste = 45.
  fim.
  senao.
  inicio.
      Varx = zero.
  fim.
  
  Variavel = Variavel + 3 - 5 x 2 / 2 + Teste.
fim.
"#;

pub fn parse(selection: u8) {
    let source_code = match selection {
        1 => CODE1,
        2 => CODE2,
        3 => CODE3,
        4 => CODE4,
        5 => CODE5,
        0_u8 | 6_u8..=u8::MAX => todo!(),
    };
    let parsed = ElgolParser::parse(Rule::programa, source_code);
    match parsed {
        Ok(pairs) => {
            println!("{:#?}", pairs);
        },
        Err(e) => println!("Erro de sintaxe: {}", e),
    }
}