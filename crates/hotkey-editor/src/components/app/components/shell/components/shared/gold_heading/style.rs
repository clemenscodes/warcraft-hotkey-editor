use super::state::GoldHeadingVariant;
use tw_macro::tw;

classes! {
    base: tw![
        "uppercase",
        "tracking-heading",
    ],
}

states! {
    GoldHeadingVariant,
    Section => tw![
        "text-warcraft-gold",
        "text-shadow-drop",
    ],
    Dialog => tw![
        "text-warcraft-gold",
        "[text-shadow:1px_1px_0_var(--color-warcraft-shadow),0_0_18px_color-mix(in_oklab,var(--color-warcraft-gold)_35%,transparent)]",
    ],
    Grid => tw![
        "font-normal",
        "text-warcraft-gold",
        "text-shadow-drop",
    ],
    Toast => tw![
        "text-shadow-drop",
    ],
}
