//! Back-end agnostic keyboard keys.

use std::default::Default;

use crate::{Button, GenericEvent};

// Defining every combination to allow assignment in static expressions.
bitflags!(
    #[allow(missing_docs)]
    #[derive(Deserialize, Serialize)]
    pub struct ModifierKey: u8 {
        /// No modifier.
        const NO_MODIFIER           = 0b0000_0000;
        /// Ctrl.
        const CTRL                  = 0b0000_0001;
        /// Shift.
        const SHIFT                 = 0b0000_0010;
        /// Alt.
        const ALT                   = 0b0000_0100;
        /// Gui.
        const GUI                   = 0b0000_1000;
        /// Ctrl + Shift.
        const CTRL_SHIFT            = ModifierKey::CTRL.bits
                                    | ModifierKey::SHIFT.bits;
        /// Ctrl + Alt.
        const CTRL_ALT              = ModifierKey::CTRL.bits
                                    | ModifierKey::ALT.bits;
        /// Ctrl + Gui.
        const CTRL_GUI              = ModifierKey::CTRL.bits
                                    | ModifierKey::GUI.bits;
        /// Ctrl + Shift + Alt.
        const CTRL_SHIFT_ALT        = ModifierKey::CTRL.bits
                                    | ModifierKey::SHIFT.bits
                                    | ModifierKey::ALT.bits;
        /// Ctrl + Shift + Gui.
        const CTRL_SHIFT_GUI        = ModifierKey::CTRL.bits
                                    | ModifierKey::SHIFT.bits
                                    | ModifierKey::GUI.bits;
        /// Ctrl + Shift + Alt + Gui.
        const CTRL_SHIFT_ALT_GUI    = ModifierKey::CTRL.bits
                                    | ModifierKey::SHIFT.bits
                                    | ModifierKey::ALT.bits
                                    | ModifierKey::GUI.bits;
        /// Shift + Alt.
        const SHIFT_ALT             = ModifierKey::SHIFT.bits
                                    | ModifierKey::ALT.bits;
        /// Shift + Gui.
        const SHIFT_GUI             = ModifierKey::SHIFT.bits
                                    | ModifierKey::GUI.bits;
        /// Shift + Alt + Gui.
        const SHIFT_ALT_GUI         = ModifierKey::SHIFT.bits
                                    | ModifierKey::ALT.bits
                                    | ModifierKey::GUI.bits;
        /// Alt + Gui.
        const ALT_GUI               = ModifierKey::ALT.bits
                                    | ModifierKey::GUI.bits;
    }
);

impl ModifierKey {
    /// Change modifier key state depending on input.
    ///
    /// If the left or side button is released, it counts as a release.
    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(Key::LCtrl | Key::RCtrl) => self.insert(ModifierKey::CTRL),
                Button::Keyboard(Key::LShift | Key::RShift) => self.insert(ModifierKey::SHIFT),
                Button::Keyboard(Key::LAlt | Key::RAlt) => self.insert(ModifierKey::ALT),
                Button::Keyboard(Key::LGui | Key::RGui) => self.insert(ModifierKey::GUI),
                _ => {}
            }
        }
        if let Some(button) = e.release_args() {
            match button {
                Button::Keyboard(Key::LCtrl | Key::RCtrl) => self.remove(ModifierKey::CTRL),
                Button::Keyboard(Key::LShift | Key::RShift) => self.remove(ModifierKey::SHIFT),
                Button::Keyboard(Key::LAlt | Key::RAlt) => self.remove(ModifierKey::ALT),
                Button::Keyboard(Key::LGui | Key::RGui) => self.remove(ModifierKey::GUI),
                _ => {}
            }
        }
        if let Some(false) = e.focus_args() {
            *self = ModifierKey::NO_MODIFIER;
        }
    }
}

impl Default for ModifierKey {
    fn default() -> ModifierKey {
        ModifierKey::NO_MODIFIER
    }
}

