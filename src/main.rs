use std::env;
use std::process;

const ROWS: usize = 5;

#[derive(Clone, Copy)]
struct Glyph {
    width: u8,
    rows: [u8; ROWS],
}

static FONT_UPPER: &[(char, Glyph)] = &[
    ('A', Glyph { width: 5, rows: [0b01110, 0b10001, 0b11111, 0b10001, 0b10001] }),
    ('B', Glyph { width: 5, rows: [0b11110, 0b10001, 0b11110, 0b10001, 0b11110] }),
    ('C', Glyph { width: 5, rows: [0b01110, 0b10001, 0b10000, 0b10001, 0b01110] }),
    ('D', Glyph { width: 5, rows: [0b11110, 0b10001, 0b10001, 0b10001, 0b11110] }),
    ('E', Glyph { width: 5, rows: [0b11111, 0b10000, 0b11110, 0b10000, 0b11111] }),
    ('F', Glyph { width: 5, rows: [0b11111, 0b10000, 0b11110, 0b10000, 0b10000] }),
    ('G', Glyph { width: 5, rows: [0b01110, 0b10001, 0b10000, 0b10111, 0b01110] }),
    ('H', Glyph { width: 5, rows: [0b10001, 0b10001, 0b11111, 0b10001, 0b10001] }),
    ('I', Glyph { width: 3, rows: [0b111, 0b010, 0b010, 0b010, 0b111] }),
    ('J', Glyph { width: 4, rows: [0b0011, 0b0001, 0b0001, 0b1001, 0b0110] }),
    ('K', Glyph { width: 5, rows: [0b10001, 0b10010, 0b11100, 0b10010, 0b10001] }),
    ('L', Glyph { width: 5, rows: [0b10000, 0b10000, 0b10000, 0b10000, 0b11111] }),
    ('M', Glyph { width: 5, rows: [0b10001, 0b11011, 0b10101, 0b10001, 0b10001] }),
    ('N', Glyph { width: 5, rows: [0b10001, 0b11001, 0b10101, 0b10011, 0b10001] }),
    ('O', Glyph { width: 5, rows: [0b01110, 0b10001, 0b10001, 0b10001, 0b01110] }),
    ('P', Glyph { width: 5, rows: [0b11110, 0b10001, 0b11110, 0b10000, 0b10000] }),
    ('Q', Glyph { width: 5, rows: [0b01110, 0b10001, 0b10001, 0b10001, 0b01111] }),
    ('R', Glyph { width: 5, rows: [0b11110, 0b10001, 0b11110, 0b10010, 0b10001] }),
    ('S', Glyph { width: 5, rows: [0b01111, 0b10000, 0b01110, 0b00001, 0b11110] }),
    ('T', Glyph { width: 5, rows: [0b11111, 0b00100, 0b00100, 0b00100, 0b00100] }),
    ('U', Glyph { width: 5, rows: [0b10001, 0b10001, 0b10001, 0b10001, 0b01110] }),
    ('V', Glyph { width: 5, rows: [0b10001, 0b10001, 0b10001, 0b01010, 0b00100] }),
    ('W', Glyph { width: 5, rows: [0b10001, 0b10001, 0b10101, 0b11011, 0b10001] }),
    ('X', Glyph { width: 5, rows: [0b10001, 0b01010, 0b00100, 0b01010, 0b10001] }),
    ('Y', Glyph { width: 5, rows: [0b10001, 0b01010, 0b00100, 0b00100, 0b00100] }),
    ('Z', Glyph { width: 5, rows: [0b11111, 0b00010, 0b00100, 0b01000, 0b11111] }),
    ('0', Glyph { width: 5, rows: [0b01110, 0b10011, 0b10101, 0b11001, 0b01110] }),
    ('1', Glyph { width: 5, rows: [0b00100, 0b01100, 0b00100, 0b00100, 0b01110] }),
    ('2', Glyph { width: 5, rows: [0b01110, 0b10001, 0b00010, 0b00100, 0b11111] }),
    ('3', Glyph { width: 5, rows: [0b11110, 0b00001, 0b01110, 0b00001, 0b11110] }),
    ('4', Glyph { width: 5, rows: [0b10010, 0b10010, 0b11111, 0b00010, 0b00010] }),
    ('5', Glyph { width: 5, rows: [0b11111, 0b10000, 0b11110, 0b00001, 0b11110] }),
    ('6', Glyph { width: 5, rows: [0b01110, 0b10000, 0b11110, 0b10001, 0b01110] }),
    ('7', Glyph { width: 5, rows: [0b11111, 0b00001, 0b00010, 0b00100, 0b01000] }),
    ('8', Glyph { width: 5, rows: [0b01110, 0b10001, 0b01110, 0b10001, 0b01110] }),
    ('9', Glyph { width: 5, rows: [0b01110, 0b10001, 0b01111, 0b00001, 0b01110] }),
    (' ', Glyph { width: 3, rows: [0, 0, 0, 0, 0] }),
    ('.', Glyph { width: 1, rows: [0, 0, 0, 0, 1] }),
    (',', Glyph { width: 1, rows: [0, 0, 0, 0, 1] }),
    ('!', Glyph { width: 1, rows: [1, 1, 1, 0, 1] }),
    ('?', Glyph { width: 5, rows: [0b01110, 0b10001, 0b00110, 0b00000, 0b00100] }),
    ('-', Glyph { width: 5, rows: [0, 0, 0b11111, 0, 0] }),
    ('_', Glyph { width: 5, rows: [0, 0, 0, 0, 0b11111] }),
    (':', Glyph { width: 1, rows: [0, 1, 0, 1, 0] }),
    ('/', Glyph { width: 5, rows: [0b00001, 0b00010, 0b00100, 0b01000, 0b10000] }),
    ('+', Glyph { width: 5, rows: [0, 0b00100, 0b11111, 0b00100, 0] }),
    ('=', Glyph { width: 5, rows: [0, 0b11111, 0, 0b11111, 0] }),
    ('*', Glyph { width: 5, rows: [0b00100, 0b10101, 0b01110, 0b10101, 0b00100] }),
    ('#', Glyph { width: 5, rows: [0b01010, 0b11111, 0b01010, 0b11111, 0b01010] }),
    ('(', Glyph { width: 3, rows: [0b001, 0b010, 0b010, 0b010, 0b001] }),
    (')', Glyph { width: 3, rows: [0b100, 0b010, 0b010, 0b010, 0b100] }),
];

