use crate::Code;

pub const FONTS_LEN: usize = 2;
pub const FONTS: [&'static str; FONTS_LEN] = [
    // normal font
    "Fira Mono-14",
    // bold font
    "Fira Mono-14:bold"
];

pub const COLORS_LEN: usize = 2;
// each color has 4 16 bit components.
pub const COLORS: [(u16, u16, u16, u16); COLORS_LEN] = [
    // background color
    (0x0000, 0x0000, 0x0000, 0xffff),
    // foreground default color
    (0xffff, 0xffff, 0xffff, 0xffff)
];

pub fn map_keystring(s: &str) -> Option<Code> {
    fn show(s: &str) -> Option<Code> {
        Some(Code::Showable(s))
    }

    match s {
        | "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j"
        | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t"
        | "u" | "v" | "x" | "y" | "z" | "w" => show(s),
        "space" => show(" "),
        "BackSpace" => Some(Code::Backspace),
        "Return" => show("\n"),
        "Left" => Some(Code::Left),
        "Right" => Some(Code::Right),
        "Up" => Some(Code::Up),
        "Down" => Some(Code::Down),
        _ => None
    }
}