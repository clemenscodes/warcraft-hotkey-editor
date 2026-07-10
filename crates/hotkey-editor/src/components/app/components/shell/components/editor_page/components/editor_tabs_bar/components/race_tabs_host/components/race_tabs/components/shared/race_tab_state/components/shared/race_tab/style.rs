use tw_macro::tw;

// The base race tab's entire look, owned here and nowhere else: this `mod style` is
// private and never re-exported, so no other component can name or reuse these classes.
// A richer look (the active tab) does not extend this — it renders `RaceTab` and adds an
// accent on top. The banner surface, image, accent, and label colour are read from the
// `--race-color`/`--banner-*`/`--race-accent`/`--label-color` custom properties the
// per-race wrapper and the active variant publish, so this base stays race- and
// state-agnostic. The accent border and glow appear only on hover (the resting,
// inactive look); the white label is the `--label-color` default.
classes! {
    base: tw![
        "group",
        "relative",
        "size-full",
        "p-0",
        "border",
        "border-warcraft-blue-deep",
        "rounded-card",
        "bg-race-banner",
        "text-warcraft-text-primary",
        "text-xl",
        "uppercase",
        "tracking-caps",
        "text-center",
        "transition-[border-color,box-shadow,transform]",
        "duration-base",
        "overflow-hidden",
        "isolate",
        "flex",
        "items-end",
        "justify-center",
        "[text-shadow:1px_1px_0_var(--color-warcraft-shadow),-1px_1px_0_var(--color-warcraft-shadow),1px_-1px_0_var(--color-warcraft-shadow),-1px_-1px_0_var(--color-warcraft-shadow),0_0_8px_color-mix(in_oklab,var(--color-warcraft-shadow)_85%,transparent)]",
        "before:content-['']",
        "before:absolute",
        "before:inset-0",
        "before:[background-image:var(--banner-image)]",
        "before:bg-contain",
        "before:bg-no-repeat",
        "before:bg-center",
        "before:brightness-150",
        "before:saturate-125",
        "before:z-0",
        "before:pointer-events-none",
        "before:block",
        "after:content-['']",
        "after:absolute",
        "after:inset-0",
        "after:bg-[linear-gradient(180deg,color-mix(in_oklab,var(--color-warcraft-shadow)_0%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-shadow)_0%,transparent)_45%,color-mix(in_oklab,var(--color-warcraft-shadow)_55%,transparent)_75%,color-mix(in_oklab,var(--color-warcraft-shadow)_85%,transparent)_100%)]",
        "after:z-1",
        "after:pointer-events-none",
        "after:block",
        "hover:text-white",
        "hover:border-(--race-accent)",
        "hover:[--glow-color:var(--race-accent)]",
        "hover:shadow-glow",
        "focus:outline-none",
        "kb-focus:outline-none",
        "kb-focus:text-white",
        "kb-focus:border-white",
        "kb-focus:[--focus-color:var(--color-warcraft-highlight)]",
        "kb-focus:shadow-focus",
    ],
    mobile: tw![
        "mobile:text-sm",
        "mobile:before:brightness-[1.35]",
        "mobile:before:saturate-[1.2]",
    ],
    tablet: tw![
        "tablet:text-sm",
        "tablet:before:brightness-[1.35]",
        "tablet:before:saturate-[1.2]",
    ],
}