static FONT_LOWER: &[(char, Glyph)] = &[
    ('a', Glyph { width: 5, rows: [0b00000, 0b01110, 0b10001, 0b10001, 0b01111] }),
    ('b', Glyph { width: 5, rows: [0b10000, 0b11110, 0b10001, 0b10001, 0b11110] }),
    ('c', Glyph { width: 5, rows: [0b00000, 0b01110, 0b10000, 0b10000, 0b01110] }),
    ('d', Glyph { width: 5, rows: [0b00001, 0b01111, 0b10001, 0b10001, 0b01111] }),
    ('e', Glyph { width: 5, rows: [0b00000, 0b01110, 0b10001, 0b11111, 0b01110] }),
    ('f', Glyph { width: 4, rows: [0b0111, 0b1000, 0b1110, 0b1000, 0b1000] }),
    ('g', Glyph { width: 5, rows: [0b00000, 0b01111, 0b10001, 0b01111, 0b00010] }),
    ('h', Glyph { width: 5, rows: [0b10000, 0b11110, 0b10001, 0b10001, 0b10001] }),
    ('i', Glyph { width: 3, rows: [0b010, 0b000, 0b010, 0b010, 0b111] }),
    ('j', Glyph { width: 3, rows: [0b010, 0b000, 0b010, 0b010, 0b100] }),
    ('k', Glyph { width: 5, rows: [0b10000, 0b10010, 0b10100, 0b11000, 0b10100] }),
    ('l', Glyph { width: 1, rows: [0b1, 0b1, 0b1, 0b1, 0b1] }),
    ('m', Glyph { width: 5, rows: [0b00000, 0b11011, 0b10101, 0b10101, 0b10101] }),
    ('n', Glyph { width: 5, rows: [0b00000, 0b11110, 0b10001, 0b10001, 0b10001] }),
    ('o', Glyph { width: 5, rows: [0b00000, 0b01110, 0b10001, 0b10001, 0b01110] }),
    ('p', Glyph { width: 5, rows: [0b00000, 0b11110, 0b10001, 0b11110, 0b10000] }),
    ('q', Glyph { width: 5, rows: [0b00000, 0b01111, 0b10001, 0b01111, 0b00001] }),
    ('r', Glyph { width: 5, rows: [0b00000, 0b11110, 0b10000, 0b10000, 0b10000] }),
    ('s', Glyph { width: 5, rows: [0b01111, 0b10000, 0b01110, 0b00001, 0b11110] }),
    ('t', Glyph { width: 3, rows: [0b010, 0b111, 0b010, 0b010, 0b011] }),
    ('u', Glyph { width: 5, rows: [0b00000, 0b10001, 0b10001, 0b10001, 0b01111] }),
    ('v', Glyph { width: 5, rows: [0b00000, 0b10001, 0b10001, 0b01010, 0b00100] }),
    ('w', Glyph { width: 5, rows: [0b00000, 0b10001, 0b10001, 0b10101, 0b01010] }),
    ('x', Glyph { width: 5, rows: [0b00000, 0b10001, 0b01010, 0b00100, 0b01010] }),
    ('y', Glyph { width: 5, rows: [0b00000, 0b10001, 0b10001, 0b01111, 0b00001] }),
    ('z', Glyph { width: 5, rows: [0b00000, 0b11111, 0b00110, 0b01100, 0b11111] }),
];

