use std::{collections::HashMap, error::Error, fmt::Display, str::FromStr, sync::LazyLock};

pub static NAMED_COLORS: LazyLock<HashMap<&str, Color>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("aliceblue", Color::Hex(0xf0, 0xf8, 0xff));
    m.insert("antiquewhite", Color::Hex(0xfa, 0xeb, 0xd7));
    m.insert("aqua", Color::Hex(0x00, 0xff, 0xff));
    m.insert("aquamarine", Color::Hex(0x7f, 0xff, 0xd4));
    m.insert("azure", Color::Hex(0xf0, 0xff, 0xff));
    m.insert("beige", Color::Hex(0xf5, 0xf5, 0xdc));
    m.insert("bisque", Color::Hex(0xff, 0xe4, 0xc4));
    m.insert("black", Color::Hex(0x00, 0x00, 0x00));
    m.insert("blanchedalmond", Color::Hex(0xff, 0xeb, 0xcd));
    m.insert("blue", Color::Hex(0x00, 0x00, 0xff));
    m.insert("blueviolet", Color::Hex(0x8a, 0x2b, 0xe2));
    m.insert("brown", Color::Hex(0xa5, 0x2a, 0x2a));
    m.insert("burlywood", Color::Hex(0xde, 0xb8, 0x87));
    m.insert("cadetblue", Color::Hex(0x5f, 0x9e, 0xa0));
    m.insert("chartreuse", Color::Hex(0x7f, 0xff, 0x00));
    m.insert("chocolate", Color::Hex(0xd2, 0x69, 0x1e));
    m.insert("coral", Color::Hex(0xff, 0x7f, 0x50));
    m.insert("cornflowerblue", Color::Hex(0x64, 0x95, 0xed));
    m.insert("cornsilk", Color::Hex(0xff, 0xf8, 0xdc));
    m.insert("crimson", Color::Hex(0xdc, 0x14, 0x3c));
    m.insert("cyan", Color::Hex(0x00, 0xff, 0xff));
    m.insert("darkblue", Color::Hex(0x00, 0x00, 0x8b));
    m.insert("darkcyan", Color::Hex(0x00, 0x8b, 0x8b));
    m.insert("darkgoldenrod", Color::Hex(0xb8, 0x86, 0x0b));
    m.insert("darkgray", Color::Hex(0xa9, 0xa9, 0xa9));
    m.insert("darkgreen", Color::Hex(0x00, 0x64, 0x00));
    m.insert("darkgrey", Color::Hex(0xa9, 0xa9, 0xa9));
    m.insert("darkkhaki", Color::Hex(0xbd, 0xb7, 0x6b));
    m.insert("darkmagenta", Color::Hex(0x8b, 0x00, 0x8b));
    m.insert("darkolivegreen", Color::Hex(0x55, 0x6b, 0x2f));
    m.insert("darkorange", Color::Hex(0xff, 0x8c, 0x00));
    m.insert("darkorchid", Color::Hex(0x99, 0x32, 0xcc));
    m.insert("darkred", Color::Hex(0x8b, 0x00, 0x00));
    m.insert("darksalmon", Color::Hex(0xe9, 0x96, 0x7a));
    m.insert("darkseagreen", Color::Hex(0x8f, 0xbc, 0x8f));
    m.insert("darkslateblue", Color::Hex(0x48, 0x3d, 0x8b));
    m.insert("darkslategray", Color::Hex(0x2f, 0x4f, 0x4f));
    m.insert("darkslategrey", Color::Hex(0x2f, 0x4f, 0x4f));
    m.insert("darkturquoise", Color::Hex(0x00, 0xce, 0xd1));
    m.insert("darkviolet", Color::Hex(0x94, 0x00, 0xd3));
    m.insert("deeppink", Color::Hex(0xff, 0x14, 0x93));
    m.insert("deepskyblue", Color::Hex(0x00, 0xbf, 0xff));
    m.insert("dimgray", Color::Hex(0x69, 0x69, 0x69));
    m.insert("dimgrey", Color::Hex(0x69, 0x69, 0x69));
    m.insert("dodgerblue", Color::Hex(0x1e, 0x90, 0xff));
    m.insert("firebrick", Color::Hex(0xb2, 0x22, 0x22));
    m.insert("floralwhite", Color::Hex(0xff, 0xfa, 0xf0));
    m.insert("forestgreen", Color::Hex(0x22, 0x8b, 0x22));
    m.insert("fuchsia", Color::Hex(0xff, 0x00, 0xff));
    m.insert("gainsboro", Color::Hex(0xdc, 0xdc, 0xdc));
    m.insert("ghostwhite", Color::Hex(0xf8, 0xf8, 0xff));
    m.insert("gold", Color::Hex(0xff, 0xd7, 0x00));
    m.insert("goldenrod", Color::Hex(0xda, 0xa5, 0x20));
    m.insert("gray", Color::Hex(0x80, 0x80, 0x80));
    m.insert("green", Color::Hex(0x00, 0x80, 0x00));
    m.insert("greenyellow", Color::Hex(0xad, 0xff, 0x2f));
    m.insert("grey", Color::Hex(0x80, 0x80, 0x80));
    m.insert("honeydew", Color::Hex(0xf0, 0xff, 0xf0));
    m.insert("hotpink", Color::Hex(0xff, 0x69, 0xb4));
    m.insert("indianred", Color::Hex(0xcd, 0x5c, 0x5c));
    m.insert("indigo", Color::Hex(0x4b, 0x00, 0x82));
    m.insert("ivory", Color::Hex(0xff, 0xff, 0xf0));
    m.insert("khaki", Color::Hex(0xf0, 0xe6, 0x8c));
    m.insert("lavender", Color::Hex(0xe6, 0xe6, 0xfa));
    m.insert("lavenderblush", Color::Hex(0xff, 0xf0, 0xf5));
    m.insert("lawngreen", Color::Hex(0x7c, 0xfc, 0x00));
    m.insert("lemonchiffon", Color::Hex(0xff, 0xfa, 0xcd));
    m.insert("lightblue", Color::Hex(0xad, 0xd8, 0xe6));
    m.insert("lightcoral", Color::Hex(0xf0, 0x80, 0x80));
    m.insert("lightcyan", Color::Hex(0xe0, 0xff, 0xff));
    m.insert("lightgoldenrodyellow", Color::Hex(0xfa, 0xfa, 0xd2));
    m.insert("lightgray", Color::Hex(0xd3, 0xd3, 0xd3));
    m.insert("lightgreen", Color::Hex(0x90, 0xee, 0x90));
    m.insert("lightgrey", Color::Hex(0xd3, 0xd3, 0xd3));
    m.insert("lightpink", Color::Hex(0xff, 0xb6, 0xc1));
    m.insert("lightsalmon", Color::Hex(0xff, 0xa0, 0x7a));
    m.insert("lightseagreen", Color::Hex(0x20, 0xb2, 0xaa));
    m.insert("lightskyblue", Color::Hex(0x87, 0xce, 0xfa));
    m.insert("lightslategray", Color::Hex(0x77, 0x88, 0x99));
    m.insert("lightslategrey", Color::Hex(0x77, 0x88, 0x99));
    m.insert("lightsteelblue", Color::Hex(0xb0, 0xc4, 0xde));
    m.insert("lightyellow", Color::Hex(0xff, 0xff, 0xe0));
    m.insert("lime", Color::Hex(0x00, 0xff, 0x00));
    m.insert("limegreen", Color::Hex(0x32, 0xcd, 0x32));
    m.insert("linen", Color::Hex(0xfa, 0xf0, 0xe6));
    m.insert("magenta", Color::Hex(0xff, 0x00, 0xff));
    m.insert("maroon", Color::Hex(0x80, 0x00, 0x00));
    m.insert("mediumaquamarine", Color::Hex(0x66, 0xcd, 0xaa));
    m.insert("mediumblue", Color::Hex(0x00, 0x00, 0xcd));
    m.insert("mediumorchid", Color::Hex(0xba, 0x55, 0xd3));
    m.insert("mediumpurple", Color::Hex(0x93, 0x70, 0xdb));
    m.insert("mediumseagreen", Color::Hex(0x3c, 0xb3, 0x71));
    m.insert("mediumslateblue", Color::Hex(0x7b, 0x68, 0xee));
    m.insert("mediumspringgreen", Color::Hex(0x00, 0xfa, 0x9a));
    m.insert("mediumturquoise", Color::Hex(0x48, 0xd1, 0xcc));
    m.insert("mediumvioletred", Color::Hex(0xc7, 0x15, 0x85));
    m.insert("midnightblue", Color::Hex(0x19, 0x19, 0x70));
    m.insert("mintcream", Color::Hex(0xf5, 0xff, 0xfa));
    m.insert("mistyrose", Color::Hex(0xff, 0xe4, 0xe1));
    m.insert("moccasin", Color::Hex(0xff, 0xe4, 0xb5));
    m.insert("navajowhite", Color::Hex(0xff, 0xde, 0xad));
    m.insert("navy", Color::Hex(0x00, 0x00, 0x80));
    m.insert("oldlace", Color::Hex(0xfd, 0xf5, 0xe6));
    m.insert("olive", Color::Hex(0x80, 0x80, 0x00));
    m.insert("olivedrab", Color::Hex(0x6b, 0x8e, 0x23));
    m.insert("orange", Color::Hex(0xff, 0xa5, 0x00));
    m.insert("orangered", Color::Hex(0xff, 0x45, 0x00));
    m.insert("orchid", Color::Hex(0xda, 0x70, 0xd6));
    m.insert("palegoldenrod", Color::Hex(0xee, 0xe8, 0xaa));
    m.insert("palegreen", Color::Hex(0x98, 0xfb, 0x98));
    m.insert("paleturquoise", Color::Hex(0xaf, 0xee, 0xee));
    m.insert("palevioletred", Color::Hex(0xdb, 0x70, 0x93));
    m.insert("papayawhip", Color::Hex(0xff, 0xef, 0xd5));
    m.insert("peachpuff", Color::Hex(0xff, 0xda, 0xb9));
    m.insert("peru", Color::Hex(0xcd, 0x85, 0x3f));
    m.insert("pink", Color::Hex(0xff, 0xc0, 0xcb));
    m.insert("plum", Color::Hex(0xdd, 0xa0, 0xdd));
    m.insert("powderblue", Color::Hex(0xb0, 0xe0, 0xe6));
    m.insert("purple", Color::Hex(0x80, 0x00, 0x80));
    m.insert("rebeccapurple", Color::Hex(0x66, 0x33, 0x99));
    m.insert("red", Color::Hex(0xff, 0x00, 0x00));
    m.insert("rosybrown", Color::Hex(0xbc, 0x8f, 0x8f));
    m.insert("royalblue", Color::Hex(0x41, 0x69, 0xe1));
    m.insert("saddlebrown", Color::Hex(0x8b, 0x45, 0x13));
    m.insert("salmon", Color::Hex(0xfa, 0x80, 0x72));
    m.insert("sandybrown", Color::Hex(0xf4, 0xa4, 0x60));
    m.insert("seagreen", Color::Hex(0x2e, 0x8b, 0x57));
    m.insert("seashell", Color::Hex(0xff, 0xf5, 0xee));
    m.insert("sienna", Color::Hex(0xa0, 0x52, 0x2d));
    m.insert("silver", Color::Hex(0xc0, 0xc0, 0xc0));
    m.insert("skyblue", Color::Hex(0x87, 0xce, 0xeb));
    m.insert("slateblue", Color::Hex(0x6a, 0x5a, 0xcd));
    m.insert("slategray", Color::Hex(0x70, 0x80, 0x90));
    m.insert("slategrey", Color::Hex(0x70, 0x80, 0x90));
    m.insert("snow", Color::Hex(0xff, 0xfa, 0xfa));
    m.insert("springgreen", Color::Hex(0x00, 0xff, 0x7f));
    m.insert("steelblue", Color::Hex(0x46, 0x82, 0xb4));
    m.insert("tan", Color::Hex(0xd2, 0xb4, 0x8c));
    m.insert("teal", Color::Hex(0x00, 0x80, 0x80));
    m.insert("thistle", Color::Hex(0xd8, 0xbf, 0xd8));
    m.insert("tomato", Color::Hex(0xff, 0x63, 0x47));
    m.insert("turquoise", Color::Hex(0x40, 0xe0, 0xd0));
    m.insert("violet", Color::Hex(0xee, 0x82, 0xee));
    m.insert("wheat", Color::Hex(0xf5, 0xde, 0xb3));
    m.insert("white", Color::Hex(0xff, 0xff, 0xff));
    m.insert("whitesmoke", Color::Hex(0xf5, 0xf5, 0xf5));
    m.insert("yellow", Color::Hex(0xff, 0xff, 0x00));
    m.insert("yellowgreen", Color::Hex(0x9a, 0xcd, 0x32));
    m
});

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    #[default]
    Default,
    Ansi(AnsiColor),
    Hex(u8, u8, u8),
}

