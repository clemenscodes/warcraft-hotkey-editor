use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "items-start",
        "gap-5",
        "px-8",
        "py-6",
        "rounded-container",
        "border-2",
        "text-warcraft-text-primary",
        "cursor-pointer",
        "outline-none",
        "bg-panel-toast",
        "transition-all",
        "duration-slow",
        "ease-[cubic-bezier(0.2,0.9,0.3,1)]",
        "starting:opacity-0",
        "starting:translate-x-8",
        "starting:scale-95",
        "kb-focus:[--focus-color:var(--color-warcraft-highlight)]",
        "kb-focus:shadow-toast-focus",
        "border-race-orc",
        "[--glow-color:var(--color-warcraft-danger)]",
        "shadow-toast",
    ],
    mobile: tw![
        "mobile:max-w-[calc(100vw-1.5rem)]",
    ],
    tablet: tw![
        "tablet:max-w-[calc(100vw-1.5rem)]",
    ],
}
