/// bigtext — render text as large box-drawing block letters.
///
/// Usage:
///   cargo run -- "HELLO WORLD"
///   cargo run -- "LAN SHARE"

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let text = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        "HELLO".to_string()
    };

    let rendered = render(&text.to_uppercase());
    println!("{}", rendered);
}

const GLYPH_HEIGHT: usize = 7;

/// Count display columns. Box-drawing chars are all 1 column wide.
fn display_width(s: &str) -> usize {
    s.chars().count()
}

/// Pad a string on the right with spaces to reach `width` display columns.
fn pad_right(s: &str, width: usize) -> String {
    let w = display_width(s);
    if w >= width {
        s.to_string()
    } else {
        format!("{}{}", s, " ".repeat(width - w))
    }
}

/// Render a string into big block letters.
pub fn render(text: &str) -> String {
    // Raw glyph rows for every character in the input.
    let raw_glyphs: Vec<Vec<&'static str>> = text.chars().map(glyph).collect();

    // Normalize: make every row of a glyph the same display width
    // (pad shorter rows on the right with spaces).
    let normalised: Vec<Vec<String>> = raw_glyphs
        .iter()
        .map(|rows| {
            let max_w = rows.iter().map(|r| display_width(r)).max().unwrap_or(0);
            rows.iter().map(|r| pad_right(r, max_w)).collect()
        })
        .collect();

    // Stitch rows together: one output line per row index.
    // Glyphs are separated by a single space column.
    (0..GLYPH_HEIGHT)
        .map(|row| {
            normalised
                .iter()
                .map(|g| {
                    if row < g.len() {
                        g[row].clone()
                    } else {
                        let w = g.first().map(|r| display_width(r)).unwrap_or(0);
                        " ".repeat(w)
                    }
                })
                .collect::<Vec<_>>()
                .join(" ") // single-column gap between letters
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Return the 7-row raw glyph for a character.
fn glyph(c: char) -> Vec<&'static str> {
    match c {
        'A' => vec![
            " █████╗ ",
            "██╔══██╗",
            "███████║",
            "██╔══██║",
            "██║  ██║",
            "╚═╝  ╚═╝",
            "        ",
        ],
        'B' => vec![
            "██████╗ ",
            "██╔══██╗",
            "██████╔╝",
            "██╔══██╗",
            "██████╔╝",
            "╚═════╝ ",
            "        ",
        ],
        'C' => vec![
            " ██████╗",
            "██╔════╝",
            "██║     ",
            "██║     ",
            "╚██████╗",
            " ╚═════╝",
            "        ",
        ],
        'D' => vec![
            "██████╗ ",
            "██╔══██╗",
            "██║  ██║",
            "██║  ██║",
            "██████╔╝",
            "╚═════╝ ",
            "        ",
        ],
        'E' => vec![
            "███████╗",
            "██╔════╝",
            "█████╗  ",
            "██╔══╝  ",
            "███████╗",
            "╚══════╝",
            "        ",
        ],
        'F' => vec![
            "███████╗",
            "██╔════╝",
            "█████╗  ",
            "██╔══╝  ",
            "██║     ",
            "╚═╝     ",
            "        ",
        ],
        'G' => vec![
            " ██████╗ ",
            "██╔════╝ ",
            "██║  ███╗",
            "██║   ██║",
            "╚██████╔╝",
            " ╚═════╝ ",
            "         ",
        ],
        'H' => vec![
            "██╗  ██╗",
            "██║  ██║",
            "███████║",
            "██╔══██║",
            "██║  ██║",
            "╚═╝  ╚═╝",
            "        ",
        ],
        'I' => vec![
            "██╗",
            "██║",
            "██║",
            "██║",
            "██║",
            "╚═╝",
            "   ",
        ],
        'J' => vec![
            "     ██╗",
            "     ██║",
            "     ██║",
            "██   ██║",
            "╚█████╔╝",
            " ╚════╝ ",
            "        ",
        ],
        'K' => vec![
            "██╗  ██╗",
            "██║ ██╔╝",
            "█████╔╝ ",
            "██╔═██╗ ",
            "██║  ██╗",
            "╚═╝  ╚═╝",
            "        ",
        ],
        'L' => vec![
            "██╗     ",
            "██║     ",
            "██║     ",
            "██║     ",
            "███████╗",
            "╚══════╝",
            "        ",
        ],
        'M' => vec![
            "███╗   ███╗",
            "████╗ ████║",
            "██╔████╔██║",
            "██║╚██╔╝██║",
            "██║ ╚═╝ ██║",
            "╚═╝     ╚═╝",
            "           ",
        ],
        'N' => vec![
            "███╗   ██╗",
            "████╗  ██║",
            "██╔██╗ ██║",
            "██║╚██╗██║",
            "██║ ╚████║",
            "╚═╝  ╚═══╝",
            "          ",
        ],
        'O' => vec![
            " ██████╗ ",
            "██╔═══██╗",
            "██║   ██║",
            "██║   ██║",
            "╚██████╔╝",
            " ╚═════╝ ",
            "         ",
        ],
        'P' => vec![
            "██████╗ ",
            "██╔══██╗",
            "██████╔╝",
            "██╔═══╝ ",
            "██║     ",
            "╚═╝     ",
            "        ",
        ],
        'Q' => vec![
            " ██████╗ ",
            "██╔═══██╗",
            "██║   ██║",
            "██║▄▄ ██║",
            "╚██████╔╝",
            " ╚══▀▀═╝ ",
            "         ",
        ],
        'R' => vec![
            "██████╗ ",
            "██╔══██╗",
            "██████╔╝",
            "██╔══██╗",
            "██║  ██║",
            "╚═╝  ╚═╝",
            "        ",
        ],
        'S' => vec![
            "███████╗",
            "██╔════╝",
            "███████╗",
            "╚════██║",
            "███████║",
            "╚══════╝",
            "        ",
        ],
        'T' => vec![
            "████████╗",
            "╚══██╔══╝",
            "   ██║   ",
            "   ██║   ",
            "   ██║   ",
            "   ╚═╝   ",
            "         ",
        ],
        'U' => vec![
            "██╗   ██╗",
            "██║   ██║",
            "██║   ██║",
            "██║   ██║",
            "╚██████╔╝",
            " ╚═════╝ ",
            "         ",
        ],
        'V' => vec![
            "██╗   ██╗",
            "██║   ██║",
            "██║   ██║",
            "╚██╗ ██╔╝",
            " ╚████╔╝ ",
            "  ╚═══╝  ",
            "         ",
        ],
        'W' => vec![
            "██╗    ██╗",
            "██║    ██║",
            "██║ █╗ ██║",
            "██║███╗██║",
            "╚███╔███╔╝",
            " ╚══╝╚══╝ ",
            "          ",
        ],
        'X' => vec![
            "██╗  ██╗",
            "╚██╗██╔╝",
            " ╚███╔╝ ",
            " ██╔██╗ ",
            "██╔╝ ██╗",
            "╚═╝  ╚═╝",
            "        ",
        ],
        'Y' => vec![
            "██╗   ██╗",
            "╚██╗ ██╔╝",
            " ╚████╔╝ ",
            "  ╚██╔╝  ",
            "   ██║   ",
            "   ╚═╝   ",
            "         ",
        ],
        'Z' => vec![
            "███████╗",
            "╚════██║",
            "    ██╔╝",
            "   ██╔╝ ",
            "███████╗",
            "╚══════╝",
            "        ",
        ],
        '0' => vec![
            " ██████╗ ",
            "██╔═████╗",
            "██║██╔██║",
            "████╔╝██║",
            "╚██████╔╝",
            " ╚═════╝ ",
            "         ",
        ],
        '1' => vec![
            " ██╗",
            "███║",
            "╚██║",
            " ██║",
            " ██║",
            " ╚═╝",
            "    ",
        ],
        '2' => vec![
            "██████╗ ",
            "╚════██╗",
            " █████╔╝",
            "██╔═══╝ ",
            "███████╗",
            "╚══════╝",
            "        ",
        ],
        '3' => vec![
            "██████╗ ",
            "╚════██╗",
            " █████╔╝",
            " ╚═══██╗",
            "██████╔╝",
            "╚═════╝ ",
            "        ",
        ],
        '4' => vec![
            "██╗  ██╗",
            "██║  ██║",
            "███████║",
            "╚════██║",
            "     ██║",
            "     ╚═╝",
            "        ",
        ],
        '5' => vec![
            "███████╗",
            "██╔════╝",
            "███████╗",
            "╚════██║",
            "███████║",
            "╚══════╝",
            "        ",
        ],
        '6' => vec![
            " ██████╗ ",
            "██╔════╝ ",
            "███████╗ ",
            "██╔═══██╗",
            "╚██████╔╝",
            " ╚═════╝ ",
            "         ",
        ],
        '7' => vec![
            "███████╗",
            "╚════██║",
            "    ██╔╝",
            "   ██╔╝ ",
            "   ██║  ",
            "   ╚═╝  ",
            "        ",
        ],
        '8' => vec![
            " █████╗ ",
            "██╔══██╗",
            "╚█████╔╝",
            "██╔══██╗",
            "╚█████╔╝",
            " ╚════╝ ",
            "        ",
        ],
        '9' => vec![
            " █████╗ ",
            "██╔══██╗",
            "╚██████║",
            " ╚═══██║",
            " █████╔╝",
            " ╚════╝ ",
            "        ",
        ],
        '!' => vec![
            "██╗",
            "██║",
            "██║",
            "╚═╝",
            "██╗",
            "╚═╝",
            "   ",
        ],
        '?' => vec![
            "██████╗ ",
            "╚════██╗",
            "  ▄███╔╝",
            "  ▀▀══╝ ",
            "  ██╗   ",
            "  ╚═╝   ",
            "        ",
        ],
        '.' => vec![
            "   ",
            "   ",
            "   ",
            "   ",
            "██╗",
            "╚═╝",
            "   ",
        ],
        ',' => vec![
            "    ",
            "    ",
            "    ",
            "    ",
            "▄██╗",
            "╚══╝",
            "    ",
        ],
        '-' => vec![
            "       ",
            "       ",
            "██████╗",
            "╚═════╝",
            "       ",
            "       ",
            "       ",
        ],
        '_' => vec![
            "        ",
            "        ",
            "        ",
            "        ",
            "        ",
            "███████╗",
            "╚══════╝",
        ],
        ':' => vec![
            "   ",
            "██╗",
            "╚═╝",
            "██╗",
            "╚═╝",
            "   ",
            "   ",
        ],
        '/' => vec![
            "      ██╗",
            "     ██╔╝",
            "    ██╔╝ ",
            "   ██╔╝  ",
            "  ██╔╝   ",
            " ╚═╝     ",
            "         ",
        ],
        ' ' => vec![
            "    ",
            "    ",
            "    ",
            "    ",
            "    ",
            "    ",
            "    ",
        ],
        _ => vec![
            "      ",
            "  ██╗ ",
            "  ╚═╝ ",
            "  ██╗ ",
            "  ╚═╝ ",
            "      ",
            "      ",
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_glyphs_correct_height() {
        for ch in "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 !?.-_:/,".chars() {
            let g = glyph(ch);
            assert_eq!(g.len(), GLYPH_HEIGHT,
                       "glyph '{}' has {} rows, expected {}", ch, g.len(), GLYPH_HEIGHT);
        }
    }

    #[test]
    fn render_all_lines_same_width() {
        let out = render("HELLO WORLD");
        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(lines.len(), GLYPH_HEIGHT);
        let first_w = display_width(lines[0]);
        for l in &lines {
            assert_eq!(display_width(l), first_w,
                       "line width mismatch:\n{}", out);
        }
    }
}