/// Represent a keyboard key.
/// Keycodes follows SDL <http://wiki.libsdl.org/SDLKeycodeLookup>
#[allow(missing_docs)]
#[derive(Copy, Clone, Deserialize, Serialize, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub enum Key {
    Unknown = 0x00,
    Backspace = 0x08,
    Tab = 0x09,
    Return = 0x0D,
    Escape = 0x1B,
    Space = 0x20,
    Exclaim = 0x21,
    Quotedbl = 0x22,
    Hash = 0x23,
    Dollar = 0x24,
    Percent = 0x25,
    Ampersand = 0x26,
    Quote = 0x27,
    LeftParen = 0x28,
    RightParen = 0x29,
    Asterisk = 0x2A,
    Plus = 0x2B,
    Comma = 0x2C,
    Minus = 0x2D,
    Period = 0x2E,
    Slash = 0x2F,
    D0 = 0x30,
    D1 = 0x31,
    D2 = 0x32,
    D3 = 0x33,
    D4 = 0x34,
    D5 = 0x35,
    D6 = 0x36,
    D7 = 0x37,
    D8 = 0x38,
    D9 = 0x39,
    Colon = 0x3A,
    Semicolon = 0x3B,
    Less = 0x3C,
    Equals = 0x3D,
    Greater = 0x3E,
    Question = 0x3F,
    At = 0x40,
    LeftBracket = 0x5B,
    Backslash = 0x5C,
    RightBracket = 0x5D,
    Caret = 0x5E,
    Underscore = 0x5F,
    Backquote = 0x60,
    A = 0x61,
    B = 0x62,
    C = 0x63,
    D = 0x64,
    E = 0x65,
    F = 0x66,
    G = 0x67,
    H = 0x68,
    I = 0x69,
    J = 0x6A,
    K = 0x6B,
    L = 0x6C,
    M = 0x6D,
    N = 0x6E,
    O = 0x6F,
    P = 0x70,
    Q = 0x71,
    R = 0x72,
    S = 0x73,
    T = 0x74,
    U = 0x75,
    V = 0x76,
    W = 0x77,
    X = 0x78,
    Y = 0x79,
    Z = 0x7A,
    Delete = 0x7F,
    CapsLock = 0x4000_0039,
    F1 = 0x4000_003A,
    F2 = 0x4000_003B,
    F3 = 0x4000_003C,
    F4 = 0x4000_003D,
    F5 = 0x4000_003E,
    F6 = 0x4000_003F,
    F7 = 0x4000_0040,
    F8 = 0x4000_0041,
    F9 = 0x4000_0042,
    F10 = 0x4000_0043,
    F11 = 0x4000_0044,
    F12 = 0x4000_0045,
    PrintScreen = 0x4000_0046,
    ScrollLock = 0x4000_0047,
    Pause = 0x4000_0048,
    Insert = 0x4000_0049,
    Home = 0x4000_004A,
    PageUp = 0x4000_004B,
    End = 0x4000_004D,
    PageDown = 0x4000_004E,
    Right = 0x4000_004F,
    Left = 0x4000_0050,
    Down = 0x4000_0051,
    Up = 0x4000_0052,
    NumLockClear = 0x4000_0053,
    NumPadDivide = 0x4000_0054,
    NumPadMultiply = 0x4000_0055,
    NumPadMinus = 0x4000_0056,
    NumPadPlus = 0x4000_0057,
    NumPadEnter = 0x4000_0058,
    NumPad1 = 0x4000_0059,
    NumPad2 = 0x4000_005A,
    NumPad3 = 0x4000_005B,
    NumPad4 = 0x4000_005C,
    NumPad5 = 0x4000_005D,
    NumPad6 = 0x4000_005E,
    NumPad7 = 0x4000_005F,
    NumPad8 = 0x4000_0060,
    NumPad9 = 0x4000_0061,
    NumPad0 = 0x4000_0062,
    NumPadPeriod = 0x4000_0063,
    Application = 0x4000_0065,
    Power = 0x4000_0066,
    NumPadEquals = 0x4000_0067,
    F13 = 0x4000_0068,
    F14 = 0x4000_0069,
    F15 = 0x4000_006A,
    F16 = 0x4000_006B,
    F17 = 0x4000_006C,
    F18 = 0x4000_006D,
    F19 = 0x4000_006E,
    F20 = 0x4000_006F,
    F21 = 0x4000_0070,
    F22 = 0x4000_0071,
    F23 = 0x4000_0072,
    F24 = 0x4000_0073,
    Execute = 0x4000_0074,
    Help = 0x4000_0075,
    Menu = 0x4000_0076,
    Select = 0x4000_0077,
    Stop = 0x4000_0078,
    Again = 0x4000_0079,
    Undo = 0x4000_007A,
    Cut = 0x4000_007B,
    Copy = 0x4000_007C,
    Paste = 0x4000_007D,
    Find = 0x4000_007E,
    Mute = 0x4000_007F,
    VolumeUp = 0x4000_0080,
    VolumeDown = 0x4000_0081,
    NumPadComma = 0x4000_0085,
    NumPadEqualsAS400 = 0x4000_0086,
    AltErase = 0x4000_0099,
    Sysreq = 0x4000_009A,
    Cancel = 0x4000_009B,
    Clear = 0x4000_009C,
    Prior = 0x4000_009D,
    Return2 = 0x4000_009E,
    Separator = 0x4000_009F,
    Out = 0x4000_00A0,
    Oper = 0x4000_00A1,
    ClearAgain = 0x4000_00A2,
    CrSel = 0x4000_00A3,
    ExSel = 0x4000_00A4,
    NumPad00 = 0x4000_00B0,
    NumPad000 = 0x4000_00B1,
    ThousandsSeparator = 0x4000_00B2,
    DecimalSeparator = 0x4000_00B3,
    CurrencyUnit = 0x4000_00B4,
    CurrencySubUnit = 0x4000_00B5,
    NumPadLeftParen = 0x4000_00B6,
    NumPadRightParen = 0x4000_00B7,
    NumPadLeftBrace = 0x4000_00B8,
    NumPadRightBrace = 0x4000_00B9,
    NumPadTab = 0x4000_00BA,
    NumPadBackspace = 0x4000_00BB,
    NumPadA = 0x4000_00BC,
    NumPadB = 0x4000_00BD,
    NumPadC = 0x4000_00BE,
    NumPadD = 0x4000_00BF,
    NumPadE = 0x4000_00C0,
    NumPadF = 0x4000_00C1,
    NumPadXor = 0x4000_00C2,
    NumPadPower = 0x4000_00C3,
    NumPadPercent = 0x4000_00C4,
    NumPadLess = 0x4000_00C5,
    NumPadGreater = 0x4000_00C6,
    NumPadAmpersand = 0x4000_00C7,
    NumPadDblAmpersand = 0x4000_00C8,
    NumPadVerticalBar = 0x4000_00C9,
    NumPadDblVerticalBar = 0x4000_00CA,
    NumPadColon = 0x4000_00CB,
    NumPadHash = 0x4000_00CC,
    NumPadSpace = 0x4000_00CD,
    NumPadAt = 0x4000_00CE,
    NumPadExclam = 0x4000_00CF,
    NumPadMemStore = 0x4000_00D0,
    NumPadMemRecall = 0x4000_00D1,
    NumPadMemClear = 0x4000_00D2,
    NumPadMemAdd = 0x4000_00D3,
    NumPadMemSubtract = 0x4000_00D4,
    NumPadMemMultiply = 0x4000_00D5,
    NumPadMemDivide = 0x4000_00D6,
    NumPadPlusMinus = 0x4000_00D7,
    NumPadClear = 0x4000_00D8,
    NumPadClearEntry = 0x4000_00D9,
    NumPadBinary = 0x4000_00DA,
    NumPadOctal = 0x4000_00DB,
    NumPadDecimal = 0x4000_00DC,
    NumPadHexadecimal = 0x4000_00DD,
    LCtrl = 0x4000_00E0,
    LShift = 0x4000_00E1,
    LAlt = 0x4000_00E2,
    LGui = 0x4000_00E3,
    RCtrl = 0x4000_00E4,
    RShift = 0x4000_00E5,
    RAlt = 0x4000_00E6,
    RGui = 0x4000_00E7,
    Mode = 0x4000_0101,
    AudioNext = 0x4000_0102,
    AudioPrev = 0x4000_0103,
    AudioStop = 0x4000_0104,
    AudioPlay = 0x4000_0105,
    AudioMute = 0x4000_0106,
    MediaSelect = 0x4000_0107,
    Www = 0x4000_0108,
    Mail = 0x4000_0109,
    Calculator = 0x4000_010A,
    Computer = 0x4000_010B,
    AcSearch = 0x4000_010C,
    AcHome = 0x4000_010D,
    AcBack = 0x4000_010E,
    AcForward = 0x4000_010F,
    AcStop = 0x4000_0110,
    AcRefresh = 0x4000_0111,
    AcBookmarks = 0x4000_0112,
    BrightnessDown = 0x4000_0113,
    BrightnessUp = 0x4000_0114,
    DisplaySwitch = 0x4000_0115,
    KbdIllumToggle = 0x4000_0116,
    KbdIllumDown = 0x4000_0117,
    KbdIllumUp = 0x4000_0118,
    Eject = 0x4000_0119,
    Sleep = 0x4000_011A,
}

