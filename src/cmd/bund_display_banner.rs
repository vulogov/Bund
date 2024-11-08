use cfonts::{ render, Options, Fonts };

pub fn banner(s: &String) -> String {
    render(
        Options {
            text: String::from(s),
            font: Fonts::FontPallet,
            ..Options::default()
        }
    ).text
}

pub fn bund_banner() -> String {
    let ban = format!("BUND {}", env!("CARGO_PKG_VERSION"));
    banner(&ban)
}
