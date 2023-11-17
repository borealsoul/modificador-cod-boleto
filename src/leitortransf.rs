use clipboard::{ClipboardContext, ClipboardProvider};
use color_print::{cprint, cprintln};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{cursor, execute};
use mod_cod_barras::*;
use std::io::stdout;

pub fn ler_código_início(ctx: &mut ClipboardContext) -> String {
    // se o conteudo do ctrl-c do usuário tiver 55 carac., tamanho de um código de barras,
    // pergunte se ele quer usar este.
    loop {
        disable_raw_mode().unwrap();
        execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
        cprintln!("<green, bold>Parece que você possui um código de barras copiado, deseja inserir esse?</>");
        cprintln!("<green, bold><u>S</>im</>/<red, bold><u>N</>ão</>");
        enable_raw_mode().unwrap();
        break match read_char().unwrap() {
            's' | ' ' => String::from(&ctx.get_contents().unwrap()),
            'n' => String::from(""),
            _ => continue,
        };
    }
}
