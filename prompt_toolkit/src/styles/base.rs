#![expect(dead_code)]

use std::{error::Error, fmt::Display};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum AttrSetting {
    Enabled,
    Disabled,
    #[default]
    Automatic,
}

impl AttrSetting {
    pub fn merge(self, other: Self) -> Self {
        if self == AttrSetting::Automatic {
            other
        } else {
            self
        }
    }

    pub fn is_on(self) -> bool {
        matches!(self, Self::Enabled)
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Attrs {
    pub color: Option<String>,
    pub background_color: Option<String>,
    pub bold: AttrSetting,
    pub underline: AttrSetting,
    pub strike: AttrSetting,
    pub italic: AttrSetting,
    pub blink: AttrSetting,
    pub reverse: AttrSetting,
    pub hidden: AttrSetting,
}

impl Attrs {
    pub fn merge(attrs: &[Self]) -> Self {
        let mut default = Self::default();
        for attr in attrs.iter().rev() {
            if default.color.is_none() {
                default.color.clone_from(&attr.color);
            }
            if default.background_color.is_none() {
                default.background_color.clone_from(&attr.background_color);
            }
            default.bold = default.bold.merge(attr.bold);
            default.underline = default.underline.merge(attr.underline);
            default.strike = default.strike.merge(attr.strike);
            default.italic = default.italic.merge(attr.italic);
            default.blink = default.blink.merge(attr.blink);
            default.reverse = default.reverse.merge(attr.reverse);
            default.hidden = default.hidden.merge(attr.hidden);
        }

        default
    }
}

#[derive(Debug)]
pub struct AnsiColorParseError(String);
impl Error for AnsiColorParseError {}
impl Display for AnsiColorParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No ansi color found for {}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AnsiColor {
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
    pub fn to_code(&self) -> i32 {
        match self {
            AnsiColor::Default => 0,
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
    pub fn to_background_code(&self) -> i32 {
        match self {
            AnsiColor::Default => 0,
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

    pub fn to_rgb(&self) -> (u8, u8, u8) {
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
}

impl TryFrom<&str> for AnsiColor {
    type Error = AnsiColorParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::try_from_str(s).ok_or_else(|| AnsiColorParseError(s.to_string()))
    }
}

pub trait Style {
    fn get_attrs(&self, style_str: &str, default: Attrs) -> Attrs;
    fn style_rules(&self) -> Vec<(String, String)>;
    fn invalidation_hash(&self) -> u64;
}

pub struct DummyStyle;

impl Style for DummyStyle {
    fn get_attrs(&self, _style_str: &str, default: Attrs) -> Attrs {
        default
    }

    fn style_rules(&self) -> Vec<(String, String)> {
        vec![]
    }

    fn invalidation_hash(&self) -> u64 {
        1
    }
}

pub struct DynamicStyle {
    get_style: Box<dyn Fn() -> Option<Box<dyn Style>>>,
}

impl DynamicStyle {
    pub fn new<F>(get_style: F) -> Self
    where
        F: Fn() -> Option<Box<dyn Style>> + 'static,
    {
        Self {
            get_style: Box::new(get_style),
        }
    }

    fn inner_style(&self) -> Box<dyn Style> {
        (self.get_style)().unwrap_or_else(|| Box::new(DummyStyle))
    }
}

impl Style for DynamicStyle {
    fn get_attrs(&self, style_str: &str, default: Attrs) -> Attrs {
        self.inner_style().get_attrs(style_str, default)
    }

    fn style_rules(&self) -> Vec<(String, String)> {
        self.inner_style().style_rules()
    }

    fn invalidation_hash(&self) -> u64 {
        self.inner_style().invalidation_hash()
    }
}

pub struct StandardStyle {
    style_rules: Vec<(String, String)>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dummy_style() {
        let dummy = DummyStyle;
        let default_attrs = Attrs::default();
        let attrs = Attrs::default();

        assert_eq!(dummy.get_attrs("any_style", attrs), default_attrs);
        assert!(dummy.style_rules().is_empty());
        assert_eq!(dummy.invalidation_hash(), 1);
    }

    #[test]
    fn test_dynamic_style() {
        let dynamic = DynamicStyle::new(move || Some(Box::new(DummyStyle)));
        let default_attrs = Attrs::default();
        let attrs = Attrs::default();

        assert_eq!(dynamic.get_attrs("any_style", attrs), default_attrs);
        assert!(dynamic.style_rules().is_empty());
        assert_eq!(dynamic.invalidation_hash(), 1);
    }

    #[test]
    fn test_dynamic_style_none() {
        let dynamic = DynamicStyle::new(|| None);
        let default_attrs = Attrs::default();
        let attrs = Attrs::default();

        assert_eq!(dynamic.get_attrs("any_style", attrs), default_attrs);
        assert!(dynamic.style_rules().is_empty());
        assert_eq!(dynamic.invalidation_hash(), 1);
    }

    #[test]
    fn test_str_to_ansi_color() {
        // Test valid colors
        assert!(matches!(
            AnsiColor::try_from("ansidefault"),
            Ok(AnsiColor::Default)
        ));
        assert!(matches!(
            AnsiColor::try_from("ansiblack"),
            Ok(AnsiColor::Black)
        ));
        assert!(matches!(AnsiColor::try_from("ansired"), Ok(AnsiColor::Red)));
        assert!(matches!(
            AnsiColor::try_from("ansiblue"),
            Ok(AnsiColor::Blue)
        ));
        assert!(matches!(
            AnsiColor::try_from("ansibrightred"),
            Ok(AnsiColor::BrightRed)
        ));
        assert!(matches!(
            AnsiColor::try_from("ansibrightblue"),
            Ok(AnsiColor::BrightBlue)
        ));

        // Test invalid colors
        assert!(AnsiColor::try_from("").is_err());
        assert!(AnsiColor::try_from("red").is_err());
        assert!(AnsiColor::try_from("invalid").is_err());
        assert!(AnsiColor::try_from("bright_red").is_err());
    }

    #[test]
    fn test_attr_setting_merge() {
        // Test with Automatic as first setting
        assert_eq!(
            AttrSetting::Automatic.merge(AttrSetting::Enabled),
            AttrSetting::Enabled
        );
        assert_eq!(
            AttrSetting::Automatic.merge(AttrSetting::Disabled),
            AttrSetting::Disabled
        );
        assert_eq!(
            AttrSetting::Automatic.merge(AttrSetting::Automatic),
            AttrSetting::Automatic
        );

        // Test with non-Automatic as first setting
        assert_eq!(
            AttrSetting::Enabled.merge(AttrSetting::Automatic),
            AttrSetting::Enabled
        );
        assert_eq!(
            AttrSetting::Enabled.merge(AttrSetting::Disabled),
            AttrSetting::Enabled
        );
        assert_eq!(
            AttrSetting::Disabled.merge(AttrSetting::Enabled),
            AttrSetting::Disabled
        );
    }

    #[test]
    fn test_attrs_merge() {
        let attr1 = Attrs {
            color: Some("red".to_string()),
            background_color: None,
            bold: AttrSetting::Enabled,
            underline: AttrSetting::Automatic,
            strike: AttrSetting::Disabled,
            italic: AttrSetting::Automatic,
            blink: AttrSetting::Disabled,
            reverse: AttrSetting::Automatic,
            hidden: AttrSetting::Automatic,
        };

        let attr2 = Attrs {
            color: None,
            background_color: Some("blue".to_string()),
            bold: AttrSetting::Automatic,
            underline: AttrSetting::Enabled,
            strike: AttrSetting::Enabled,
            italic: AttrSetting::Disabled,
            blink: AttrSetting::Automatic,
            reverse: AttrSetting::Automatic,
            hidden: AttrSetting::Automatic,
        };

        let merged = Attrs::merge(&[attr1, attr2]);

        assert_eq!(merged.color, Some("red".to_string()));
        assert_eq!(merged.background_color, Some("blue".to_string()));
        assert_eq!(merged.bold, AttrSetting::Enabled);
        assert_eq!(merged.underline, AttrSetting::Enabled);
        assert_eq!(merged.strike, AttrSetting::Enabled);
        assert_eq!(merged.italic, AttrSetting::Disabled);
        assert_eq!(merged.blink, AttrSetting::Disabled);
        assert_eq!(merged.reverse, AttrSetting::Automatic);
        assert_eq!(merged.hidden, AttrSetting::Automatic);
    }

    #[test]
    fn test_attr_setting_is_on() {
        assert!(AttrSetting::Enabled.is_on());
        assert!(!AttrSetting::Disabled.is_on());
        assert!(!AttrSetting::Automatic.is_on());
    }
}
