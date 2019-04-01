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
        | "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
        | "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j"
        | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t"
        | "u" | "v" | "x" | "y" | "z" | "w"
        | "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J"
        | "K" | "L" | "M" | "N" | "O" | "P" | "Q" | "R" | "S" | "T"
        | "U" | "V" | "X" | "Y" | "Z" | "W" => show(s),
        "grave" => show("`"),
        "minus" => show("-"),
        "equal" => show("="),
        "bracketleft" => show("["),
        "bracketright" => show("]"),
        "semicolon" => show(";"),
        "apostrophe" => show("'"),
        "numbersign" => show("#"),
        "backslash" => show("\\"),
        "comma" => show(","),
        "period" => show("."),
        "slash" => show("/"),

        // TODO unicode fix
        // "notsign" => show("¬"),

        "exclam" => show("!"),
        "quotedbl" => show("\""),

        // TODO unicode fix
        //"sterling" => show("£"),

        "dollar" => show("$"),
        "percent" => show("%"),
        "asciicircum" => show("^"),
        "ampersand" => show("&"),
        "asterisk" => show("*"),
        "parenleft" => show("("),
        "parenright" => show(")"),
        "underscore" => show("_"),
        "plus" => show("+"),
        "braceleft" => show("{"),
        "braceright" => show("}"),
        "colon" => show(":"),
        "at" => show("@"),
        "asciitilde" => show("~"),
        "bar" => show("|"),
        "less" => show("<"),
        "greater" => show(">"),
        "question" => show("?"),


        "space" => show(" "),
        "Tab" => show("\t"),
        "Return" => show("\n"),

        // non showable
        "BackSpace" => Some(Code::Backspace),
        "Left" => Some(Code::Left),
        "Right" => Some(Code::Right),
        "Up" => Some(Code::Up),
        "Down" => Some(Code::Down),
        _ => None
    }
}