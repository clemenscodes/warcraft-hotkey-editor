use tw_macro::tw;

// The one action button every dialog footer shows, the same width whatever
// its label, so switching dialogs never changes it.
classes! {
    base: tw![
        "inline-flex",
        "items-center",
        "justify-center",
        "w-[22em]",
        "min-h-12",
        "px-7",
        "py-3",
        "border",
        "border-warcraft-gold",
        "rounded-panel",
        "text-warcraft-gold",
        "text-lg",
        "tracking-caps",
        "uppercase",
        "cursor-pointer",
        "bg-panel-gold-resting",
        "shadow-glow",
        "transition-[background,box-shadow]",
        "duration-fast",
        "focus:outline-none",
        "kb-focus:border-white",
        "kb-focus:text-white",
        "kb-focus:[--focus-color:var(--color-warcraft-highlight)]",
        "kb-focus:shadow-focus",
        "hover:bg-panel-gold",
        "hover:shadow-glow-strong",
    ],
    mobile: tw![
        "mobile:px-4",
        "mobile:py-2",
        "mobile:text-sm",
    ],
    tablet: tw![
        "tablet:px-4",
        "tablet:py-2",
        "tablet:text-md",
    ],
    laptop: tw![
        "laptop:px-4",
        "laptop:py-2",
        "laptop:text-md",
    ],
    desktop: tw![
        "desktop:px-4",
        "desktop:py-2",
        "desktop:text-md",
    ],
    qhd: tw![
        "qhd:min-h-16.75",
        "qhd:px-5.5",
        "qhd:py-2.75",
        "qhd:text-xl",
    ],
    uhd: tw![
        "uhd:min-h-20.5",
        "uhd:px-6.75",
        "uhd:py-3.5",
        "uhd:text-2xl",
    ],
}
