use tw_macro::tw;

classes! {
    base: tw![
        "fixed",
        "top-0",
        "right-0",
        "h-dvh",
        "max-h-dvh",
        "z-71",
        "w-[min(74vw,280px)]",
        // bg-panel-dark is an 85-90% gradient and needs an opaque colour under it.
        // A drawer must not show the page through itself.
        "bg-warcraft-bg-base",
        "bg-panel-dark",
        "border-l",
        "border-l-warcraft-gold/30",
        "shadow-drawer",
        "flex",
        "flex-col",
        "starting:translate-x-full",
        "transition-transform",
        "duration-slow",
        "ease-[cubic-bezier(0.16,1,0.3,1)]",
    ],
}
