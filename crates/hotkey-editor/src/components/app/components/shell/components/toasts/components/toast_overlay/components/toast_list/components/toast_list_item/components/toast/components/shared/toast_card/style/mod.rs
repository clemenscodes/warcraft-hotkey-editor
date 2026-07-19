use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "items-start",
        "gap-[1.25em]",
        "px-[2em]",
        "py-[1.5em]",
        "rounded-container",
        "border-2",
        "border-(--toast-accent)",
        "text-base",
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
        "shadow-toast",
    ],
    mobile: tw![
        "mobile:max-w-[calc(100vw-1.5rem)]",
        "mobile:gap-[0.9em]",
        "mobile:px-[1.25em]",
        "mobile:py-[1em]",
        "mobile:border-0",
        "mobile:text-sm",
    ],
    tablet: tw![
        "tablet:max-w-[calc(100vw-1.5rem)]",
    ],
}
