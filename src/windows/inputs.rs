use ::*;

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum KeybdKey {
    BackspaceKey = 0x08,
    TabKey = 0x09,
    EnterKey = 0x0D,
    EscapeKey = 0x1B,
    SpaceKey = 0x20,
    HomeKey = 0x24,
    LeftKey = 0x25,
    UpKey = 0x26,
    RightKey = 0x27,
    DownKey = 0x28,
    InsertKey = 0x2D,
    DeleteKey = 0x2E,
    Numrow0Key = 0x30,
    Numrow1Key = 0x31,
    Numrow2Key = 0x32,
    Numrow3Key = 0x33,
    Numrow4Key = 0x34,
    Numrow5Key = 0x35,
    Numrow6Key = 0x36,
    Numrow7Key = 0x37,
    Numrow8Key = 0x38,
    Numrow9Key = 0x39,
    AKey = 0x41,
    BKey = 0x42,
    CKey = 0x43,
    DKey = 0x44,
    EKey = 0x45,
    FKey = 0x46,
    GKey = 0x47,
    HKey = 0x48,
    IKey = 0x49,
    JKey = 0x4A,
    KKey = 0x4B,
    LKey = 0x4C,
    MKey = 0x4D,
    NKey = 0x4E,
    OKey = 0x4F,
    PKey = 0x50,
    QKey = 0x51,
    RKey = 0x52,
    SKey = 0x53,
    TKey = 0x54,
    UKey = 0x55,
    VKey = 0x56,
    WKey = 0x57,
    XKey = 0x58,
    YKey = 0x59,
    ZKey = 0x5A,
    Numpad0Key = 0x60,
    Numpad1Key = 0x61,
    Numpad2Key = 0x62,
    Numpad3Key = 0x63,
    Numpad4Key = 0x64,
    Numpad5Key = 0x65,
    Numpad6Key = 0x66,
    Numpad7Key = 0x67,
    Numpad8Key = 0x68,
    Numpad9Key = 0x69,
    F1Key = 0x70,
    F2Key = 0x71,
    F3Key = 0x72,
    F4Key = 0x73,
    F5Key = 0x74,
    F6Key = 0x75,
    F7Key = 0x76,
    F8Key = 0x77,
    F9Key = 0x78,
    F10Key = 0x79,
    F11Key = 0x7A,
    F12Key = 0x7B,
    NumLockKey = 0x90,
    ScrollLockKey = 0x91,
    CapsLockKey = 0x14,
    LShiftKey = 0xA0,
    RShiftKey = 0xA1,
    LControlKey = 0xA2,
    RControlKey = 0xA3,
}

impl From<MouseButton> for u32 {
    fn from(button: MouseButton) -> u32 {
        match button {
            MouseButton::LeftButton => 0x01,
            MouseButton::RightButton => 0x02,
            MouseButton::MiddleButton => 0x04,
            MouseButton::X1Button => 0x05,
            MouseButton::X2Button => 0x06,
            MouseButton::OtherButton(code) => code,
        }
    }
}
