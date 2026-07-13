use tw_macro::tw;
// Fills its host container and draws its structure as a cqi-scaled drawing: it takes the
// host's full box (`size-full`) — the host owns the `aspect-[39/10]` shape and gets its
// size from the header — and expresses every interior *layout* length — padding, gap,
// border, radius, font — as a `cqi` fraction of the host box, so making the host taller
// scales the button up in proportion. The gold glow and focus ring are the shared design
// tokens (`shadow-glow-soft` at rest, `shadow-glow-strong` on hover, `shadow-focus` on
// focus) that every gold button wears — a glow is shared vocabulary, not a per-button
// reinvention. The header sizes the host off the shared row height.

classes! {
    base: tw![
        "inline-flex",
        "items-center",
        "justify-center",
        "gap-[4.27cqi]",
        "size-full",
        "px-[8.55cqi]",
        "border-[0.3cqi]",
        "border-warcraft-gold",
        "rounded-[3.2cqi]",
        "text-warcraft-gold",
        "text-[8.55cqi]",
        "tracking-eyebrow",
        "font-medium",
        "cursor-pointer",
        "bg-panel-gold-resting",
        "shadow-glow-soft",
        "transition-[background,box-shadow,transform]",
        "duration-fast",
        "focus:outline-none",
        "kb-focus:border-white",
        "kb-focus:text-white",
        "kb-focus:[--focus-color:var(--color-warcraft-highlight)]",
        "kb-focus:shadow-focus",
        "hover:bg-panel-gold-active",
        "hover:shadow-glow-strong",
    ],
}
