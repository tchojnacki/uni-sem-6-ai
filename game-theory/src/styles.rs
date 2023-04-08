use colored::Color;

pub const EMPTY_FG: Color = Color::TrueColor {
    r: 0,
    g: 115,
    b: 82,
};

pub const EMPTY_BG: Color = Color::TrueColor {
    r: 0,
    g: 144,
    b: 103,
};

pub const BLACK_FG: Color = Color::TrueColor {
    r: 47,
    g: 65,
    b: 60,
};

pub const BLACK_BG: Color = Color::TrueColor {
    r: 19,
    g: 26,
    b: 24,
};

pub const WHITE_FG: Color = Color::TrueColor {
    r: 195,
    g: 202,
    b: 200,
};

pub const WHITE_BG: Color = Color::TrueColor {
    r: 244,
    g: 253,
    b: 250,
};

pub const VALID_FG: Color = Color::TrueColor {
    r: 115,
    g: 57,
    b: 74,
};

pub fn strip_string(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars();
    while let Some(mut char) = chars.next() {
        // Skip ANSI escape codes
        if char == '\u{1b}' {
            while char != 'm' {
                char = chars.next().unwrap_or('m');
            }
        }
        // Accept only 0, 1 and 2
        if matches!(char, '0' | '1' | '2') {
            result.push(char);
        }
    }
    result
}
