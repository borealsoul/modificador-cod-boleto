use mod_cod_barras::*;
// Chrono ajuda a lidar com strings relacionadas a datas.
use chrono::NaiveDate;
// Clipboard lê o texto copiado pelo sistema, e devolve o texto final.
use clipboard::{ClipboardContext, ClipboardProvider};
// Color_Print adiciona tags alá XML para estilo de prints e str.
use color_print::{cprint, cprintln};
// Crossterm dá várias funções extras para um CLI.
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{cursor, execute};
// Rustyline simplifica read_line, e adiciona mais recursos.
use rustyline::{DefaultEditor, Result};
use std::io::stdout;
// use std::{thread, time::Duration};

fn main() -> Result<()> {
    // rl é um atalho para read_line de rustyline
    let mut rl = DefaultEditor::new()?;
    // _cod_barras_lido é o código de barras bruto inserido pelo usuário
    let mut _cod_barras_lido = String::new();
    // cod_barras_liq é o código sem dig. verificadores, espaços, traços, etc.
    let mut cod_barras_liq = String::new();
    // ctx é o atalho para acessar o ctrl-c, provido pelo clipboard
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    loop {
        // se o conteudo do ctrl-c do usuário tiver 55 carac., tamanho de um código de barras,
        // pergunte se ele quer usar este.
        match &ctx.get_contents().unwrap().len() {
            55 => {
                cprintln!("<green, bold>Parece que você possui um código de barras copiado, deseja inserir esse?</>");
                cprintln!("<green, bold><u>S</>im</>/<red, bold><u>N</>ão</>");
                enable_raw_mode().unwrap();
                match read_char()? {
                    's' | ' ' => {
                        _cod_barras_lido = String::from(&ctx.get_contents().unwrap());
                        disable_raw_mode().unwrap();
                        break;
                    }
                    'n' | _ => (),
                };
            }
            _ => (),
        };
        disable_raw_mode().unwrap();

        // Pede para o usuário digitar o código de barras desejado, caso ele não tenha 55 carac.,
        // repita o pedido.
        cprintln!("<green, bold>Digite o código de barras:</> ");
        _cod_barras_lido = String::from(rl.readline("> ")?.trim());

        // println!("{} Caracteres.", &cod_barras_lido.len());

        // https://www.reddit.com/r/rust/comments/obnlv8/some_neat_rust_syntax_loop_break_match/
        break match &_cod_barras_lido.len() {
            55 => (),
            _ => {
                cprintln!("<y, bold>Há um problema com seu código, favor verifique.</>");
                continue;
            }
        };
    }

    // Limpa o código de barras copiado, tirando os dígitos verificadores e etc.
    while !&_cod_barras_lido.is_empty() {
        // println!("Caracteres Restantes: {}", &cod_barras_lido.len());
        // println!("Seu módulo é {}.", &cod_barras_lido.len() % 2);
        let mut _cod_barra_pedaco_tmp = String::new();
        match &_cod_barras_lido.len() % 2 {
            0 => (_, _cod_barras_lido) = dividir_string(_cod_barras_lido, 3),
            _ => {
                (_cod_barra_pedaco_tmp, _cod_barras_lido) = dividir_string(_cod_barras_lido, 11);
                cod_barras_liq += &_cod_barra_pedaco_tmp;
            }
        };
    }

    drop(_cod_barras_lido);

    // println!("{}", cod_barras_liq);

    let mut cod_barras_arr: [String; 10] = Default::default();

    for i in 0..LAYOUT.len() {
        (cod_barras_arr[i], cod_barras_liq) = dividir_string(cod_barras_liq, LAYOUT[i]);

        // Se o código não for composto apenas por números, dê pânico e feche o programa.
        let _ = match (&cod_barras_arr[i]).parse::<usize>() {
            Ok(usize) => usize,
            Err(_) => panic!("Esse código de barras não é valido, fechando..."),
        };
    }

    loop {
        execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();

        cprint!("<cyan, bold>O código de barras atual é:</> ");
        for i in 0..LAYOUT.len() {
            print!("{}", cod_barras_arr[i]);
        }
        print!(".");

        lista_cod(&cod_barras_arr);

        enable_raw_mode().unwrap();
        match read_char()? {
            'v' => {
                disable_raw_mode().unwrap();

                cprintln!("<green, bold>Digite o valor novo <i>com todas as casas decimais</>.</>");

                let valor_guia = &cod_barras_arr[2].parse::<f32>().unwrap() / 100.0;

                loop {
                    let Ok(valor_tmp) = rl
                        .readline_with_initial(
                            "> R$ ",
                            (&valor_guia.to_string().replace(".", ","), ""),
                        )?
                        .replace(".", ",")
                        .replace(",", "")
                        .parse::<u32>()
                    else {
                        cprintln!("<yellow, bold>Há um problema com o valor digitado, tente novamente.</>");
                        continue;
                    };

                    if valor_tmp.to_string().len() > 11 {
                        continue;
                    }

                    cod_barras_arr[2] = format!("{:0>11}", &valor_tmp.to_string());
                    break;
                }
            }
            'd' => {
                // let data_tmp = NaiveDate::parse_from_str(&cod_barras_arr[4], "%d/%m/%Y").unwrap();
                disable_raw_mode().unwrap();
                let mut valor_tmp: String = String::new();

                cprintln!("<green, bold>Digite a nova data <i>com barras entre os números</>.</>");

                loop {
                    let data_tmp: String = NaiveDate::parse_from_str(&cod_barras_arr[4], "%Y%m%d")
                        .unwrap()
                        .format("%d/%m/%Y")
                        .to_string();
                    valor_tmp = rl.readline_with_initial("> ", (&data_tmp, ""))?;

                    match valor_tmp.len() {
                        10 => (),
                        _ => {
                            cprintln!("<yellow, bold>Há um problema com o valor digitado, tente novamente.</>");
                            continue;
                        }
                    }

                    cod_barras_arr[4] = match NaiveDate::parse_from_str(&valor_tmp, "%d/%m/%Y") {
                        Ok(NaiveDate) => NaiveDate,
                        Err(ParseError) => {
                            cprintln!("<yellow, bold>Há um problema com o valor digitado, tente novamente.</>");
                            continue;
                        }
                    }
                    .format("%Y%m%d")
                    .to_string();
                    break;
                }
            }
            'g' => {
                disable_raw_mode().unwrap();

                cprintln!("<green, bold>Digite o novo número da guia:</>");
                let guia_tmp = &cod_barras_arr[5].parse::<u16>().unwrap();
                let valor_tmp: String =
                    rl.readline_with_initial("> ", (&guia_tmp.to_string(), ""))?;

                cod_barras_arr[5] = format!("{:0>7}", valor_tmp);
            }
            'p' => {
                disable_raw_mode().unwrap();

                cprintln!("<green, bold>Digite a nova parcela:</>");
                let parcela_tmp = &cod_barras_arr[6].parse::<u16>().unwrap();
                let valor_tmp: String =
                    rl.readline_with_initial("> ", (&parcela_tmp.to_string(), ""))?;

                cod_barras_arr[6] = format!("{:0>3}", valor_tmp);
            }
            'e' => {
                disable_raw_mode().unwrap();

                cprintln!("<green, bold>Digite o novo exercício:</>");
                let exercicio_tmp = &cod_barras_arr[8].parse::<u8>().unwrap();
                let valor_tmp =
                    rl.readline_with_initial("> 20", (&exercicio_tmp.to_string(), ""))?;

                cod_barras_arr[8] = valor_tmp;
            }
            't' => {
                disable_raw_mode().unwrap();

                cprintln!("<green, bold>Digite o código do novo tributo:</>");
                let tributo_tmp = &cod_barras_arr[9].parse::<u8>().unwrap();
                let valor_tmp = rl.readline_with_initial("> ", (&tributo_tmp.to_string(), ""))?;

                cod_barras_arr[9] = format!("{:0>4}", valor_tmp);
            }
            's' | _ => {
                let mut valor_tmp: String = String::new();

                for i in 0..=LAYOUT.len() {
                    valor_tmp += &cod_barras_arr[i];
                }

                ctx.set_contents(valor_tmp.to_owned()).unwrap();
                break;
            }
        }
    }

    disable_raw_mode().unwrap();

    Ok(())
}
