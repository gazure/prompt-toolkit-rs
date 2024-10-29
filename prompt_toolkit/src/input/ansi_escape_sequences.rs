use std::{collections::HashMap, sync::LazyLock};

use crate::keys::Keys;

pub static ANSI_SEQUENCES: LazyLock<HashMap<&'static str, Vec<Keys>>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert("\x00", vec![Keys::ControlAt]); // Control-At (Also for Ctrl-Space)
    map.insert("\x01", vec![Keys::ControlA]); // Control-A (home)
    map.insert("\x02", vec![Keys::ControlB]); // Control-B (emacs cursor left)
    map.insert("\x03", vec![Keys::ControlC]); // Control-C (interrupt)
    map.insert("\x04", vec![Keys::ControlD]); // Control-D (exit)
    map.insert("\x05", vec![Keys::ControlE]); // Control-E (end)
    map.insert("\x06", vec![Keys::ControlF]); // Control-F (cursor forward)
    map.insert("\x07", vec![Keys::ControlG]); // Control-G
    map.insert("\x08", vec![Keys::ControlH]); // Control-H (8) (Identical to '\b')
    map.insert("\x09", vec![Keys::ControlI]); // Control-I (9) (Identical to '\t')
    map.insert("\x0a", vec![Keys::ControlJ]); // Control-J (10) (Identical to '\n')
    map.insert("\x0b", vec![Keys::ControlK]); // Control-K (delete until end of line; vertical tab)
    map.insert("\x0c", vec![Keys::ControlL]); // Control-L (clear; form feed)
    map.insert("\x0d", vec![Keys::ControlM]); // Control-M (13) (Identical to '\r')
    map.insert("\x0e", vec![Keys::ControlN]); // Control-N (14) (history forward)
    map.insert("\x0f", vec![Keys::ControlO]); // Control-O (15)
    map.insert("\x10", vec![Keys::ControlP]); // Control-P (16) (history back)
    map.insert("\x11", vec![Keys::ControlQ]); // Control-Q
    map.insert("\x12", vec![Keys::ControlR]); // Control-R (18) (reverse search)
    map.insert("\x13", vec![Keys::ControlS]); // Control-S (19) (forward search)
    map.insert("\x14", vec![Keys::ControlT]); // Control-T
    map.insert("\x15", vec![Keys::ControlU]); // Control-U
    map.insert("\x16", vec![Keys::ControlV]); // Control-V
    map.insert("\x17", vec![Keys::ControlW]); // Control-W
    map.insert("\x18", vec![Keys::ControlX]); // Control-X
    map.insert("\x19", vec![Keys::ControlY]); // Control-Y (25)
    map.insert("\x1a", vec![Keys::ControlZ]); // Control-Z
    map.insert("\x1b", vec![Keys::Escape]); // Also Control-[
    map.insert("\x1b\x5b", vec![Keys::ShiftEscape]); // Using ESC [ instead of 0x9b
    map.insert("\x1c", vec![Keys::ControlBackslash]); // Both Control-\ (also Ctrl-| )
    map.insert("\x1d", vec![Keys::ControlCloseBracket]); // Control-]
    map.insert("\x1e", vec![Keys::ControlCircumflex]); // Control-^
    map.insert("\x1f", vec![Keys::ControlUnderscore]); // Control-underscore (Also for Ctrl-hyphen.)
                                                       // ASCII Delete (0x7f)
                                                       // Vt220 (and Linux terminal) send this when pressing backspace. We map this
                                                       // to ControlH, because that will make it easier to create key bindings that
                                                       // work everywhere, with the trade-off that it's no longer possible to
                                                       // handle backspace and control-h individually for the few terminals that
                                                       // support it. (Most terminals send ControlH when backspace is pressed.)
                                                       // See: http://www.ibb.net/~anne/keyboard.html
    map.insert("\x7f", vec![Keys::ControlH]);
    // --
    // Various
    map.insert("\x1b[1~", vec![Keys::Home]); // tmux
    map.insert("\x1b[2~", vec![Keys::Insert]);
    map.insert("\x1b[3~", vec![Keys::Delete]);
    map.insert("\x1b[4~", vec![Keys::End]); // tmux
    map.insert("\x1b[5~", vec![Keys::PageUp]);
    map.insert("\x1b[6~", vec![Keys::PageDown]);
    map.insert("\x1b[7~", vec![Keys::Home]); // xrvt
    map.insert("\x1b[8~", vec![Keys::End]); // xrvt
    map.insert("\x1b[Z", vec![Keys::BackTab]); // shift + tab
    map.insert("\x1b\x09", vec![Keys::BackTab]); // Linux console
    map.insert("\x1b[~", vec![Keys::BackTab]); // Windows console
                                               // --
                                               // Function keys.
    map.insert("\x1bOP", vec![Keys::F1]);
    map.insert("\x1bOQ", vec![Keys::F2]);
    map.insert("\x1bOR", vec![Keys::F3]);
    map.insert("\x1bOS", vec![Keys::F4]);
    map.insert("\x1b[[A", vec![Keys::F1]); // Linux console.
    map.insert("\x1b[[B", vec![Keys::F2]); // Linux console.
    map.insert("\x1b[[C", vec![Keys::F3]); // Linux console.
    map.insert("\x1b[[D", vec![Keys::F4]); // Linux console.
    map.insert("\x1b[[E", vec![Keys::F5]); // Linux console.
    map.insert("\x1b[11~", vec![Keys::F1]); // rxvt-unicode
    map.insert("\x1b[12~", vec![Keys::F2]); // rxvt-unicode
    map.insert("\x1b[13~", vec![Keys::F3]); // rxvt-unicode
    map.insert("\x1b[14~", vec![Keys::F4]); // rxvt-unicode
    map.insert("\x1b[15~", vec![Keys::F5]);
    map.insert("\x1b[17~", vec![Keys::F6]);
    map.insert("\x1b[18~", vec![Keys::F7]);
    map.insert("\x1b[19~", vec![Keys::F8]);
    map.insert("\x1b[20~", vec![Keys::F9]);
    map.insert("\x1b[21~", vec![Keys::F10]);
    map.insert("\x1b[23~", vec![Keys::F11]);
    map.insert("\x1b[24~", vec![Keys::F12]);
    map.insert("\x1b[25~", vec![Keys::F13]);
    map.insert("\x1b[26~", vec![Keys::F14]);
    map.insert("\x1b[28~", vec![Keys::F15]);
    map.insert("\x1b[29~", vec![Keys::F16]);
    map.insert("\x1b[31~", vec![Keys::F17]);
    map.insert("\x1b[32~", vec![Keys::F18]);
    map.insert("\x1b[33~", vec![Keys::F19]);
    map.insert("\x1b[34~", vec![Keys::F20]);
    // Xterm
    map.insert("\x1b[1;2P", vec![Keys::F13]);
    map.insert("\x1b[1;2Q", vec![Keys::F14]);
    // "\x1b[1;2R": Keys.F15,  # Conflicts with CPR response.
    map.insert("\x1b[1;2S", vec![Keys::F16]);
    map.insert("\x1b[15;2~", vec![Keys::F17]);
    map.insert("\x1b[17;2~", vec![Keys::F18]);
    map.insert("\x1b[18;2~", vec![Keys::F19]);
    map.insert("\x1b[19;2~", vec![Keys::F20]);
    map.insert("\x1b[20;2~", vec![Keys::F21]);
    map.insert("\x1b[21;2~", vec![Keys::F22]);
    map.insert("\x1b[23;2~", vec![Keys::F23]);
    map.insert("\x1b[24;2~", vec![Keys::F24]);
    // --
    // CSI 27 disambiguated modified "other" keys (xterm)
    // Ref: https://invisible-island.net/xterm/modified-keys.html
    // These are currently unsupported, so just re-map some common ones to the
    // unmodified versions
    map.insert("\x1b[27;2;13~", vec![Keys::ControlM]); // Shift + Enter
    map.insert("\x1b[27;5;13~", vec![Keys::ControlM]); // Ctrl + Enter
    map.insert("\x1b[27;6;13~", vec![Keys::ControlM]); // Ctrl + Shift + Enter
                                                       // --
                                                       // Control + function keys.
    map.insert("\x1b[1;5P", vec![Keys::ControlF1]);
    map.insert("\x1b[1;5Q", vec![Keys::ControlF2]);
    // "\x1b[1;5R": Keys.ControlF3,  # Conflicts with CPR response.
    map.insert("\x1b[1;5S", vec![Keys::ControlF4]);
    map.insert("\x1b[15;5~", vec![Keys::ControlF5]);
    map.insert("\x1b[17;5~", vec![Keys::ControlF6]);
    map.insert("\x1b[18;5~", vec![Keys::ControlF7]);
    map.insert("\x1b[19;5~", vec![Keys::ControlF8]);
    map.insert("\x1b[20;5~", vec![Keys::ControlF9]);
    map.insert("\x1b[21;5~", vec![Keys::ControlF10]);
    map.insert("\x1b[23;5~", vec![Keys::ControlF11]);
    map.insert("\x1b[24;5~", vec![Keys::ControlF12]);
    map.insert("\x1b[1;6P", vec![Keys::ControlF13]);
    map.insert("\x1b[1;6Q", vec![Keys::ControlF14]);
    // "\x1b[1;6R": Keys.ControlF15,  # Conflicts with CPR response.
    map.insert("\x1b[1;6S", vec![Keys::ControlF16]);
    map.insert("\x1b[15;6~", vec![Keys::ControlF17]);
    map.insert("\x1b[17;6~", vec![Keys::ControlF18]);
    map.insert("\x1b[18;6~", vec![Keys::ControlF19]);
    map.insert("\x1b[19;6~", vec![Keys::ControlF20]);
    map.insert("\x1b[20;6~", vec![Keys::ControlF21]);
    map.insert("\x1b[21;6~", vec![Keys::ControlF22]);
    map.insert("\x1b[23;6~", vec![Keys::ControlF23]);
    map.insert("\x1b[24;6~", vec![Keys::ControlF24]);
    // --
    // Tmux (Win32 subsystem) sends the following scroll events.
    map.insert("\x1b[62~", vec![Keys::ScrollUp]);
    map.insert("\x1b[63~", vec![Keys::ScrollDown]);
    map.insert("\x1b[200~", vec![Keys::BracketedPaste]); // Start of bracketed paste.
                                                         // --
                                                         // Sequences generated by numpad 5. Not sure what it means. (It doesn't
                                                         // appear in 'infocmp'. Just ignore.
    map.insert("\x1b[E", vec![Keys::Ignore]); // Xterm.
    map.insert("\x1b[G", vec![Keys::Ignore]); // Linux console.
                                              // --
                                              // Meta/control/escape + pageup/pagedown/insert/delete.
    map.insert("\x1b[3;2~", vec![Keys::ShiftDelete]); // xterm, gnome-terminal.
    map.insert("\x1b[5;2~", vec![Keys::ShiftPageUp]);
    map.insert("\x1b[6;2~", vec![Keys::ShiftPageDown]);
    map.insert("\x1b[2;3~", vec![Keys::Escape, Keys::Insert]);
    map.insert("\x1b[3;3~", vec![Keys::Escape, Keys::Delete]);
    map.insert("\x1b[5;3~", vec![Keys::Escape, Keys::PageUp]);
    map.insert("\x1b[6;3~", vec![Keys::Escape, Keys::PageDown]);
    map.insert("\x1b[2;4~", vec![Keys::Escape, Keys::ShiftInsert]);
    map.insert("\x1b[3;4~", vec![Keys::Escape, Keys::ShiftDelete]);
    map.insert("\x1b[5;4~", vec![Keys::Escape, Keys::ShiftPageUp]);
    map.insert("\x1b[6;4~", vec![Keys::Escape, Keys::ShiftPageDown]);
    map.insert("\x1b[3;5~", vec![Keys::ControlDelete]); // xterm, gnome-terminal.
    map.insert("\x1b[5;5~", vec![Keys::ControlPageUp]);
    map.insert("\x1b[6;5~", vec![Keys::ControlPageDown]);
    map.insert("\x1b[3;6~", vec![Keys::ControlShiftDelete]);
    map.insert("\x1b[5;6~", vec![Keys::ControlShiftPageUp]);
    map.insert("\x1b[6;6~", vec![Keys::ControlShiftPageDown]);
    map.insert("\x1b[2;7~", vec![Keys::Escape, Keys::ControlInsert]);
    map.insert("\x1b[5;7~", vec![Keys::Escape, Keys::ControlPageDown]);
    map.insert("\x1b[6;7~", vec![Keys::Escape, Keys::ControlPageDown]);
    map.insert("\x1b[2;8~", vec![Keys::Escape, Keys::ControlShiftInsert]);
    map.insert("\x1b[5;8~", vec![Keys::Escape, Keys::ControlShiftPageDown]);
    map.insert("\x1b[6;8~", vec![Keys::Escape, Keys::ControlShiftPageDown]);
    // --
    // Arrows.
    // (Normal cursor mode).
    map.insert("\x1b[A", vec![Keys::Up]);
    map.insert("\x1b[B", vec![Keys::Down]);
    map.insert("\x1b[C", vec![Keys::Right]);
    map.insert("\x1b[D", vec![Keys::Left]);
    map.insert("\x1b[H", vec![Keys::Home]);
    map.insert("\x1b[F", vec![Keys::End]);
    // Tmux sends following keystrokes when control+arrow is pressed, but for
    // Emacs ansi-term sends the same sequences for normal arrow keys. Consider
    // it a normal arrow press, because that's more important.
    // (Application cursor mode).
    map.insert("\x1bOA", vec![Keys::Up]);
    map.insert("\x1bOB", vec![Keys::Down]);
    map.insert("\x1bOC", vec![Keys::Right]);
    map.insert("\x1bOD", vec![Keys::Left]);
    map.insert("\x1bOF", vec![Keys::End]);
    map.insert("\x1bOH", vec![Keys::Home]);
    // Shift + arrows.
    map.insert("\x1b[1;2A", vec![Keys::ShiftUp]);
    map.insert("\x1b[1;2B", vec![Keys::ShiftDown]);
    map.insert("\x1b[1;2C", vec![Keys::ShiftRight]);
    map.insert("\x1b[1;2D", vec![Keys::ShiftLeft]);
    map.insert("\x1b[1;2F", vec![Keys::ShiftEnd]);
    map.insert("\x1b[1;2H", vec![Keys::ShiftHome]);
    // Meta + arrow keys. Several terminals handle this differently.
    // The following sequences are for xterm and gnome-terminal.
    //     (Iterm sends ESC followed by the normal arrow_up/down/left/right
    //     sequences, and the OSX Terminal sends ESCb and ESCf for "alt
    //     arrow_left" and "alt arrow_right." We don't handle these
    //     explicitly, in here, because would could not distinguish between
    //     pressing ESC (to go to Vi navigation mode), followed by just the
    //     'b' or 'f' key. These combinations are handled in
    //     the input processor.)
    map.insert("\x1b[1;3A", vec![Keys::Escape, Keys::Up]);
    map.insert("\x1b[1;3B", vec![Keys::Escape, Keys::Down]);
    map.insert("\x1b[1;3C", vec![Keys::Escape, Keys::Right]);
    map.insert("\x1b[1;3D", vec![Keys::Escape, Keys::Left]);
    map.insert("\x1b[1;3F", vec![Keys::Escape, Keys::End]);
    map.insert("\x1b[1;3H", vec![Keys::Escape, Keys::Home]);
    // Alt+shift+number.
    map.insert("\x1b[1;4A", vec![Keys::Escape, Keys::ShiftDown]);
    map.insert("\x1b[1;4B", vec![Keys::Escape, Keys::ShiftUp]);
    map.insert("\x1b[1;4C", vec![Keys::Escape, Keys::ShiftRight]);
    map.insert("\x1b[1;4D", vec![Keys::Escape, Keys::ShiftLeft]);
    map.insert("\x1b[1;4F", vec![Keys::Escape, Keys::ShiftEnd]);
    map.insert("\x1b[1;4H", vec![Keys::Escape, Keys::ShiftHome]);
    // Control + arrows.
    map.insert("\x1b[1;5A", vec![Keys::ControlUp]); // Cursor Mode
    map.insert("\x1b[1;5B", vec![Keys::ControlDown]); // Cursor Mode
    map.insert("\x1b[1;5C", vec![Keys::ControlRight]); // Cursor Mode
    map.insert("\x1b[1;5D", vec![Keys::ControlLeft]); // Cursor Mode
    map.insert("\x1b[1;5F", vec![Keys::ControlEnd]);
    map.insert("\x1b[1;5H", vec![Keys::ControlHome]);
    // Tmux sends following keystrokes when control+arrow is pressed, but for
    // Emacs ansi-term sends the same sequences for normal arrow keys. Consider
    // it a normal arrow press, because that's more important.
    map.insert("\x1b[5A", vec![Keys::ControlUp]);
    map.insert("\x1b[5B", vec![Keys::ControlDown]);
    map.insert("\x1b[5C", vec![Keys::ControlRight]);
    map.insert("\x1b[5D", vec![Keys::ControlLeft]);
    map.insert("\x1bOc", vec![Keys::ControlRight]); // rxvt
    map.insert("\x1bOd", vec![Keys::ControlLeft]); // rxvt
                                                   // Control + shift + arrows.
    map.insert("\x1b[1;6A", vec![Keys::ControlShiftDown]);
    map.insert("\x1b[1;6B", vec![Keys::ControlShiftUp]);
    map.insert("\x1b[1;6C", vec![Keys::ControlShiftRight]);
    map.insert("\x1b[1;6D", vec![Keys::ControlShiftLeft]);
    map.insert("\x1b[1;6F", vec![Keys::ControlShiftEnd]);
    map.insert("\x1b[1;6H", vec![Keys::ControlShiftHome]);
    // Control + Meta + arrows.
    map.insert("\x1b[1;7A", vec![Keys::Escape, Keys::ControlDown]);
    map.insert("\x1b[1;7B", vec![Keys::Escape, Keys::ControlUp]);
    map.insert("\x1b[1;7C", vec![Keys::Escape, Keys::ControlRight]);
    map.insert("\x1b[1;7D", vec![Keys::Escape, Keys::ControlLeft]);
    map.insert("\x1b[1;7F", vec![Keys::Escape, Keys::ControlEnd]);
    map.insert("\x1b[1;7H", vec![Keys::Escape, Keys::ControlHome]);
    // Meta + Shift + arrows.
    map.insert("\x1b[1;8A", vec![Keys::Escape, Keys::ControlShiftDown]);
    map.insert("\x1b[1;8B", vec![Keys::Escape, Keys::ControlShiftUp]);
    map.insert("\x1b[1;8C", vec![Keys::Escape, Keys::ControlShiftRight]);
    map.insert("\x1b[1;8D", vec![Keys::Escape, Keys::ControlShiftLeft]);
    map.insert("\x1b[1;8F", vec![Keys::Escape, Keys::ControlShiftEnd]);
    map.insert("\x1b[1;8H", vec![Keys::Escape, Keys::ControlShiftHome]);
    // Meta + arrow on (some?) Macs when using iTerm defaults (see issue #483).
    map.insert("\x1b[1;9A", vec![Keys::Escape, Keys::Up]);
    map.insert("\x1b[1;9B", vec![Keys::Escape, Keys::Down]);
    map.insert("\x1b[1;9C", vec![Keys::Escape, Keys::Right]);
    map.insert("\x1b[1;9D", vec![Keys::Escape, Keys::Left]);
    // --
    // Control/shift/meta + number in mintty.
    // (c-2 will actually send c-@ and c-6 will send c-^.)
    map.insert("\x1b[1;5p", vec![Keys::Control0]);
    map.insert("\x1b[1;5q", vec![Keys::Control1]);
    map.insert("\x1b[1;5r", vec![Keys::Control2]);
    map.insert("\x1b[1;5s", vec![Keys::Control3]);
    map.insert("\x1b[1;5t", vec![Keys::Control4]);
    map.insert("\x1b[1;5u", vec![Keys::Control5]);
    map.insert("\x1b[1;5v", vec![Keys::Control6]);
    map.insert("\x1b[1;5w", vec![Keys::Control7]);
    map.insert("\x1b[1;5x", vec![Keys::Control8]);
    map.insert("\x1b[1;5y", vec![Keys::Control9]);
    map.insert("\x1b[1;6p", vec![Keys::ControlShift0]);
    map.insert("\x1b[1;6q", vec![Keys::ControlShift1]);
    map.insert("\x1b[1;6r", vec![Keys::ControlShift2]);
    map.insert("\x1b[1;6s", vec![Keys::ControlShift3]);
    map.insert("\x1b[1;6t", vec![Keys::ControlShift4]);
    map.insert("\x1b[1;6u", vec![Keys::ControlShift5]);
    map.insert("\x1b[1;6v", vec![Keys::ControlShift6]);
    map.insert("\x1b[1;6w", vec![Keys::ControlShift7]);
    map.insert("\x1b[1;6x", vec![Keys::ControlShift8]);
    map.insert("\x1b[1;6y", vec![Keys::ControlShift9]);
    map.insert("\x1b[1;7p", vec![Keys::Escape, Keys::Control0]);
    map.insert("\x1b[1;7q", vec![Keys::Escape, Keys::Control1]);
    map.insert("\x1b[1;7r", vec![Keys::Escape, Keys::Control2]);
    map.insert("\x1b[1;7s", vec![Keys::Escape, Keys::Control3]);
    map.insert("\x1b[1;7t", vec![Keys::Escape, Keys::Control4]);
    map.insert("\x1b[1;7u", vec![Keys::Escape, Keys::Control5]);
    map.insert("\x1b[1;7v", vec![Keys::Escape, Keys::Control6]);
    map.insert("\x1b[1;7w", vec![Keys::Escape, Keys::Control7]);
    map.insert("\x1b[1;7x", vec![Keys::Escape, Keys::Control8]);
    map.insert("\x1b[1;7y", vec![Keys::Escape, Keys::Control9]);
    map.insert("\x1b[1;8p", vec![Keys::Escape, Keys::ControlShift0]);
    map.insert("\x1b[1;8q", vec![Keys::Escape, Keys::ControlShift1]);
    map.insert("\x1b[1;8r", vec![Keys::Escape, Keys::ControlShift2]);
    map.insert("\x1b[1;8s", vec![Keys::Escape, Keys::ControlShift3]);
    map.insert("\x1b[1;8t", vec![Keys::Escape, Keys::ControlShift4]);
    map.insert("\x1b[1;8u", vec![Keys::Escape, Keys::ControlShift5]);
    map.insert("\x1b[1;8v", vec![Keys::Escape, Keys::ControlShift6]);
    map.insert("\x1b[1;8w", vec![Keys::Escape, Keys::ControlShift7]);
    map.insert("\x1b[1;8x", vec![Keys::Escape, Keys::ControlShift8]);
    map.insert("\x1b[1;8y", vec![Keys::Escape, Keys::ControlShift9]);

    map
});
