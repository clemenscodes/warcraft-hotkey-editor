use super::state::SystemKeyPickerKeyState;
use tw_macro::tw;

classes! {
    base: tw![
        "relative",
        "group/tooltip",
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
    ],
    mobile: tw![
        "mobile:w-[7cqi]",
        "mobile:h-[8.5cqi]",
        "mobile:p-0",
        "mobile:text-[1.6cqi]",
        "mobile:data-[wide=true]:w-[14cqi]",
    ],
}

states! {
    SystemKeyPickerKeyState,
    Normal => tw![
        "[background:color-mix(in_oklab,var(--color-warcraft-gold-dark)_55%,transparent)]",
        "border-warcraft-gold-border",
        "text-warcraft-gold",
        "[&:hover]:border-warcraft-gold",
        "[&:hover]:[background:color-mix(in_oklab,var(--color-warcraft-gold)_12%,transparent)]",
        "[&:hover]:shadow-glow-soft",
    ],
    Current => tw![
        "bg-panel-gold",
        "border-warcraft-gold",
        "text-warcraft-gold",
        "shadow-glow",
    ],
    Conflict => tw![
        "[background:color-mix(in_oklab,var(--color-race-orc-strong)_50%,transparent)]",
        "border-race-orc-strong",
        "text-race-orc",
        "[&:hover]:border-warcraft-danger",
        "[&:hover]:[background:color-mix(in_oklab,var(--color-race-orc-strong)_55%,transparent)]",
        "[--glow-color:var(--color-warcraft-danger)]", "[&:hover]:shadow-glow-soft",
    ],
}
