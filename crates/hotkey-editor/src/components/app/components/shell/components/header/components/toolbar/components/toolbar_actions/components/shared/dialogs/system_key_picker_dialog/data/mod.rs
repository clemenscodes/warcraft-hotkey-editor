use warcraft_keybinds::{Digit, FunctionKey, KeyCode, Letter, MouseButton, NumpadKey, Punctuation};

pub(super) struct BoardKey {
    pub(super) code: KeyCode,
    pub(super) label: &'static str,
}

pub(super) static KEYBOARD_ROWS: &[&[BoardKey]] = &[
    &[
        BoardKey {
            code: KeyCode::Escape,
            label: "Esc",
        },
        BoardKey {
            code: KeyCode::Function(FunctionKey::F1),
            label: "F1",
        },
        BoardKey {
            code: KeyCode::Function(FunctionKey::F2),
            label: "F2",
        },
        BoardKey {
            code: KeyCode::Function(FunctionKey::F3),
            label: "F3",
        },
        BoardKey {
            code: KeyCode::Function(FunctionKey::F4),
            label: "F4",
        },
        BoardKey {
            code: KeyCode::Function(FunctionKey::F5),
            label: "F5",
        },
        BoardKey {
            code: KeyCode::Function(FunctionKey::F6),
            label: "F6",
        },
        BoardKey {
            code: KeyCode::Function(FunctionKey::F7),
            label: "F7",
        },
        BoardKey {
            code: KeyCode::Function(FunctionKey::F8),
            label: "F8",
        },
        BoardKey {
            code: KeyCode::Function(FunctionKey::F9),
            label: "F9",
        },
        BoardKey {
            code: KeyCode::Function(FunctionKey::F10),
            label: "F10",
        },
        BoardKey {
            code: KeyCode::Function(FunctionKey::F11),
            label: "F11",
        },
        BoardKey {
            code: KeyCode::Function(FunctionKey::F12),
            label: "F12",
        },
    ],
    &[
        BoardKey {
            code: KeyCode::Punctuation(Punctuation::Backtick),
            label: "`",
        },
        BoardKey {
            code: KeyCode::Digit(Digit::One),
            label: "1",
        },
        BoardKey {
            code: KeyCode::Digit(Digit::Two),
            label: "2",
        },
        BoardKey {
            code: KeyCode::Digit(Digit::Three),
            label: "3",
        },
        BoardKey {
            code: KeyCode::Digit(Digit::Four),
            label: "4",
        },
        BoardKey {
            code: KeyCode::Digit(Digit::Five),
            label: "5",
        },
        BoardKey {
            code: KeyCode::Digit(Digit::Six),
            label: "6",
        },
        BoardKey {
            code: KeyCode::Digit(Digit::Seven),
            label: "7",
        },
        BoardKey {
            code: KeyCode::Digit(Digit::Eight),
            label: "8",
        },
        BoardKey {
            code: KeyCode::Digit(Digit::Nine),
            label: "9",
        },
        BoardKey {
            code: KeyCode::Digit(Digit::Zero),
            label: "0",
        },
        BoardKey {
            code: KeyCode::Punctuation(Punctuation::Minus),
            label: "-",
        },
        BoardKey {
            code: KeyCode::Punctuation(Punctuation::Equals),
            label: "=",
        },
    ],
    &[
        BoardKey {
            code: KeyCode::Letter(Letter::Q),
            label: "Q",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::W),
            label: "W",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::E),
            label: "E",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::R),
            label: "R",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::T),
            label: "T",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::Y),
            label: "Y",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::U),
            label: "U",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::I),
            label: "I",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::O),
            label: "O",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::P),
            label: "P",
        },
        BoardKey {
            code: KeyCode::Punctuation(Punctuation::LeftBracket),
            label: "[",
        },
        BoardKey {
            code: KeyCode::Punctuation(Punctuation::RightBracket),
            label: "]",
        },
        BoardKey {
            code: KeyCode::Punctuation(Punctuation::Backslash),
            label: "\\",
        },
    ],
    &[
        BoardKey {
            code: KeyCode::Letter(Letter::A),
            label: "A",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::S),
            label: "S",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::D),
            label: "D",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::F),
            label: "F",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::G),
            label: "G",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::H),
            label: "H",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::J),
            label: "J",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::K),
            label: "K",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::L),
            label: "L",
        },
        BoardKey {
            code: KeyCode::Punctuation(Punctuation::Semicolon),
            label: ";",
        },
        BoardKey {
            code: KeyCode::Punctuation(Punctuation::Quote),
            label: "'",
        },
    ],
    &[
        BoardKey {
            code: KeyCode::Letter(Letter::Z),
            label: "Z",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::X),
            label: "X",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::C),
            label: "C",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::V),
            label: "V",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::B),
            label: "B",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::N),
            label: "N",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::M),
            label: "M",
        },
        BoardKey {
            code: KeyCode::Punctuation(Punctuation::Comma),
            label: ",",
        },
        BoardKey {
            code: KeyCode::Punctuation(Punctuation::Period),
            label: ".",
        },
        BoardKey {
            code: KeyCode::Punctuation(Punctuation::Slash),
            label: "/",
        },
    ],
    &[
        BoardKey {
            code: KeyCode::Space,
            label: "Space",
        },
        BoardKey {
            code: KeyCode::Backspace,
            label: "Backspace",
        },
        BoardKey {
            code: KeyCode::Insert,
            label: "Ins",
        },
        BoardKey {
            code: KeyCode::Delete,
            label: "Del",
        },
        BoardKey {
            code: KeyCode::Home,
            label: "Home",
        },
        BoardKey {
            code: KeyCode::End,
            label: "End",
        },
        BoardKey {
            code: KeyCode::PageUp,
            label: "PgUp",
        },
        BoardKey {
            code: KeyCode::PageDown,
            label: "PgDn",
        },
    ],
    &[
        BoardKey {
            code: KeyCode::Up,
            label: "↑",
        },
        BoardKey {
            code: KeyCode::Left,
            label: "←",
        },
        BoardKey {
            code: KeyCode::Down,
            label: "↓",
        },
        BoardKey {
            code: KeyCode::Right,
            label: "→",
        },
    ],
    &[
        BoardKey {
            code: KeyCode::Mouse(MouseButton::Back),
            label: "Mouse4",
        },
        BoardKey {
            code: KeyCode::Mouse(MouseButton::Forward),
            label: "Mouse5",
        },
    ],
];

