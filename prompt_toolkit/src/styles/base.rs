#![expect(dead_code)]

use crate::styles::{AnsiColor, Color};
use anyhow::Result;
use regex::Regex;
use std::{collections::HashSet, default, error::Error, fmt::Display, sync::LazyLock};
use tracing::warn;

static CLASS_NAMES_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-z0-9.\s_-]*$").expect("valid regex"));

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Attrs {
    pub color: Option<Color>,
    pub background_color: Option<Color>,
    pub bold: AttrSetting,
    pub underline: AttrSetting,
    pub strike: AttrSetting,
    pub italic: AttrSetting,
    pub blink: AttrSetting,
    pub reverse: AttrSetting,
    pub hidden: AttrSetting,
}

impl Default for Attrs {
    fn default() -> Self {
        Self {
            color: Some(Color::default()),
            background_color: Some(Color::default()),
            bold: AttrSetting::Disabled,
            underline: AttrSetting::Disabled,
            strike: AttrSetting::Disabled,
            italic: AttrSetting::Disabled,
            blink: AttrSetting::Disabled,
            reverse: AttrSetting::Disabled,
            hidden: AttrSetting::Disabled,
        }
    }
}

impl Attrs {
    pub fn merge(attrs: &[Self]) -> Self {
        let mut empty = Self::empty();
        let default_attr = Self::default();
        let mut attrs_to_merge = vec![&default_attr];
        attrs_to_merge.extend(attrs);
        for attr in attrs_to_merge.iter().rev() {
            if empty.color.is_none() {
                empty.color.clone_from(&attr.color);
            }
            if empty.background_color.is_none() {
                empty.background_color.clone_from(&attr.background_color);
            }
            empty.bold = empty.bold.merge(attr.bold);
            empty.underline = empty.underline.merge(attr.underline);
            empty.strike = empty.strike.merge(attr.strike);
            empty.italic = empty.italic.merge(attr.italic);
            empty.blink = empty.blink.merge(attr.blink);
            empty.reverse = empty.reverse.merge(attr.reverse);
            empty.hidden = empty.hidden.merge(attr.hidden);
        }

        empty
    }

    pub fn empty() -> Self {
        Self {
            color: None,
            background_color: None,
            bold: AttrSetting::Automatic,
            underline: AttrSetting::Automatic,
            strike: AttrSetting::Automatic,
            italic: AttrSetting::Automatic,
            blink: AttrSetting::Automatic,
            reverse: AttrSetting::Automatic,
            hidden: AttrSetting::Automatic,
        }
    }