impl Color {
    fn parse(s: &str) -> Option<Self> {
        if let Ok(ansi) = s.parse::<AnsiColor>() {
            Some(Color::Ansi(ansi))
        } else if let Some(color) = NAMED_COLORS.get(s.to_lowercase().as_str()) {
            Some(*color)
        } else if s.starts_with('#') && s.len() == 7 {
            // Parse hex color like #RRGGBB
            let r = u8::from_str_radix(&s[1..3], 16).ok()?;
            let g = u8::from_str_radix(&s[3..5], 16).ok()?;
            let b = u8::from_str_radix(&s[5..7], 16).ok()?;
            Some(Color::Hex(r, g, b))
        } else if s.starts_with('#') && s.len() == 4 {
            // Parse 3 digit hex color like #RGB
            // Parse hex color like #RGB where each digit is duplicated
            let r = u8::from_str_radix(&s[1..2].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&s[2..3].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&s[3..4].repeat(2), 16).ok()?;
            Some(Color::Hex(r, g, b))
        } else if s == "default" || s.is_empty() {
            Some(Color::Default)
        } else {
            None
        }
    }

    pub fn rgb(self) -> (u8, u8, u8) {
        match self {
            Color::Default => (0, 0, 0),
            Color::Ansi(ansi_color) => ansi_color.rgb(),
            Color::Hex(r, g, b) => (r, g, b),
        }
    }

