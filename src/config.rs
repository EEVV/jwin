use crate::Code;

// <FONT NAME>-<FONT SIZE>
pub const FONT: &'static str = "Fira Mono-14";

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
        Some(Code::Showable(s.to_string()))
    }

    match s {
        | "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j"
        | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t"
        | "u" | "v" | "x" | "y" | "z" | "w" => show(s),
        "space" => show(" "),
        "BackSpace" => Some(Code::Backspace),
        "Return" => Some(Code::Return),
        _ => None
    }
}