use std::{collections::HashMap, sync::LazyLock};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Keys {
    Character(char),
    Escape,
    ShiftEscape,
    ControlAt,
    ControlA,
    ControlB,
    ControlC,
    ControlD,
    ControlE,
    ControlF,
    ControlG,
    ControlH,
    ControlI,
    ControlJ,
    ControlK,
    ControlL,
    ControlM,
    ControlN,
    ControlO,
    ControlP,
    ControlQ,
    ControlR,
    ControlS,
    ControlT,
    ControlU,
    ControlV,
    ControlW,
    ControlX,
    ControlY,
    ControlZ,
    ControlOpenBracket,
    ControlBackslash,
    ControlCloseBracket,
    ControlCircumflex,
    ControlUnderscore,
    Control1,
    Control2,
    Control3,
    Control4,
    Control5,
    Control6,
    Control7,
    Control8,
    Control9,
    Control0,
    ControlShift1,
    ControlShift2,
    ControlShift3,
    ControlShift4,
    ControlShift5,
    ControlShift6,
    ControlShift7,
    ControlShift8,
    ControlShift9,
    ControlShift0,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    Insert,
    Delete,
    PageUp,
    PageDown,
    ControlLeft,
    ControlRight,
    ControlUp,
    ControlDown,
    ControlHome,
    ControlEnd,
    ControlInsert,
    ControlDelete,
    ControlPageUp,
    ControlPageDown,
    ShiftLeft,
    ShiftRight,
    ShiftUp,
    ShiftDown,
    ShiftHome,
    ShiftEnd,
    ShiftInsert,
    ShiftDelete,
    ShiftPageUp,
    ShiftPageDown,
    ControlShiftLeft,
    ControlShiftRight,
    ControlShiftUp,
    ControlShiftDown,
    ControlShiftHome,
    ControlShiftEnd,
    ControlShiftInsert,
    ControlShiftDelete,
    ControlShiftPageUp,
    ControlShiftPageDown,
    BackTab,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    ControlF1,
    ControlF2,
    ControlF3,
    ControlF4,
    ControlF5,
    ControlF6,
    ControlF7,
    ControlF8,
    ControlF9,
    ControlF10,
    ControlF11,
    ControlF12,
    ControlF13,
    ControlF14,
    ControlF15,
    ControlF16,
    ControlF17,
    ControlF18,
    ControlF19,
    ControlF20,
    ControlF21,
    ControlF22,
    ControlF23,
    ControlF24,
    Any,
    ScrollUp,
    ScrollDown,
    CPRResponse,
    Vt100MouseEvent,
    WindowsMouseEvent,
    BracketedPaste,
    SigInt,
    Ignore,
}