pub(super) static NUMPAD_ROWS: &[&[BoardKey]] = &[
    &[
        BoardKey {
            code: KeyCode::Numpad(NumpadKey::Num7),
            label: "Num7",
        },
        BoardKey {
            code: KeyCode::Numpad(NumpadKey::Num8),
            label: "Num8",
        },
        BoardKey {
            code: KeyCode::Numpad(NumpadKey::Num9),
            label: "Num9",
        },
        BoardKey {
            code: KeyCode::Numpad(NumpadKey::Divide),
            label: "Num/",
        },
    ],
    &[
        BoardKey {
            code: KeyCode::Numpad(NumpadKey::Num4),
            label: "Num4",
        },
        BoardKey {
            code: KeyCode::Numpad(NumpadKey::Num5),
            label: "Num5",
        },
        BoardKey {
            code: KeyCode::Numpad(NumpadKey::Num6),
            label: "Num6",
        },
        BoardKey {
            code: KeyCode::Numpad(NumpadKey::Multiply),
            label: "Num*",
        },
    ],
    &[
        BoardKey {
            code: KeyCode::Numpad(NumpadKey::Num1),
            label: "Num1",
        },
        BoardKey {
            code: KeyCode::Numpad(NumpadKey::Num2),
            label: "Num2",
        },
        BoardKey {
            code: KeyCode::Numpad(NumpadKey::Num3),
            label: "Num3",
        },
        BoardKey {
            code: KeyCode::Numpad(NumpadKey::Subtract),
            label: "Num-",
        },
    ],
    &[
        BoardKey {
            code: KeyCode::Numpad(NumpadKey::Num0),
            label: "Num0",
        },
        BoardKey {
            code: KeyCode::Numpad(NumpadKey::Decimal),
            label: "Num.",
        },
        BoardKey {
            code: KeyCode::Numpad(NumpadKey::Add),
            label: "Num+",
        },
    ],
];