impl From<u32> for Key {
    fn from(val: u32) -> Key {
        match val {
            0x00 => Key::Unknown,
            0x08 => Key::Backspace,
            0x09 => Key::Tab,
            0x0D => Key::Return,
            0x1B => Key::Escape,
            0x20 => Key::Space,
            0x21 => Key::Exclaim,
            0x22 => Key::Quotedbl,
            0x23 => Key::Hash,
            0x24 => Key::Dollar,
            0x25 => Key::Percent,
            0x26 => Key::Ampersand,
            0x27 => Key::Quote,
            0x28 => Key::LeftParen,
            0x29 => Key::RightParen,
            0x2A => Key::Asterisk,
            0x2B => Key::Plus,
            0x2C => Key::Comma,
            0x2D => Key::Minus,
            0x2E => Key::Period,
            0x2F => Key::Slash,
            0x30 => Key::D0,
            0x31 => Key::D1,
            0x32 => Key::D2,
            0x33 => Key::D3,
            0x34 => Key::D4,
            0x35 => Key::D5,
            0x36 => Key::D6,
            0x37 => Key::D7,
            0x38 => Key::D8,
            0x39 => Key::D9,
            0x3A => Key::Colon,
            0x3B => Key::Semicolon,
            0x3C => Key::Less,
            0x3D => Key::Equals,
            0x3E => Key::Greater,
            0x3F => Key::Question,
            0x40 => Key::At,
            0x5B => Key::LeftBracket,
            0x5C => Key::Backslash,
            0x5D => Key::RightBracket,
            0x5E => Key::Caret,
            0x5F => Key::Underscore,
            0x60 => Key::Backquote,
            0x61 => Key::A,
            0x62 => Key::B,
            0x63 => Key::C,
            0x64 => Key::D,
            0x65 => Key::E,
            0x66 => Key::F,
            0x67 => Key::G,
            0x68 => Key::H,
            0x69 => Key::I,
            0x6A => Key::J,
            0x6B => Key::K,
            0x6C => Key::L,
            0x6D => Key::M,
            0x6E => Key::N,
            0x6F => Key::O,
            0x70 => Key::P,
            0x71 => Key::Q,
            0x72 => Key::R,
            0x73 => Key::S,
            0x74 => Key::T,
            0x75 => Key::U,
            0x76 => Key::V,
            0x77 => Key::W,
            0x78 => Key::X,
            0x79 => Key::Y,
            0x7A => Key::Z,
            0x7F => Key::Delete,
            0x4000_0039 => Key::CapsLock,
            0x4000_003A => Key::F1,
            0x4000_003B => Key::F2,
            0x4000_003C => Key::F3,
            0x4000_003D => Key::F4,
            0x4000_003E => Key::F5,
            0x4000_003F => Key::F6,
            0x4000_0040 => Key::F7,
            0x4000_0041 => Key::F8,
            0x4000_0042 => Key::F9,
            0x4000_0043 => Key::F10,
            0x4000_0044 => Key::F11,
            0x4000_0045 => Key::F12,
            0x4000_0046 => Key::PrintScreen,
            0x4000_0047 => Key::ScrollLock,
            0x4000_0048 => Key::Pause,
            0x4000_0049 => Key::Insert,
            0x4000_004A => Key::Home,
            0x4000_004B => Key::PageUp,
            0x4000_004D => Key::End,
            0x4000_004E => Key::PageDown,
            0x4000_004F => Key::Right,
            0x4000_0050 => Key::Left,
            0x4000_0051 => Key::Down,
            0x4000_0052 => Key::Up,
            0x4000_0053 => Key::NumLockClear,
            0x4000_0054 => Key::NumPadDivide,
            0x4000_0055 => Key::NumPadMultiply,
            0x4000_0056 => Key::NumPadMinus,
            0x4000_0057 => Key::NumPadPlus,
            0x4000_0058 => Key::NumPadEnter,
            0x4000_0059 => Key::NumPad1,
            0x4000_005A => Key::NumPad2,
            0x4000_005B => Key::NumPad3,
            0x4000_005C => Key::NumPad4,
            0x4000_005D => Key::NumPad5,
            0x4000_005E => Key::NumPad6,
            0x4000_005F => Key::NumPad7,
            0x4000_0060 => Key::NumPad8,
            0x4000_0061 => Key::NumPad9,
            0x4000_0062 => Key::NumPad0,
            0x4000_0063 => Key::NumPadPeriod,
            0x4000_0065 => Key::Application,
            0x4000_0066 => Key::Power,
            0x4000_0067 => Key::NumPadEquals,
            0x4000_0068 => Key::F13,
            0x4000_0069 => Key::F14,
            0x4000_006A => Key::F15,
            0x4000_006B => Key::F16,
            0x4000_006C => Key::F17,
            0x4000_006D => Key::F18,
            0x4000_006E => Key::F19,
            0x4000_006F => Key::F20,
            0x4000_0070 => Key::F21,
            0x4000_0071 => Key::F22,
            0x4000_0072 => Key::F23,
            0x4000_0073 => Key::F24,
            0x4000_0074 => Key::Execute,
            0x4000_0075 => Key::Help,
            0x4000_0076 => Key::Menu,
            0x4000_0077 => Key::Select,
            0x4000_0078 => Key::Stop,
            0x4000_0079 => Key::Again,
            0x4000_007A => Key::Undo,
            0x4000_007B => Key::Cut,
            0x4000_007C => Key::Copy,
            0x4000_007D => Key::Paste,
            0x4000_007E => Key::Find,
            0x4000_007F => Key::Mute,
            0x4000_0080 => Key::VolumeUp,
            0x4000_0081 => Key::VolumeDown,
            0x4000_0085 => Key::NumPadComma,
            0x4000_0086 => Key::NumPadEqualsAS400,
            0x4000_0099 => Key::AltErase,
            0x4000_009A => Key::Sysreq,
            0x4000_009B => Key::Cancel,
            0x4000_009C => Key::Clear,
            0x4000_009D => Key::Prior,
            0x4000_009E => Key::Return2,
            0x4000_009F => Key::Separator,
            0x4000_00A0 => Key::Out,
            0x4000_00A1 => Key::Oper,
            0x4000_00A2 => Key::ClearAgain,
            0x4000_00A3 => Key::CrSel,
            0x4000_00A4 => Key::ExSel,
            0x4000_00B0 => Key::NumPad00,
            0x4000_00B1 => Key::NumPad000,
            0x4000_00B2 => Key::ThousandsSeparator,
            0x4000_00B3 => Key::DecimalSeparator,
            0x4000_00B4 => Key::CurrencyUnit,
            0x4000_00B5 => Key::CurrencySubUnit,
            0x4000_00B6 => Key::NumPadLeftParen,
            0x4000_00B7 => Key::NumPadRightParen,
            0x4000_00B8 => Key::NumPadLeftBrace,
            0x4000_00B9 => Key::NumPadRightBrace,
            0x4000_00BA => Key::NumPadTab,
            0x4000_00BB => Key::NumPadBackspace,
            0x4000_00BC => Key::NumPadA,
            0x4000_00BD => Key::NumPadB,
            0x4000_00BE => Key::NumPadC,
            0x4000_00BF => Key::NumPadD,
            0x4000_00C0 => Key::NumPadE,
            0x4000_00C1 => Key::NumPadF,
            0x4000_00C2 => Key::NumPadXor,
            0x4000_00C3 => Key::NumPadPower,
            0x4000_00C4 => Key::NumPadPercent,
            0x4000_00C5 => Key::NumPadLess,
            0x4000_00C6 => Key::NumPadGreater,
            0x4000_00C7 => Key::NumPadAmpersand,
            0x4000_00C8 => Key::NumPadDblAmpersand,
            0x4000_00C9 => Key::NumPadVerticalBar,
            0x4000_00CA => Key::NumPadDblVerticalBar,
            0x4000_00CB => Key::NumPadColon,
            0x4000_00CC => Key::NumPadHash,
            0x4000_00CD => Key::NumPadSpace,
            0x4000_00CE => Key::NumPadAt,
            0x4000_00CF => Key::NumPadExclam,
            0x4000_00D0 => Key::NumPadMemStore,
            0x4000_00D1 => Key::NumPadMemRecall,
            0x4000_00D2 => Key::NumPadMemClear,
            0x4000_00D3 => Key::NumPadMemAdd,
            0x4000_00D4 => Key::NumPadMemSubtract,
            0x4000_00D5 => Key::NumPadMemMultiply,
            0x4000_00D6 => Key::NumPadMemDivide,
            0x4000_00D7 => Key::NumPadPlusMinus,
            0x4000_00D8 => Key::NumPadClear,
            0x4000_00D9 => Key::NumPadClearEntry,
            0x4000_00DA => Key::NumPadBinary,
            0x4000_00DB => Key::NumPadOctal,
            0x4000_00DC => Key::NumPadDecimal,
            0x4000_00DD => Key::NumPadHexadecimal,
            0x4000_00E0 => Key::LCtrl,
            0x4000_00E1 => Key::LShift,
            0x4000_00E2 => Key::LAlt,
            0x4000_00E3 => Key::LGui,
            0x4000_00E4 => Key::RCtrl,
            0x4000_00E5 => Key::RShift,
            0x4000_00E6 => Key::RAlt,
            0x4000_00E7 => Key::RGui,
            0x4000_0101 => Key::Mode,
            0x4000_0102 => Key::AudioNext,
            0x4000_0103 => Key::AudioPrev,
            0x4000_0104 => Key::AudioStop,
            0x4000_0105 => Key::AudioPlay,
            0x4000_0106 => Key::AudioMute,
            0x4000_0107 => Key::MediaSelect,
            0x4000_0108 => Key::Www,
            0x4000_0109 => Key::Mail,
            0x4000_010A => Key::Calculator,
            0x4000_010B => Key::Computer,
            0x4000_010C => Key::AcSearch,
            0x4000_010D => Key::AcHome,
            0x4000_010E => Key::AcBack,
            0x4000_010F => Key::AcForward,
            0x4000_0110 => Key::AcStop,
            0x4000_0111 => Key::AcRefresh,
            0x4000_0112 => Key::AcBookmarks,
            0x4000_0113 => Key::BrightnessDown,
            0x4000_0114 => Key::BrightnessUp,
            0x4000_0115 => Key::DisplaySwitch,
            0x4000_0116 => Key::KbdIllumToggle,
            0x4000_0117 => Key::KbdIllumDown,
            0x4000_0118 => Key::KbdIllumUp,
            0x4000_0119 => Key::Eject,
            0x4000_011A => Key::Sleep,

            _ => Key::Unknown,
        }
    }
}

