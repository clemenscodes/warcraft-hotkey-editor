use tw_macro::tw;

classes! {
    base: tw![
        "relative",
        "flex",
        "size-full",
        "min-w-0",
        "has-[.active-accent:not([aria-hidden=true])]:[--label-color:var(--race-color)]",
    ],
}