impl Keys {
    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Keys::Character(_) => "",
            Keys::Escape => "escape",
            Keys::ShiftEscape => "s-escape",
            Keys::ControlAt => "c-@",
            Keys::ControlA => "c-a",
            Keys::ControlB => "c-b",
            Keys::ControlC => "c-c",
            Keys::ControlD => "c-d",
            Keys::ControlE => "c-e",
            Keys::ControlF => "c-f",
            Keys::ControlG => "c-g",
            Keys::ControlH => "c-h",
            Keys::ControlI => "c-i",
            Keys::ControlJ => "c-j",
            Keys::ControlK => "c-k",
            Keys::ControlL => "c-l",
            Keys::ControlM => "c-m",
            Keys::ControlN => "c-n",
            Keys::ControlO => "c-o",
            Keys::ControlP => "c-p",
            Keys::ControlQ => "c-q",
            Keys::ControlR => "c-r",
            Keys::ControlS => "c-s",
            Keys::ControlT => "c-t",
            Keys::ControlU => "c-u",
            Keys::ControlV => "c-v",
            Keys::ControlW => "c-w",
            Keys::ControlX => "c-x",
            Keys::ControlY => "c-y",
            Keys::ControlZ => "c-z",
            Keys::ControlOpenBracket => "c-[",
            Keys::ControlBackslash => "c-\\",
            Keys::ControlCloseBracket => "c-]",
            Keys::ControlCircumflex => "c-^",
            Keys::ControlUnderscore => "c-_",
            Keys::Control0 => "c-0",
            Keys::Control1 => "c-1",
            Keys::Control2 => "c-2",
            Keys::Control3 => "c-3",
            Keys::Control4 => "c-4",
            Keys::Control5 => "c-5",
            Keys::Control6 => "c-6",
            Keys::Control7 => "c-7",
            Keys::Control8 => "c-8",
            Keys::Control9 => "c-9",
            Keys::ControlShift0 => "c-s-0",
            Keys::ControlShift1 => "c-s-1",
            Keys::ControlShift2 => "c-s-2",
            Keys::ControlShift3 => "c-s-3",
            Keys::ControlShift4 => "c-s-4",
            Keys::ControlShift5 => "c-s-5",
            Keys::ControlShift6 => "c-s-6",
            Keys::ControlShift7 => "c-s-7",
            Keys::ControlShift8 => "c-s-8",
            Keys::ControlShift9 => "c-s-9",
            Keys::Left => "left",
            Keys::Right => "right",
            Keys::Up => "up",
            Keys::Down => "down",
            Keys::Home => "home",
            Keys::End => "end",
            Keys::Insert => "insert",
            Keys::Delete => "delete",
            Keys::PageUp => "page-up",
            Keys::PageDown => "page-down",
            Keys::ControlLeft => "c-left",
            Keys::ControlRight => "c-right",
            Keys::ControlUp => "c-up",
            Keys::ControlDown => "c-down",
            Keys::ControlHome => "c-home",
            Keys::ControlEnd => "c-end",
            Keys::ControlInsert => "c-insert",
            Keys::ControlDelete => "c-delete",
            Keys::ControlPageUp => "c-page-up",
            Keys::ControlPageDown => "c-page-down",
            Keys::ShiftLeft => "s-left",
            Keys::ShiftRight => "s-right",
            Keys::ShiftUp => "s-up",
            Keys::ShiftDown => "s-down",
            Keys::ShiftHome => "s-home",
            Keys::ShiftEnd => "s-end",
            Keys::ShiftInsert => "s-insert",
            Keys::ShiftDelete => "s-delete",
            Keys::ShiftPageUp => "s-page-up",
            Keys::ShiftPageDown => "s-page-down",
            Keys::ControlShiftLeft => "c-s-left",
            Keys::ControlShiftRight => "c-s-right",
            Keys::ControlShiftUp => "c-s-up",
            Keys::ControlShiftDown => "c-s-down",
            Keys::ControlShiftHome => "c-s-home",
            Keys::ControlShiftEnd => "c-s-end",
            Keys::ControlShiftInsert => "c-s-insert",
            Keys::ControlShiftDelete => "c-s-delete",
            Keys::ControlShiftPageUp => "c-s-page-up",
            Keys::ControlShiftPageDown => "c-s-page-down",
            Keys::BackTab => "s-tab",
            Keys::F1 => "f1",
            Keys::F2 => "f2",
            Keys::F3 => "f3",
            Keys::F4 => "f4",
            Keys::F5 => "f5",
            Keys::F6 => "f6",
            Keys::F7 => "f7",
            Keys::F8 => "f8",
            Keys::F9 => "f9",
            Keys::F10 => "f10",
            Keys::F11 => "f11",
            Keys::F12 => "f12",
            Keys::F13 => "f13",
            Keys::F14 => "f14",
            Keys::F15 => "f15",
            Keys::F16 => "f16",
            Keys::F17 => "f17",
            Keys::F18 => "f18",
            Keys::F19 => "f19",
            Keys::F20 => "f20",
            Keys::F21 => "f21",
            Keys::F22 => "f22",
            Keys::F23 => "f23",
            Keys::F24 => "f24",
            Keys::ControlF1 => "c-f1",
            Keys::ControlF2 => "c-f2",
            Keys::ControlF3 => "c-f3",
            Keys::ControlF4 => "c-f4",
            Keys::ControlF5 => "c-f5",
            Keys::ControlF6 => "c-f6",
            Keys::ControlF7 => "c-f7",
            Keys::ControlF8 => "c-f8",
            Keys::ControlF9 => "c-f9",
            Keys::ControlF10 => "c-f10",
            Keys::ControlF11 => "c-f11",
            Keys::ControlF12 => "c-f12",
            Keys::ControlF13 => "c-f13",
            Keys::ControlF14 => "c-f14",
            Keys::ControlF15 => "c-f15",
            Keys::ControlF16 => "c-f16",
            Keys::ControlF17 => "c-f17",
            Keys::ControlF18 => "c-f18",
            Keys::ControlF19 => "c-f19",
            Keys::ControlF20 => "c-f20",
            Keys::ControlF21 => "c-f21",
            Keys::ControlF22 => "c-f22",
            Keys::ControlF23 => "c-f23",
            Keys::ControlF24 => "c-f24",
            Keys::Any => "<any>",
            Keys::ScrollUp => "<scroll-up>",
            Keys::ScrollDown => "<scroll-down>",
            Keys::CPRResponse => "<cpr-response>",
            Keys::Vt100MouseEvent => "<vt100-mouse-event>",
            Keys::WindowsMouseEvent => "<windows-mouse-event>",
            Keys::BracketedPaste => "<bracketed-paste>",
            Keys::SigInt => "<sigint>",
            Keys::Ignore => "<ignore>",
        }
    }
}

