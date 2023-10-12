use chrono::NaiveDate;
use color_print::cprintln;
use core::cmp;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};

// Layout é quantos caracteres há em cada argumento do código de barras
pub const LAYOUT: [u8; 10] = [3, 1, 11, 4, 8, 7, 3, 1, 2, 4];
//  https://www.notion.so/Layout-do-C-digo-de-Barras-d1e55be98dd64099aeeffff52b2e201f?pvs=4
//  Início  Fim     Valor
//  1       3       "816”
//  4       4       Dig. Verificador - Mod. 10
//  5       15      Valor da Guia (11, 2)
//  16      19      Cód. Municipal Febrabam
//  20      27      Vencimento (AAAAMMDD)
//  28      34      Núm. Guia
//  35      37      Parcela (000 é Única)
//  38      38      Cód. Layout
//  39      40      Exercício (AA)
//  41      44      Tributo

// Lê o carac. que o usuário digitou, pego da demo interativa do crossterm.
pub fn read_char() -> std::io::Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            kind: KeyEventKind::Press,
            modifiers: _,
            state: _,
        })) = crossterm::event::read()
        {
            return Ok(c.to_ascii_lowercase());
        }
    }
}

// Divide uma string, pela quantidade de caracteres providos pelo parâmetro.
//
// função: dividir_string
// param: str_inicial, String que será dividida.
// param: num_digitos, número de caracteres do pedaço que será dividido.
// retorna: tuple:
//  _pedaco: O pedaço que foi dividido,
//  _resto: O restante da string após o pedaço.
pub fn dividir_string(str_inicial: String, num_digitos: u8) -> (String, String) {
    // https://users.rust-lang.org/t/solved-how-to-split-string-into-multiple-sub-strings-with-given-length/10542
    let (_pedaco, _resto) = str_inicial.split_at(cmp::min(num_digitos.into(), str_inicial.len()));
    // println!("O recorte é {_pedaco}, com o restante {_resto}");
    (String::from(_pedaco), String::from(_resto))
}

// Lista os argumentos do código de barras de forma descrita e espaçada.
//
// função: lista_cod
// param: cod_barras, a array já dividida com o código de barras
pub fn lista_cod(cod_barras: &[String; 10]) {
    // valor_guia = cod_barras[2] (o valor da guia como um número inteiro de 11 carac.) como float32 dividido por 100
    // após formatá-la com duas casas decimas e trocar o "." por ",", imprima-a.
    let valor_guia: f32 = (cod_barras[2].parse::<f32>().unwrap()) / 100.0;
    let guia_tmp = format!("{:.2}", valor_guia).replace(".", ",");
    cprintln!("\n<blue, bold><u>V</u>alor da guia:</> R$ {}", guia_tmp);

    // _vencimento = Data Ingênua (NaiveDate) de cod_barras[4] (data de vencimento como AAAAMMDD),
    // formatado como DD/MM/AAAA
    let mut _vencimento = (NaiveDate::parse_from_str(cod_barras[4].clone().as_str(), "%Y%m%d"))
        .unwrap()
        .format("%d/%m/%Y");
    cprintln!("<blue, bold><u>D</>ata de Vencimento:</> {_vencimento}");

    // Para i entre 5 e o tamanho do Layout (9)
    // converta o valor do cod_barras respectivo para unsign16
    // e o imprima.
    for (indice, codigo) in cod_barras.iter().enumerate() {
        match indice {
            5 | 6 | 8 | 9 => {
                let cod_tmp: &u16 = &codigo.parse::<u16>().unwrap();

                match indice {
                    5 => cprintln!("<blue, bold>Núm. da <u>G</>uia:</> {cod_tmp}"),
                    6 => cprintln!("<blue, bold><u>P</>arcela:</> {cod_tmp}"),
                    8 => cprintln!("<blue, bold><u>E</>xercício:</> 20{cod_tmp}"),
                    9 => cprintln!("<blue, bold><u>T</>ributo:</> {cod_tmp}"),
                    _ => (),
                };
            }
            _ => (),
        }
    }

    cprintln!("<red, bold><u>S</>air</>\n");
}