impl Key {
    /// Returns an id of the key
    #[inline(always)]
    pub fn code(&self) -> i32 {
        *self as i32
    }
}

impl From<Key> for u32 {
    #[inline(always)]
    fn from(key: Key) -> u32 {
        key as u32
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn keycode() {
        use super::{Key, Key::*};

        let keys = vec![
            Unknown,
            Backspace,
            Tab,
            Return,
            Escape,
            Space,
            Exclaim,
            Quotedbl,
            Hash,
            Dollar,
            Percent,
            Ampersand,
            Quote,
            LeftParen,
            RightParen,
            Asterisk,
            Plus,
            Comma,
            Minus,
            Period,
            Slash,
            D0,
            D1,
            D2,
            D3,
            D4,
            D5,
            D6,
            D7,
            D8,
            D9,
            Colon,
            Semicolon,
            Less,
            Equals,
            Greater,
            Question,
            At,
            LeftBracket,
            Backslash,
            RightBracket,
            Caret,
            Underscore,
            Backquote,
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
            P,
            Q,
            R,
            S,
            T,
            U,
            V,
            W,
            X,
            Y,
            Z,
            Delete,
            CapsLock,
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
            PrintScreen,
            ScrollLock,
            Pause,
            Insert,
            Home,
            PageUp,
            End,
            PageDown,
            Right,
            Left,
            Down,
            Up,
            NumLockClear,
            NumPadDivide,
            NumPadMultiply,
            NumPadMinus,
            NumPadPlus,
            NumPadEnter,
            NumPad1,
            NumPad2,
            NumPad3,
            NumPad4,
            NumPad5,
            NumPad6,
            NumPad7,
            NumPad8,
            NumPad9,
            NumPad0,
            NumPadPeriod,
            Application,
            Power,
            NumPadEquals,
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
            Execute,
            Help,
            Menu,
            Select,
            Stop,
            Again,
            Undo,
            Cut,
            Copy,
            Paste,
            Find,
            Mute,
            VolumeUp,
            VolumeDown,
            NumPadComma,
            NumPadEqualsAS400,
            AltErase,
            Sysreq,
            Cancel,
            Clear,
            Prior,
            Return2,
            Separator,
            Out,
            Oper,
            ClearAgain,
            CrSel,
            ExSel,
            NumPad00,
            NumPad000,
            ThousandsSeparator,
            DecimalSeparator,
            CurrencyUnit,
            CurrencySubUnit,
            NumPadLeftParen,
            NumPadRightParen,
            NumPadLeftBrace,
            NumPadRightBrace,
            NumPadTab,
            NumPadBackspace,
            NumPadA,
            NumPadB,
            NumPadC,
            NumPadD,
            NumPadE,
            NumPadF,
            NumPadXor,
            NumPadPower,
            NumPadPercent,
            NumPadLess,
            NumPadGreater,
            NumPadAmpersand,
            NumPadDblAmpersand,
            NumPadVerticalBar,
            NumPadDblVerticalBar,
            NumPadColon,
            NumPadHash,
            NumPadSpace,
            NumPadAt,
            NumPadExclam,
            NumPadMemStore,
            NumPadMemRecall,
            NumPadMemClear,
            NumPadMemAdd,
            NumPadMemSubtract,
            NumPadMemMultiply,
            NumPadMemDivide,
            NumPadPlusMinus,
            NumPadClear,
            NumPadClearEntry,
            NumPadBinary,
            NumPadOctal,
            NumPadDecimal,
            NumPadHexadecimal,
            LCtrl,
            LShift,
            LAlt,
            LGui,
            RCtrl,
            RShift,
            RAlt,
            RGui,
            Mode,
            AudioNext,
            AudioPrev,
            AudioStop,
            AudioPlay,
            AudioMute,
            MediaSelect,
            Www,
            Mail,
            Calculator,
            Computer,
            AcSearch,
            AcHome,
            AcBack,
            AcForward,
            AcStop,
            AcRefresh,
            AcBookmarks,
            BrightnessDown,
            BrightnessUp,
            DisplaySwitch,
            KbdIllumToggle,
            KbdIllumDown,
            KbdIllumUp,
            Eject,
            Sleep,
        ];
        for &key in &keys {
            let val: u32 = key.into();
            let key2: Key = val.into();
            assert_eq!(key, key2);
        }
    }
}