pub static ALL_KEYS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    // Yikes
    let v: Vec<&'static str> = vec![
        Keys::Escape.as_str(),
        Keys::ShiftEscape.as_str(),
        Keys::ControlAt.as_str(),
        Keys::ControlA.as_str(),
        Keys::ControlB.as_str(),
        Keys::ControlC.as_str(),
        Keys::ControlD.as_str(),
        Keys::ControlE.as_str(),
        Keys::ControlF.as_str(),
        Keys::ControlG.as_str(),
        Keys::ControlH.as_str(),
        Keys::ControlI.as_str(),
        Keys::ControlJ.as_str(),
        Keys::ControlK.as_str(),
        Keys::ControlL.as_str(),
        Keys::ControlM.as_str(),
        Keys::ControlN.as_str(),
        Keys::ControlO.as_str(),
        Keys::ControlP.as_str(),
        Keys::ControlQ.as_str(),
        Keys::ControlR.as_str(),
        Keys::ControlS.as_str(),
        Keys::ControlT.as_str(),
        Keys::ControlU.as_str(),
        Keys::ControlV.as_str(),
        Keys::ControlW.as_str(),
        Keys::ControlX.as_str(),
        Keys::ControlY.as_str(),
        Keys::ControlZ.as_str(),
        Keys::ControlOpenBracket.as_str(),
        Keys::ControlBackslash.as_str(),
        Keys::ControlCloseBracket.as_str(),
        Keys::ControlCircumflex.as_str(),
        Keys::ControlUnderscore.as_str(),
        Keys::Control0.as_str(),
        Keys::Control1.as_str(),
        Keys::Control2.as_str(),
        Keys::Control3.as_str(),
        Keys::Control4.as_str(),
        Keys::Control5.as_str(),
        Keys::Control6.as_str(),
        Keys::Control7.as_str(),
        Keys::Control8.as_str(),
        Keys::Control9.as_str(),
        Keys::ControlShift0.as_str(),
        Keys::ControlShift1.as_str(),
        Keys::ControlShift2.as_str(),
        Keys::ControlShift3.as_str(),
        Keys::ControlShift4.as_str(),
        Keys::ControlShift5.as_str(),
        Keys::ControlShift6.as_str(),
        Keys::ControlShift7.as_str(),
        Keys::ControlShift8.as_str(),
        Keys::ControlShift9.as_str(),
        Keys::Left.as_str(),
        Keys::Right.as_str(),
        Keys::Up.as_str(),
        Keys::Down.as_str(),
        Keys::Home.as_str(),
        Keys::End.as_str(),
        Keys::Insert.as_str(),
        Keys::Delete.as_str(),
        Keys::PageUp.as_str(),
        Keys::PageDown.as_str(),
        Keys::ControlLeft.as_str(),
        Keys::ControlRight.as_str(),
        Keys::ControlUp.as_str(),
        Keys::ControlDown.as_str(),
        Keys::ControlHome.as_str(),
        Keys::ControlEnd.as_str(),
        Keys::ControlInsert.as_str(),
        Keys::ControlDelete.as_str(),
        Keys::ControlPageUp.as_str(),
        Keys::ControlPageDown.as_str(),
        Keys::ShiftLeft.as_str(),
        Keys::ShiftRight.as_str(),
        Keys::ShiftUp.as_str(),
        Keys::ShiftDown.as_str(),
        Keys::ShiftHome.as_str(),
        Keys::ShiftEnd.as_str(),
        Keys::ShiftInsert.as_str(),
        Keys::ShiftDelete.as_str(),
        Keys::ShiftPageUp.as_str(),
        Keys::ShiftPageDown.as_str(),
        Keys::ControlShiftLeft.as_str(),
        Keys::ControlShiftRight.as_str(),
        Keys::ControlShiftUp.as_str(),
        Keys::ControlShiftDown.as_str(),
        Keys::ControlShiftHome.as_str(),
        Keys::ControlShiftEnd.as_str(),
        Keys::ControlShiftInsert.as_str(),
        Keys::ControlShiftDelete.as_str(),
        Keys::ControlShiftPageUp.as_str(),
        Keys::ControlShiftPageDown.as_str(),
        Keys::BackTab.as_str(),
        Keys::F1.as_str(),
        Keys::F2.as_str(),
        Keys::F3.as_str(),
        Keys::F4.as_str(),
        Keys::F5.as_str(),
        Keys::F6.as_str(),
        Keys::F7.as_str(),
        Keys::F8.as_str(),
        Keys::F9.as_str(),
        Keys::F10.as_str(),
        Keys::F11.as_str(),
        Keys::F12.as_str(),
        Keys::F13.as_str(),
        Keys::F14.as_str(),
        Keys::F15.as_str(),
        Keys::F16.as_str(),
        Keys::F17.as_str(),
        Keys::F18.as_str(),
        Keys::F19.as_str(),
        Keys::F20.as_str(),
        Keys::F21.as_str(),
        Keys::F22.as_str(),
        Keys::F23.as_str(),
        Keys::F24.as_str(),
        Keys::ControlF1.as_str(),
        Keys::ControlF2.as_str(),
        Keys::ControlF3.as_str(),
        Keys::ControlF4.as_str(),
        Keys::ControlF5.as_str(),
        Keys::ControlF6.as_str(),
        Keys::ControlF7.as_str(),
        Keys::ControlF8.as_str(),
        Keys::ControlF9.as_str(),
        Keys::ControlF10.as_str(),
        Keys::ControlF11.as_str(),
        Keys::ControlF12.as_str(),
        Keys::ControlF13.as_str(),
        Keys::ControlF14.as_str(),
        Keys::ControlF15.as_str(),
        Keys::ControlF16.as_str(),
        Keys::ControlF17.as_str(),
        Keys::ControlF18.as_str(),
        Keys::ControlF19.as_str(),
        Keys::ControlF20.as_str(),
        Keys::ControlF21.as_str(),
        Keys::ControlF22.as_str(),
        Keys::ControlF23.as_str(),
        Keys::ControlF24.as_str(),
        Keys::Any.as_str(),
        Keys::ScrollUp.as_str(),
        Keys::ScrollDown.as_str(),
        Keys::CPRResponse.as_str(),
        Keys::Vt100MouseEvent.as_str(),
        Keys::WindowsMouseEvent.as_str(),
        Keys::BracketedPaste.as_str(),
        Keys::SigInt.as_str(),
        Keys::Ignore.as_str(),
    ];
    v
});

pub static KEY_ALIASES: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("backspace", "c-h");
    m.insert("c-space", "c-@");
    m.insert("enter", "c-m");
    m.insert("tab", "c-i");
    m.insert("s-c-left", "c-s-left");
    m.insert("s-c-right", "c-s-right");
    m.insert("s-c-home", "c-s-home");
    m.insert("s-c-end", "c-s-end");
    m
});
