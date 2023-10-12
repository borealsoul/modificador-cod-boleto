use chrono::NaiveDate;
use clipboard::{ClipboardContext, ClipboardProvider};
use color_print::{cprint, cprintln};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{cursor, execute};
use mod_cod_barras::*;
use rustyline::{DefaultEditor, Result};
use std::io::stdout;

fn main() -> Result<()> {
    // rl é um atalho para read_line de rustyline
    let mut rl = DefaultEditor::new()?;
    // _cod_barras_lido é o código de barras bruto inserido pelo usuário
    let mut _cod_barras_lido = String::new();
    // cod_barras_liq é o código sem dig. verificadores, espaços, traços, etc.
    let mut cod_barras_liq = String::new();
    // ctx é o atalho para acessar o ctrl-c, provido pelo clipboard
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    // se o conteudo do ctrl-c do usuário tiver 55 carac., tamanho de um código de barras,
    // pergunte se ele quer usar este.
    if ctx.get_contents().unwrap().len() == 55 {
        loop {
            disable_raw_mode().unwrap();
            execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
            cprintln!("<green, bold>Parece que você possui um código de barras copiado, deseja inserir esse?</>");
            cprintln!("<green, bold><u>S</>im</>/<red, bold><u>N</>ão</>");
            enable_raw_mode().unwrap();
            break match read_char()? {
                's' | ' ' => {
                    _cod_barras_lido = String::from(&ctx.get_contents().unwrap());
                }
                'n' => (),
                _ => continue,
            };
        }
    };

    disable_raw_mode().unwrap();

    if _cod_barras_lido.is_empty() {
        loop {
            // Pede para o usuário digitar o código de barras desejado, caso ele não tenha 55 carac.,
            // repita o pedido.
            cprintln!("<green, bold>Digite o código de barras:</> ");
            _cod_barras_lido = String::from(rl.readline("> ")?.trim());

            if _cod_barras_lido.len() != 55 {
                cprintln!("<y, bold>Há um problema com seu código, favor verifique.</>");
                continue;
            }

            break;
        }
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

    // Cria a array cod_barras_arr, e a preenche com os dígitos do cod_barras_liq, usando o LAYOUT
    // como base.
    let mut cod_barras_arr: [String; 10] = Default::default();
    for (cod_barras, layout) in cod_barras_arr.iter_mut().zip(LAYOUT.iter()) {
        (*cod_barras, cod_barras_liq) = dividir_string(cod_barras_liq, *layout);

        // Se o código não for composto apenas por números, dê pânico e feche o programa.
        let Ok(_) = cod_barras.parse::<usize>() else {
            panic!("Esse código de barras não é valido, fechando...");
        };
    }

    loop {
        // Limpa a tela do terminal.
        execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();

        cprint!("<cyan, bold>O código de barras atual é:</> ");
        for i in cod_barras_arr.iter() {
            print!("{}", i);
        }
        print!(".");

        lista_cod(&cod_barras_arr);

        enable_raw_mode().unwrap();
        match read_char()? {
            // Caso o usuário escolha mudar o valor:
            // valor_guia é um float com os dígitos da guia.
            // Se o valor_tmp lido do input, sem virgulas ou pontos,
            // for possível converter para f32.
            // Formate-o como string, preenchendo-o de zeros até dar 11 dígitos.
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
            // Converte a data_tmp do valor original da array de AAAAMMDD para DD/MM/AAAA.
            // Se o valor digitado tiver 10 dígitos, e for um NaiveDate válido,
            // devolva-o reformatado como AAAAMMDD para a array.
            'd' => {
                disable_raw_mode().unwrap();

                cprintln!("<green, bold>Digite a nova data <i>com barras entre os números</>.</>");

                loop {
                    let data_tmp: String = NaiveDate::parse_from_str(&cod_barras_arr[4], "%Y%m%d")
                        .unwrap()
                        .format("%d/%m/%Y")
                        .to_string();
                    let Ok(valor_tmp) = NaiveDate::parse_from_str(
                        &rl.readline_with_initial("> ", (&data_tmp, ""))?,
                        "%d/%m/%Y",
                    ) else {
                        cprintln!("<yellow, bold>Há um problema com o valor digitado, tente novamente.</>");
                        continue;
                    };

                    cod_barras_arr[4] = valor_tmp.format("%Y%m%d").to_string();
                    break;
                }
            }
            'g' => {
                disable_raw_mode().unwrap();

                loop {
                    cprintln!("<green, bold>Digite o novo número da guia:</>");

                    let Ok(valor_tmp) = rl
                        .readline_with_initial(
                            "> ",
                            (cod_barras_arr[5].trim_start_matches('0'), ""),
                        )?
                        .parse::<u16>()
                    else {
                        cprintln!("<yellow, bold>Há um problema com o valor digitado, tente novamente.</>");
                        continue;
                    };

                    cod_barras_arr[5] = format!("{:0>7}", valor_tmp);
                    break;
                }
            }
            'p' => {
                disable_raw_mode().unwrap();

                loop {
                    cprintln!("<green, bold>Digite a nova parcela:</>");
                    let Ok(valor_tmp) = rl
                        .readline_with_initial(
                            "> ",
                            (cod_barras_arr[6].trim_start_matches('0'), ""),
                        )?
                        .parse::<u8>()
                    else {
                        cprintln!("<yellow, bold>Há um problema com o valor digitado, tente novamente.</>");
                        continue;
                    };

                    // Se o valor não for entre 1 e 46 (o limite de parcelas aceitas), peça o
                    // número novamente.
                    match valor_tmp {
                        1..=46 => (),
                        _ => {
                            cprintln!(
                            "<yellow, bold>O número de parcelas é inválido, tente novamente.</>"
                        );
                            continue;
                        }
                    }

                    cod_barras_arr[6] = format!("{:0>3}", valor_tmp);
                    break;
                }
            }
            'e' => {
                disable_raw_mode().unwrap();

                // Pega o ano da data de vencimento como AA, em número.
                let data_tmp = NaiveDate::parse_from_str(&cod_barras_arr[4], "%Y%m%d")
                    .unwrap()
                    .format("%y")
                    .to_string()
                    .parse::<u8>()
                    .unwrap();

                loop {
                    cprintln!("<green, bold>Digite o novo exercício:</>");
                    let Ok(valor_tmp) = rl
                        .readline_with_initial(
                            "> 20",
                            (cod_barras_arr[8].trim_start_matches('0'), ""),
                        )?
                        .parse::<u8>()
                    else {
                        cprintln!("<yellow, bold>Há um problema com o valor digitado, tente novamente.</>");
                        continue;
                    };

                    // Se o exercício for no futuro, se comparado ao vencimento,
                    // Ou for há mais de quatro anos atrás, retorne um erro.
                    if valor_tmp != data_tmp {
                        if let 1..=4 = data_tmp as i32 - valor_tmp as i32 {
                        } else {
                            // match data_tmp as i32 - valor_tmp as i32 {
                            //     1..=4 => break,
                            //     _ => (),
                            // }
                            cprintln!("<yellow, bold>Você parece ter digitado um ano diferente do ano de vencimento da guia.</>");
                            continue;
                        }
                    };

                    cod_barras_arr[8] = valor_tmp.to_string();
                    break;
                }
            }
            't' => {
                disable_raw_mode().unwrap();

                loop {
                    cprintln!("<green, bold>Digite o código do novo tributo:</>");
                    let Ok(valor_tmp) = rl
                        .readline_with_initial(
                            "> ",
                            (cod_barras_arr[9].trim_start_matches('0'), ""),
                        )?
                        .parse::<u16>()
                    else {
                        cprintln!("<yellow, bold>Há um problema com o valor digitado, tente novamente.</>");
                        continue;
                    };

                    cod_barras_arr[9] = format!("{:0>4}", valor_tmp);
                    break;
                }
            }
            's' => {
                let mut valor_tmp: String = String::new();

                for cod_barras in cod_barras_arr.iter() {
                    valor_tmp += cod_barras;
                }

                ctx.set_contents(valor_tmp.to_owned()).unwrap();
                break;
            }
            _ => continue,
        }
    }

    disable_raw_mode().unwrap();

    Ok(())
}
