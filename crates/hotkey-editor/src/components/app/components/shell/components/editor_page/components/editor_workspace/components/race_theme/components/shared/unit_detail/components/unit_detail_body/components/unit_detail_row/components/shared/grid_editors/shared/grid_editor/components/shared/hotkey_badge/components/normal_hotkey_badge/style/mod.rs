use tw_macro::tw;

// The shared badge chrome (layout, sizing, border weight, type, shadow) written out in
// full, plus this tone's own colour triple. Sibling tones share these *values*, never a
// look: each writes its own list, so editing one never reaches the others.

classes! {
    base: tw![
        "inline-flex",
        "items-center",
        "justify-center",
        "min-w-[24cqi]",
        "h-[24cqi]",
        "px-[5cqi]",
        "rounded-[5cqi]",
        "border",
        "text-[17cqi]/[1]",
        "font-normal",
        "pointer-events-none",
        "text-shadow-drop",
        "bg-warcraft-shadow/78",
        "border-warcraft-gold/55",
        "text-warcraft-gold",
    ],
}
