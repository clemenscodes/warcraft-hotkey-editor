use crate::components::app::components::shell::components::shared::icons::{
    ICON_ARROW_DOWN, ICON_ARROW_LEFT, ICON_ARROW_RIGHT, ICON_ARROW_UP,
};

/// How a key's label is drawn. The arrow keys are glyphs the bundled font does
/// not have, so they are drawn as icons, named for screen readers; every other
/// label is plain text in the font.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(super) enum KeyCapGlyph {
    Icon {
        icon: &'static str,
        name: &'static str,
    },
    Text,
}

impl From<&str> for KeyCapGlyph {
    fn from(label: &str) -> Self {
        match label {
            "↑" => Self::Icon {
                icon: ICON_ARROW_UP,
                name: "Up arrow",
            },
            "←" => Self::Icon {
                icon: ICON_ARROW_LEFT,
                name: "Left arrow",
            },
            "↓" => Self::Icon {
                icon: ICON_ARROW_DOWN,
                name: "Down arrow",
            },
            "→" => Self::Icon {
                icon: ICON_ARROW_RIGHT,
                name: "Right arrow",
            },
            _ => Self::Text,
        }
    }
}