fn lookup(c: char, lowercase: bool) -> Glyph {
    if lowercase {
        for (ch, glyph) in FONT_LOWER {
            if *ch == c {
                return *glyph;
            }
        }
    }
    for (ch, glyph) in FONT_UPPER {
        if *ch == c {
            return *glyph;
        }
    }
    Glyph { width: 3, rows: [0, 0, 0, 0, 0] }
}

fn parse_color(s: &str) -> Option<(u8, u8, u8)> {
    let named = |r, g, b| Some((r, g, b));
    match s.to_ascii_lowercase().as_str() {
        "black" => return named(0, 0, 0),
        "red" => return named(255, 0, 0),
        "green" => return named(0, 255, 0),
        "blue" => return named(0, 0, 255),
        "yellow" => return named(255, 255, 0),
        "cyan" => return named(0, 255, 255),
        "magenta" => return named(255, 0, 255),
        "white" => return named(255, 255, 255),
        "gray" | "grey" => return named(128, 128, 128),
        "orange" => return named(255, 165, 0),
        "purple" => return named(128, 0, 128),
        _ => {}
    }
    let hex = s.strip_prefix('#').unwrap_or(s);
    if hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        return Some((r, g, b));
    }
    None
}

fn usage() {
    eprintln!("blockhead - render text as big block-letter art");
    eprintln!();
    eprintln!("usage: blockhead [--color <hex|name>] [--spacing <n>] <text>");
    eprintln!();
    eprintln!("options:");
    eprintln!("  --color <hex|name>   truecolor output (e.g. #ff8800, red)");
    eprintln!("  --spacing <n>        columns between letters (default 1)");
    eprintln!("  --shade              vertical fade: top row bright, dims each row down");
    eprintln!("  --lowercase          render lowercase letters (default: uppercase)");
    eprintln!("  -h, --help           show this help");
}