    pub fn from_style_string(style: &str) -> Self {
        let mut attr = if style.contains("noinherit") {
            Attrs::default()
        } else {
            Attrs::empty()
        };

        for part in style.split_whitespace() {
            match part {
                "bold" => attr.bold = AttrSetting::Enabled,
                "nobold" => attr.bold = AttrSetting::Disabled,
                "italic" => attr.italic = AttrSetting::Enabled,
                "noitalic" => attr.italic = AttrSetting::Disabled,
                "underline" => attr.underline = AttrSetting::Enabled,
                "nounderline" => attr.underline = AttrSetting::Disabled,
                "strike" => attr.strike = AttrSetting::Enabled,
                "nostrike" => attr.strike = AttrSetting::Disabled,
                "blink" => attr.blink = AttrSetting::Enabled,
                "noblink" => attr.blink = AttrSetting::Disabled,
                "reverse" => attr.reverse = AttrSetting::Enabled,
                "noreverse" => attr.reverse = AttrSetting::Disabled,
                "hidden" => attr.hidden = AttrSetting::Enabled,
                "nohidden" => attr.hidden = AttrSetting::Disabled,
                background_color if background_color.starts_with("bg:") => {
                    attr.background_color = background_color[3..].parse().ok();
                }
                foreground_color if foreground_color.starts_with("fg:") => {
                    attr.color = foreground_color[3..].parse().ok();
                }
                _ => {} // TODO: optional "fg:" prefix
            }
        }

        attr
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

#[derive(Debug)]
pub struct StandardStyle {
    style_rules: Vec<(String, String)>,
    class_names_and_attrs: Vec<(HashSet<String>, Attrs)>,
}

impl StandardStyle {
    pub fn new(style_rules: Vec<(String, String)>) -> Result<Self> {
        let mut class_names_and_attrs: Vec<(HashSet<String>, Attrs)> = Vec::new();
        for (class_names, style_string) in style_rules.clone() {
            if CLASS_NAMES_REGEX.is_match(&class_names) {
                let class_names_set: HashSet<String> = class_names
                    .to_lowercase()
                    .split_whitespace()
                    .map(std::string::ToString::to_string)
                    .collect();
                let attrs = Attrs::from_style_string(&style_string);
                class_names_and_attrs.push((class_names_set, attrs));
            } else {
                return Err(anyhow::anyhow!("Invalid class name: {}", class_names));
            }
        }
        Ok(Self {
            style_rules,
            class_names_and_attrs,
        })
    }
}

impl Style for StandardStyle {
    fn get_attrs(&self, style_str: &str, default: Attrs) -> Attrs {
        let mut attrs_vec = vec![default];
        for (class_names, attrs) in &self.class_names_and_attrs {
            if !class_names.is_empty() {
                attrs_vec.push(*attrs);
            }
        }

        for part in style_str.split_whitespace() {
            if let Some(stripped_part) = part.strip_prefix("class:") {
                warn!("classes not supported yet");
                let part_without_prefix = &stripped_part.to_lowercase();
                let _new_class_names: Vec<String> = part_without_prefix
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect();
                // TODO: Combinations....
            } else {
                let inline_attrs = Attrs::from_style_string(part);
                attrs_vec.push(inline_attrs);
            }
        }

        Attrs::merge(&attrs_vec)
    }

    fn style_rules(&self) -> Vec<(String, String)> {
        self.style_rules.clone()
    }

    fn invalidation_hash(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        for (class_names, attrs) in &self.class_names_and_attrs {
            for class_name in class_names {
                class_name.hash(&mut hasher);
            }
            let attrs_bytes = format!("{attrs:?}").into_bytes();
            for byte in attrs_bytes {
                byte.hash(&mut hasher);
            }
        }
        hasher.finish()
    }
}

pub struct MergedStyle {
    styles: Vec<Box<dyn Style>>,
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
            color: Some(Color::Ansi(AnsiColor::Red)),
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
            background_color: Some(Color::Ansi(AnsiColor::Blue)),
            bold: AttrSetting::Automatic,
            underline: AttrSetting::Enabled,
            strike: AttrSetting::Enabled,
            italic: AttrSetting::Disabled,
            blink: AttrSetting::Automatic,
            reverse: AttrSetting::Automatic,
            hidden: AttrSetting::Automatic,
        };

        let merged = Attrs::merge(&[attr1, attr2]);

        assert_eq!(merged.color, Some(Color::Ansi(AnsiColor::Red)));
        assert_eq!(merged.background_color, Some(Color::Ansi(AnsiColor::Blue)));
        assert_eq!(merged.bold, AttrSetting::Enabled);
        assert_eq!(merged.underline, AttrSetting::Enabled);
        assert_eq!(merged.strike, AttrSetting::Enabled);
        assert_eq!(merged.italic, AttrSetting::Disabled);
        assert_eq!(merged.blink, AttrSetting::Disabled);
        assert_eq!(merged.reverse, AttrSetting::Disabled);
        assert_eq!(merged.hidden, AttrSetting::Disabled);
    }

    #[test]
    fn test_attr_setting_is_on() {
        assert!(AttrSetting::Enabled.is_on());
        assert!(!AttrSetting::Disabled.is_on());
        assert!(!AttrSetting::Automatic.is_on());
    }
}
