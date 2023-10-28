#[repr(C)]
pub enum Keycode {
    // LATIN CHARACTERS
    Q = 1,
    W = 2,
    E = 3,
    R = 4,
    T = 5,
    Y = 6,
    U = 7,
    I = 8,
    O = 9,
    P = 10,
    A = 11,
    S = 12,
    D = 13,
    F = 14,
    G = 15,
    H = 16,
    J = 17,
    K = 18,
    L = 19,
    Z = 20,
    X = 21,
    C = 22,
    V = 23,
    B = 24,
    N = 25,
    M = 26,

    // NUMBERS
    Num0 = 27,
    Num1 = 28,
    Num2 = 29,
    Num3 = 30,
    Num4 = 31,
    Num5 = 32,
    Num6 = 33,
    Num7 = 34,
    Num8 = 35,
    Num9 = 36,

    // PUNCTUNATION, ARTITHMETIC OPERATORS
    Minus = 37,
    Equal = 38,

    LsquareBracket = 39,
    RsquareBracket = 40,
    Semicolon = 41,
    Quote = 42,
    Backslash = 43,
    Comma = 44,
    Dot = 45,
    Slash = 46,
    Tilde = 47,
    Paragraph = 48,

    // NUMPAD
    Numpad0 = 49,
    Numpad1 = 50,
    Numpad2 = 51,
    Numpad3 = 52,
    Numpad4 = 53,
    Numpad5 = 54,
    Numpad6 = 55,
    Numpad7 = 56,
    Numpad8 = 57,
    Numpad9 = 58,
    NumpadNumLockClear = 59,
    NumpadDiv = 60,
    NumpadMul = 61,
    NumpadSub = 62,
    NumpadAdd = 63,
    NumpadEnter = 64,
    NumpadDot = 65,
    NumpadEqual = 66,

    // FUNCTIONAL
    F1 = 67,
    F2 = 68,
    F3 = 69,
    F4 = 70,
    F5 = 71,
    F6 = 72,
    F7 = 73,
    F8 = 74,
    F9 = 75,
    F10 = 76,
    F11 = 77,
    F12 = 78,
    F13 = 79,
    F14 = 80,
    F15 = 81,
    F16 = 82,
    F17 = 83,
    F18 = 84,
    F19 = 85,

    // Control buttons
    Esc = 86,
    Enter = 87,
    Space = 88,
    LShift = 89,
    RShift = 90,
    LOptionLAlt = 91,
    ROptionRAlt = 92,
    LCmdLCtrl = 93,
    RCmdRCtrl = 94,
    LControlLWin = 95,
    RControlRWin = 96,

    CapsLock = 97,
    Tab = 98,
    LeftArrow = 99,
    RightArrow = 100,
    UpArrow = 101,
    DownArrow = 102,
    Backspace = 103,
    Delete = 104,
    Insert = 105,
    Home = 106,
    End = 107,
    PageUp = 108,
    PageDown = 109,

    Print = 110,
    Scroll = 111,
    Pause = 112,
    
    Eject = 113,
}
