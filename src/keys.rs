// for reference see HID specs
// https://usb.org/sites/default/files/hut1_4.pdf#page=90
#[non_exhaustive]
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KeyboardKey {
    None = 0x00,

    ErrorRollOver = 0x01,
    ErrorPostFail = 0x02,
    ErrorUndefined = 0x03,

    PhyA = 0x04,
    PhyB = 0x05,
    PhyC = 0x06,
    PhyD = 0x07,
    PhyE = 0x08,
    PhyF = 0x09,
    PhyG = 0x0a,
    PhyH = 0x0b,
    PhyI = 0x0c,
    PhyJ = 0x0d,
    PhyK = 0x0e,
    PhyL = 0x0f,
    PhyM = 0x10,
    PhyN = 0x11,
    PhyO = 0x12,
    PhyP = 0x13,
    PhyQ = 0x14,
    PhyR = 0x15,
    PhyS = 0x16,
    PhyT = 0x17,
    PhyU = 0x18,
    PhyV = 0x19,
    PhyW = 0x1a,
    PhyX = 0x1b,
    PhyY = 0x1c,
    PhyZ = 0x1d,
    Phy1 = 0x1e,
    Phy2 = 0x1f,
    Phy3 = 0x20,
    Phy4 = 0x21,
    Phy5 = 0x22,
    Phy6 = 0x23,
    Phy7 = 0x24,
    Phy8 = 0x25,
    Phy9 = 0x26,
    Phy0 = 0x27,
    Enter = 0x28,
    Escape = 0x29,
    Backspace = 0x2a,
    Tab = 0x2b,
    Space = 0x2c,

    /// Hyphen or underscore in US layout, next to zero
    PhyHyphen = 0x2d,

    /// Equal sign or plus in US layout, next to backspace
    PhyEqual = 0x2e,

    /// Opening square bracket or opening brace in US layout, next to P
    PhyOpenBrace = 0x2f,

    /// Closing square bracket or closing brace in US layout
    PhyCloseBrace = 0x30,

    /// Backslash or pipe on a US keyboard
    PhyBackslash = 0x31,

    /// Backslash or pipe in US layout on a non-US keyboard,
    /// hash or tilde on GB keyboard, typically found near Enter
    PhyHash = 0x32,

    // Semicolon or colon in US layout, next to L
    PhySemicolon = 0x33,

    // Single or double quote in US layout, next to semicolon
    PhyQuote = 0x34,

    /// Grave accent and tilde in US layout, at the start of the button row
    PhyGrave = 0x35,

    PhyComma = 0x36,
    PhyPeriod = 0x37,
    PhySlash = 0x38,
    CapsLock = 0x39,
    F1 = 0x3a,
    F2 = 0x3b,
    F3 = 0x3c,
    F4 = 0x3d,
    F5 = 0x3e,
    F6 = 0x3f,
    F7 = 0x40,
    F8 = 0x41,
    F9 = 0x42,
    F10 = 0x43,
    F11 = 0x44,
    F12 = 0x45,
    PrintScreen = 0x46,
    ScrollLock = 0x47,
    Pause = 0x48,
    Insert = 0x49,
    Home = 0x4a,
    PageUp = 0x4b,
    Delete = 0x4c,
    End = 0x4d,
    PageDown = 0x4e,
    ArrowRight = 0x4f,
    ArrowLeft = 0x50,
    ArrowDown = 0x51,
    ArrowUp = 0x52,
    NumLock = 0x53,
    KeypadDivide = 0x54,
    KeypadMultiply = 0x55,
    KeypadMinus = 0x56,
    KeypadPlus = 0x57,
    KeypadEnter = 0x58,
    Keypad1 = 0x59,
    Keypad2 = 0x5a,
    Keypad3 = 0x5b,
    Keypad4 = 0x5c,
    Keypad5 = 0x5d,
    Keypad6 = 0x5e,
    Keypad7 = 0x5f,
    Keypad8 = 0x60,
    Keypad9 = 0x61,
    Keypad0 = 0x62,
    KeypadPoint = 0x63,

    /// Backslash or pipe in US layout on a non-US keyboard,
    /// angle brackets on various other layouts, near Left Shift
    PhysChevron = 0x64,

    LegacyApplication = 0x65,
    Power = 0x66,
    KeypadEqual = 0x67,
    F13 = 0x68,
    F14 = 0x69,
    F15 = 0x6a,
    F16 = 0x6b,
    F17 = 0x6c,
    F18 = 0x6d,
    F19 = 0x6e,
    F20 = 0x6f,
    F21 = 0x70,
    F22 = 0x71,
    F23 = 0x72,
    F24 = 0x73,
    Execute = 0x74,
    Help = 0x75,
    Menu = 0x76,
    Select = 0x77,
    Stop = 0x78,
    Again = 0x79,
    Undo = 0x7a,
    Cut = 0x7b,
    Copy = 0x7c,
    Paste = 0x7d,
    Find = 0x7e,
    Mute = 0x7f,
    VolumeUp = 0x80,
    VolumeDown = 0x81,
    LegacyLockingCapsLock = 0x82,
    LegacyLockingNumLock = 0x83,
    LegacyLockingScrollLock = 0x84,
    KeypadBrazilianComma = 0x85,
    LegacyKeypadEqual = 0x86,
    International1 = 0x87,
    International2 = 0x88,
    International3 = 0x89,
    International4 = 0x8a,
    International5 = 0x8b,
    International6 = 0x8c,
    International7 = 0x8d,
    International8 = 0x8e,
    International9 = 0x8f,
    International10 = 0x90,
    International11 = 0x91,
    International12 = 0x92,
    International13 = 0x93,
    International14 = 0x94,
    International15 = 0x95,
    International16 = 0x96,
    International17 = 0x97,
    International18 = 0x98,
    EraseAlt = 0x99,

    // TODO fill in 0x99-0xa4 and 0xb0-0xdd

    //
    ControlLeft = 0xe0,
    ShiftLeft = 0xe1,
    AltLeft = 0xe2,
    SuperLeft = 0xe3,
    ControlRight = 0xe4,
    ShiftRight = 0xe5,
    AltRight = 0xe6,
    SuperRight = 0xe7,
}
impl From<KeyboardKey> for u8 {
    fn from(key: KeyboardKey) -> u8 {
        key as u8
    }
}