fn pixel(glyph: &Glyph, row: usize, col: usize) -> bool {
    if row >= ROWS {
        return false;
    }
    if col as u8 >= glyph.width {
        return false;
    }
    let shift = glyph.width - 1 - col as u8;
    (glyph.rows[row] >> shift) & 1 == 1
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        usage();
        process::exit(1);
    }

    let mut color: Option<(u8, u8, u8)> = None;
    let mut spacing: usize = 1;
    let mut shade = false;
    let mut lowercase = false;
    let mut words: Vec<String> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        let a = args[i].clone();

        let value_for = |name: &str, inline: Option<&str>, i: &mut usize| -> Option<String> {
            if let Some(v) = inline {
                return Some(v.to_string());
            }
            if *i + 1 < args.len() {
                *i += 1;
                return Some(args[*i].clone());
            }
            eprintln!("blockhead: {} needs a value", name);
            None
        };

        if a == "-h" || a == "--help" {
            usage();
            return;
        } else if a == "--color" || a.starts_with("--color=") {
            let inline = a.strip_prefix("--color=");
            match value_for("--color", inline, &mut i) {
                Some(v) => match parse_color(&v) {
                    Some(c) => color = Some(c),
                    None => {
                        eprintln!("blockhead: invalid color '{}'", v);
                        process::exit(1);
                    }
                },
                None => process::exit(1),
            }
        } else if a == "--spacing" || a.starts_with("--spacing=") {
            let inline = a.strip_prefix("--spacing=");
            match value_for("--spacing", inline, &mut i) {
                Some(v) => match v.parse::<usize>() {
                    Ok(n) => spacing = n,
                    Err(_) => {
                        eprintln!("blockhead: --spacing needs a number");
                        process::exit(1);
                    }
                },
                None => process::exit(1),
            }
        } else if a == "--shade" || a == "--gradient" {
            shade = true;
        } else if a == "--lowercase" || a == "--lower" || a == "-l" {
            lowercase = true;
        } else if a.starts_with('-') && a.len() > 1 {
            eprintln!("blockhead: unknown flag '{}'", a);
            usage();
            process::exit(1);
        } else {
            words.push(a);
        }
        i += 1;
    }

    if words.is_empty() {
        usage();
        process::exit(1);
    }

    let text = if lowercase {
        words.join(" ").to_ascii_lowercase()
    } else {
        words.join(" ").to_ascii_uppercase()
    };
    let glyphs: Vec<Glyph> = text.chars().map(|c| lookup(c, lowercase)).collect();

    let (cr, cg, cb) = color.unwrap_or(if shade { (255, 255, 255) } else { (0, 0, 0) });
    let use_color = color.is_some() || shade;

    let bands = (ROWS + 1) / 2;
    for band in 0..bands {
        let fade = 1.0 - band as f32 / bands as f32;
        let (br, bg, bb) = if shade {
            (
                (cr as f32 * fade) as u8,
                (cg as f32 * fade) as u8,
                (cb as f32 * fade) as u8,
            )
        } else {
            (cr, cg, cb)
        };
        let mut line = String::new();
        for (gi, glyph) in glyphs.iter().enumerate() {
            if gi > 0 {
                for _ in 0..spacing {
                    line.push(' ');
                }
            }
            for col in 0..glyph.width as usize {
                let top = pixel(glyph, band * 2, col);
                let bottom = pixel(glyph, band * 2 + 1, col);
                line.push(match (top, bottom) {
                    (true, true) => '\u{2588}',
                    (true, false) => '\u{2580}',
                    (false, true) => '\u{2584}',
                    (false, false) => ' ',
                });
            }
        }
        if use_color {
            println!("\x1b[38;2;{};{};{}m{}\x1b[0m", br, bg, bb, line);
        } else {
            println!("{}", line);
        }
    }
}
