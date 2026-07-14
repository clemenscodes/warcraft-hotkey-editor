use tw_macro::tw;
// The drawer must NOT create a containing block: it hosts each dialog-owning row's `fixed`
// dialog, which must center on the viewport, not on this panel. A transform (`translate`) at
// rest would trap those dialogs, so the resting state carries no transform — it slides in from
// `starting:translate-x-full` down to `translate: none`, leaving no containing block once open.

classes! {
    base: tw![
        "fixed",
        "top-0",
        "right-0",
        "h-dvh",
        "max-h-dvh",
        "z-71",
        "w-[min(74vw,280px)]",
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
