use tw_macro::tw;
// The active category tab: shared chrome values on the blue active surface, wearing the
// race accent border/text/glow read from the theme's `--race-accent`. Shared values
// with the idle sibling.
classes! {
    base: tw![
        "flex-1","min-w-0","min-h-11","px-2","border","rounded-card","text-sm","tracking-label","uppercase","text-center","cursor-pointer","transition-all","duration-fast","whitespace-nowrap","overflow-hidden","text-ellipsis","focus:outline-none","kb-focus:border-white","kb-focus:[--focus-color:var(--color-warcraft-highlight)]","kb-focus:shadow-focus",
        "bg-panel-blue",
        "border-[var(--race-accent)]",
        "text-[var(--race-accent)]",
        "[--glow-color:var(--race-accent)]",
        "shadow-glow-soft",
    ],
    mobile: tw!["mobile:text-xs","mobile:px-1.5","mobile:h-11","mobile:leading-none"], tablet: tw!["tablet:text-xs","tablet:px-1.5","tablet:h-11","tablet:leading-none"],
}
