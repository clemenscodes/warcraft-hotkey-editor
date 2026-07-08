use tw_macro::tw;

// The shared badge chrome written out in full, plus this tone's own muted colour triple.
// Sibling tones share these *values*, never a look.

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
        "bg-warcraft-bg-mid",
        "border-warcraft-text-faint",
        "text-warcraft-text-secondary",
    ],
}
