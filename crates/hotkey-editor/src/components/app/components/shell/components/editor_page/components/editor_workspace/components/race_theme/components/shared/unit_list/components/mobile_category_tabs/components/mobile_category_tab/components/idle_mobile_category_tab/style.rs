use tw_macro::tw;
// The idle category tab: shared chrome values, muted, taking the race accent border on
// hover (generic `var(--race-accent)` from the theme container — no per-race arm, no
// join macro). Shared values with the active sibling.
classes! {
    base: tw![
        "flex-1","min-w-0","min-h-11","px-2","border","rounded-card","text-sm","tracking-label","uppercase","text-center","cursor-pointer","transition-all","duration-fast","whitespace-nowrap","overflow-hidden","text-ellipsis","focus:outline-none","kb-focus:border-white","kb-focus:[--focus-color:var(--color-warcraft-highlight)]","kb-focus:shadow-focus",
        "bg-warcraft-bg-mid/55",
        "border-warcraft-blue-deep",
        "text-warcraft-text-secondary",
        "hover:bg-warcraft-blue-deep/70",
        "hover:text-white",
        "hover:border-[var(--race-accent)]",
    ],
    mobile: tw!["mobile:text-xs","mobile:px-1.5","mobile:h-11","mobile:leading-none"], tablet: tw!["tablet:text-xs","tablet:px-1.5","tablet:h-11","tablet:leading-none"],
}
