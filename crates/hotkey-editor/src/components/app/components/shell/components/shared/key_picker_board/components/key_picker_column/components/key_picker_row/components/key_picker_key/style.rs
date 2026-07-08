use super::state::KeyPickerKeyState;
use tw_macro::internal::{join_into, joined_len};
use tw_macro::tw;
use tw_macro::{ClassList, TailwindClass};

classes! {
    base: tw![
        "relative",
        "group/tooltip",
        "[anchor-name:--tooltip-anchor]",
        "[anchor-scope:--tooltip-anchor]",
        "min-w-0",
        "w-[6cqi]",
        "h-[7cqi]",
        "px-1",
        "flex",
        "items-center",
        "justify-center",
        "border",
        "rounded-control",
        "text-[1.7cqi]",
        "leading-none",
        "cursor-pointer",
        "whitespace-nowrap",
        "transition-[border-color,background,box-shadow]", "duration-fast",
        "text-shadow-outline",
        "focus:outline-none",
        "kb-focus:outline-none",
        "kb-focus:border-white",
        "kb-focus:text-white",
        "kb-focus:[--focus-color:var(--color-warcraft-highlight)]", "kb-focus:shadow-focus",
        "data-[wide=true]:w-[12cqi]",
        "disabled:cursor-not-allowed",
        "disabled:opacity-85",
    ],
    mobile: tw![
        "mobile:w-[7cqi]",
        "mobile:h-[8.5cqi]",
        "mobile:p-0",
        "mobile:text-[1.6cqi]",
        "mobile:data-[wide=true]:w-[14cqi]",
    ],
}

// Each key state is a plain `match` arm that joins one overlay onto the base
// `CLASS_STR`, never a `states!` table: the state is one look parameterised by the
// key's domain cell state (`Available` xor `Current` xor `Conflict`), so it is a
// data→token function exactly like `race_theme`'s `theme(race)`.
macro_rules! key_picker_key_class {
    ($($utility:literal),+ $(,)?) => {{
        const OVERLAY: &[TailwindClass] = tw![$($utility),+];
        const LEN: usize = joined_len(CLASS_STR, &[OVERLAY]);
        const BYTES: [u8; LEN] = join_into::<LEN>(CLASS_STR, &[OVERLAY]);
        const CLASS: ClassList = ClassList::new(match ::core::str::from_utf8(&BYTES) {
            ::core::result::Result::Ok(class) => class,
            ::core::result::Result::Err(_) => ::core::panic!("non-utf8 key picker key class"),
        });
        CLASS
    }};
}

pub(super) fn class(state: KeyPickerKeyState) -> ClassList {
    match state {
        KeyPickerKeyState::Available => key_picker_key_class![
            "[background:color-mix(in_oklab,var(--color-warcraft-gold-dark)_55%,transparent)]",
            "border-warcraft-gold-border",
            "text-warcraft-gold",
            "[&:hover:not(:disabled)]:border-warcraft-gold",
            "[&:hover:not(:disabled)]:[background:color-mix(in_oklab,var(--color-warcraft-gold)_12%,transparent)]",
            "[&:hover:not(:disabled)]:shadow-glow-soft",
        ],
        KeyPickerKeyState::Current => key_picker_key_class![
            "bg-panel-gold",
            "border-warcraft-gold",
            "text-warcraft-gold",
            "shadow-glow",
        ],
        KeyPickerKeyState::Conflict => key_picker_key_class![
            "[background:color-mix(in_oklab,var(--color-race-orc-strong)_50%,transparent)]",
            "border-race-orc-strong",
            "text-race-orc",
            "[&:hover:not(:disabled)]:border-warcraft-danger",
            "[&:hover:not(:disabled)]:[background:color-mix(in_oklab,var(--color-race-orc-strong)_55%,transparent)]",
            "[--glow-color:var(--color-warcraft-danger)]",
            "[&:hover:not(:disabled)]:shadow-glow-soft",
        ],
    }
}