    pub fn closest_ansi(self) -> AnsiColor {
        match self {
            Color::Default => AnsiColor::Default,
            Color::Ansi(ansi_color) => ansi_color,
            Color::Hex(r, g, b) => AnsiColor::closest_from_rgb(r, g, b, &[]),
        }
    }
}

impl FromStr for Color {
    type Err = ColorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Color::parse(s).ok_or(ColorParseError(s.to_string()))
    }
}

#[derive(Debug)]
pub struct ColorParseError(String);
impl Error for ColorParseError {}
impl Display for ColorParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not parse \"{}\" as a color", self.0)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AnsiColor {
    #[default]
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl AnsiColor {
    pub fn code(self) -> i32 {
        match self {
            AnsiColor::Default => 39,
            AnsiColor::Black => 30,
            AnsiColor::Red => 31,
            AnsiColor::Green => 32,
            AnsiColor::Yellow => 33,
            AnsiColor::Blue => 34,
            AnsiColor::Magenta => 35,
            AnsiColor::Cyan => 36,
            AnsiColor::White => 37,
            AnsiColor::BrightBlack => 90,
            AnsiColor::BrightRed => 91,
            AnsiColor::BrightGreen => 92,
            AnsiColor::BrightYellow => 93,
            AnsiColor::BrightBlue => 94,
            AnsiColor::BrightMagenta => 95,
            AnsiColor::BrightCyan => 96,
            AnsiColor::BrightWhite => 97,
        }
    }
    pub fn background_code(self) -> i32 {
        match self {
            AnsiColor::Default => 49,
            AnsiColor::Black => 40,
            AnsiColor::Red => 41,
            AnsiColor::Green => 42,
            AnsiColor::Yellow => 43,
            AnsiColor::Blue => 44,
            AnsiColor::Magenta => 45,
            AnsiColor::Cyan => 46,
            AnsiColor::White => 47,
            AnsiColor::BrightBlack => 100,
            AnsiColor::BrightRed => 101,
            AnsiColor::BrightGreen => 102,
            AnsiColor::BrightYellow => 103,
            AnsiColor::BrightBlue => 104,
            AnsiColor::BrightMagenta => 105,
            AnsiColor::BrightCyan => 106,
            AnsiColor::BrightWhite => 107,
        }
    }

    pub fn rgb(self) -> (u8, u8, u8) {
        match self {
            AnsiColor::Default | AnsiColor::Black => (0, 0, 0), // Default to black
            AnsiColor::Red => (205, 0, 0),
            AnsiColor::Green => (0, 205, 0),
            AnsiColor::Yellow => (205, 205, 0),
            AnsiColor::Blue => (0, 0, 238),
            AnsiColor::Magenta => (205, 0, 205),
            AnsiColor::Cyan => (0, 205, 205),
            AnsiColor::White => (229, 229, 229),
            AnsiColor::BrightBlack => (127, 127, 127),
            AnsiColor::BrightRed => (255, 0, 0),
            AnsiColor::BrightGreen => (0, 255, 0),
            AnsiColor::BrightYellow => (255, 255, 0),
            AnsiColor::BrightBlue => (92, 92, 255),
            AnsiColor::BrightMagenta => (255, 0, 255),
            AnsiColor::BrightCyan => (0, 255, 255),
            AnsiColor::BrightWhite => (255, 255, 255),
        }
    }

    fn try_from_str(s: &str) -> Option<Self> {
        match s {
            "ansidefault" => Some(Self::Default),
            "ansiblack" => Some(Self::Black),
            "ansired" | "ansidarkred" => Some(Self::Red),
            "ansigreen" | "ansidarkgreen" => Some(Self::Green),
            "ansiyellow" | "ansibrown" => Some(Self::Yellow),
            "ansiblue" | "ansidarkblue" => Some(Self::Blue),
            "ansimagenta" | "ansipurple" => Some(Self::Magenta),
            "ansicyan" | "ansiteal" => Some(Self::Cyan),
            "ansiwhite" | "ansilightgray" => Some(Self::White),
            "ansibrightblack" | "ansidarkgray" => Some(Self::BrightBlack),
            "ansibrightred" => Some(Self::BrightRed),
            "ansibrightgreen" => Some(Self::BrightGreen),
            "ansibrightyellow" => Some(Self::BrightYellow),
            "ansibrightblue" => Some(Self::BrightBlue),
            "ansibrightmagenta" | "ansifuchsia" => Some(Self::BrightMagenta),
            "ansibrightcyan" | "ansiturquoise" => Some(Self::BrightCyan),
            "ansibrightwhite" => Some(Self::BrightWhite),
            _ => None,
        }
    }

    pub fn closest_from_rgb(r: u8, g: u8, b: u8, exclude: &[Self]) -> Self {
        let (r, g, b) = (i32::from(r), i32::from(g), i32::from(b));

        let saturation = (r - g).abs() + (g - b).abs() + (b - r).abs();
        let mut exclusions = vec![];
        exclusions.extend_from_slice(exclude);

        if saturation > 30 {
            exclusions.push(Self::White);
            exclusions.push(Self::BrightBlack);
            exclusions.push(Self::Black);
        }
        // Find the closest match among non-excluded colors
        let mut distance: i32 = 257 * 257 * 3;
        let mut best_color = Self::Default;

        for color in [
            Self::Red,
            Self::Green,
            Self::Blue,
            Self::Yellow,
            Self::Magenta,
            Self::Cyan,
            Self::White,
            Self::BrightRed,
            Self::BrightGreen,
            Self::BrightBlue,
            Self::BrightYellow,
            Self::BrightMagenta,
            Self::BrightCyan,
            Self::BrightWhite,
            Self::Black,
            Self::BrightBlack,
        ] {
            if exclusions.contains(&color) {
                continue;
            }

            let (r2, g2, b2) = color.rgb();
            let (r2, g2, b2) = (i32::from(r2), i32::from(g2), i32::from(b2));
            let score = (r - r2).pow(2) + (g - g2).pow(2) + (b - b2).pow(2);

            if score < distance {
                distance = score;
                best_color = color;
            }
        }

        best_color
    }
}

impl FromStr for AnsiColor {
    type Err = ColorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_str(s).ok_or_else(|| ColorParseError(s.to_string()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_str_to_ansi_color() {
        // Test valid colors
        assert!(matches!(
            "ansidefault".parse::<AnsiColor>(),
            Ok(AnsiColor::Default)
        ));
        assert!(matches!(
            "ansiblack".parse::<AnsiColor>(),
            Ok(AnsiColor::Black)
        ));
        assert!(matches!("ansired".parse::<AnsiColor>(), Ok(AnsiColor::Red)));
        assert!(matches!(
            "ansiblue".parse::<AnsiColor>(),
            Ok(AnsiColor::Blue)
        ));
        assert!(matches!(
            "ansibrightred".parse::<AnsiColor>(),
            Ok(AnsiColor::BrightRed)
        ));
        assert!(matches!(
            "ansibrightblue".parse::<AnsiColor>(),
            Ok(AnsiColor::BrightBlue)
        ));

        // Test invalid colors
        assert!("".parse::<AnsiColor>().is_err());
        assert!("red".parse::<AnsiColor>().is_err());
        assert!("invalid".parse::<AnsiColor>().is_err());
        assert!("bright_red".parse::<AnsiColor>().is_err());
    }

    #[test]
    fn test_ansi_color_code() {
        assert_eq!(AnsiColor::Default.code(), 39);
        assert_eq!(AnsiColor::Black.code(), 30);
        assert_eq!(AnsiColor::Red.code(), 31);
        assert_eq!(AnsiColor::Green.code(), 32);
        assert_eq!(AnsiColor::Yellow.code(), 33);
        assert_eq!(AnsiColor::Blue.code(), 34);
        assert_eq!(AnsiColor::Magenta.code(), 35);
        assert_eq!(AnsiColor::Cyan.code(), 36);
        assert_eq!(AnsiColor::White.code(), 37);
        assert_eq!(AnsiColor::BrightBlack.code(), 90);
        assert_eq!(AnsiColor::BrightRed.code(), 91);
        assert_eq!(AnsiColor::BrightGreen.code(), 92);
        assert_eq!(AnsiColor::BrightYellow.code(), 93);
        assert_eq!(AnsiColor::BrightBlue.code(), 94);
        assert_eq!(AnsiColor::BrightMagenta.code(), 95);
        assert_eq!(AnsiColor::BrightCyan.code(), 96);
        assert_eq!(AnsiColor::BrightWhite.code(), 97);
    }

    #[test]
    fn test_ansi_color_background_code() {
        assert_eq!(AnsiColor::Default.background_code(), 49);
        assert_eq!(AnsiColor::Black.background_code(), 40);
        assert_eq!(AnsiColor::Red.background_code(), 41);
        assert_eq!(AnsiColor::Green.background_code(), 42);
        assert_eq!(AnsiColor::Yellow.background_code(), 43);
        assert_eq!(AnsiColor::Blue.background_code(), 44);
        assert_eq!(AnsiColor::Magenta.background_code(), 45);
        assert_eq!(AnsiColor::Cyan.background_code(), 46);
        assert_eq!(AnsiColor::White.background_code(), 47);
        assert_eq!(AnsiColor::BrightBlack.background_code(), 100);
        assert_eq!(AnsiColor::BrightRed.background_code(), 101);
        assert_eq!(AnsiColor::BrightGreen.background_code(), 102);
        assert_eq!(AnsiColor::BrightYellow.background_code(), 103);
        assert_eq!(AnsiColor::BrightBlue.background_code(), 104);
        assert_eq!(AnsiColor::BrightMagenta.background_code(), 105);
        assert_eq!(AnsiColor::BrightCyan.background_code(), 106);
        assert_eq!(AnsiColor::BrightWhite.background_code(), 107);
    }

    #[test]
    fn test_ansi_color_rgb() {
        assert_eq!(AnsiColor::Default.rgb(), (0, 0, 0));
        assert_eq!(AnsiColor::Black.rgb(), (0, 0, 0));
        assert_eq!(AnsiColor::Red.rgb(), (205, 0, 0));
        assert_eq!(AnsiColor::Green.rgb(), (0, 205, 0));
        assert_eq!(AnsiColor::Yellow.rgb(), (205, 205, 0));
        assert_eq!(AnsiColor::Blue.rgb(), (0, 0, 238));
        assert_eq!(AnsiColor::Magenta.rgb(), (205, 0, 205));
        assert_eq!(AnsiColor::Cyan.rgb(), (0, 205, 205));
        assert_eq!(AnsiColor::White.rgb(), (229, 229, 229));
        assert_eq!(AnsiColor::BrightBlack.rgb(), (127, 127, 127));
        assert_eq!(AnsiColor::BrightRed.rgb(), (255, 0, 0));
        assert_eq!(AnsiColor::BrightGreen.rgb(), (0, 255, 0));
        assert_eq!(AnsiColor::BrightYellow.rgb(), (255, 255, 0));
        assert_eq!(AnsiColor::BrightBlue.rgb(), (92, 92, 255));
        assert_eq!(AnsiColor::BrightMagenta.rgb(), (255, 0, 255));
        assert_eq!(AnsiColor::BrightCyan.rgb(), (0, 255, 255));
        assert_eq!(AnsiColor::BrightWhite.rgb(), (255, 255, 255));
    }

    #[test]
    fn test_closest_from_rgb() {
        // TODO: Verify these assertions are valid
        // Test exact matches
        assert_eq!(
            AnsiColor::closest_from_rgb(255, 0, 0, &[]),
            AnsiColor::BrightRed
        );
        assert_eq!(
            AnsiColor::closest_from_rgb(0, 255, 0, &[]),
            AnsiColor::BrightGreen
        );
        assert_eq!(AnsiColor::closest_from_rgb(0, 0, 255, &[]), AnsiColor::Blue);
        assert_eq!(AnsiColor::closest_from_rgb(0, 0, 0, &[]), AnsiColor::Black);
        assert_eq!(
            AnsiColor::closest_from_rgb(255, 255, 255, &[]),
            AnsiColor::BrightWhite
        );

        // Test colors with exclusions
        assert_eq!(
            AnsiColor::closest_from_rgb(255, 0, 0, &[AnsiColor::BrightRed]),
            AnsiColor::Red
        );
        assert_eq!(
            AnsiColor::closest_from_rgb(255, 255, 255, &[AnsiColor::BrightWhite]),
            AnsiColor::White
        );

        // Test approximate matches
        assert_eq!(AnsiColor::closest_from_rgb(180, 0, 0, &[]), AnsiColor::Red);
        assert_eq!(
            AnsiColor::closest_from_rgb(100, 100, 100, &[]),
            AnsiColor::BrightBlack
        );
        assert_eq!(
            AnsiColor::closest_from_rgb(200, 200, 0, &[]),
            AnsiColor::Yellow
        );

        // Test colors with multiple exclusions
        assert_eq!(
            AnsiColor::closest_from_rgb(255, 0, 0, &[AnsiColor::BrightRed, AnsiColor::Red]),
            AnsiColor::Yellow
        );
    }

    #[test]
    fn test_str_to_color() {
        // Test valid colors
        assert!(matches!(
            "#aabbcc".parse::<Color>(),
            Ok(Color::Hex(0xaa, 0xbb, 0xcc))
        ));
        assert!(matches!(
            "#ABC".parse::<Color>(),
            Ok(Color::Hex(0xaa, 0xbb, 0xcc))
        ));
        assert!(matches!("red".parse::<Color>(), Ok(Color::Hex(0xff, 0, 0))));
        assert!(matches!(
            "blue".parse::<Color>(),
            Ok(Color::Hex(0, 0, 0xff))
        ));
        assert!(matches!(
            "green".parse::<Color>(),
            Ok(Color::Hex(0, 0x80, 0))
        ));
        assert!(matches!(
            "ansired".parse::<Color>(),
            Ok(Color::Ansi(AnsiColor::Red))
        ));
        assert!(matches!("default".parse::<Color>(), Ok(Color::Default)));
        assert!(matches!("".parse::<Color>(), Ok(Color::Default)));

        // Test invalid colors
        assert!("invalid".parse::<Color>().is_err());
        assert!("#12".parse::<Color>().is_err());
        assert!("#1234567".parse::<Color>().is_err());
        assert!("#ghi".parse::<Color>().is_err());
    }

    #[test]
    fn test_color_rgb() {
        assert_eq!(Color::Default.rgb(), (0, 0, 0));
        assert_eq!(Color::Ansi(AnsiColor::Red).rgb(), (205, 0, 0));
        assert_eq!(Color::Hex(0x12, 0x34, 0x56).rgb(), (0x12, 0x34, 0x56));
    }
}
