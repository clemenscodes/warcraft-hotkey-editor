use tw_macro::tw;
// The footer is the app's bottom chrome — the full-bleed mirror of the header at the other
// end of the shell, and, like the header, a query container (`@container`) any `cqi` length
// beneath it could resolve against. It is fine print, so its size barely changes across the
// whole width range: one small type step (`text-xs`) is the footer's single knob, and every
// glyph, icon, and horizontal gap below expresses its length in `em`, so the whole footer
// scales as one drawing off that one font size.
//
// The vertical rhythm is one spacing step, used for both `py` (top/bottom) and `gap-y`
// (between the three rows — credit, links, disclaimer). Equal `py` and `gap-y` put the rows
// in four equal vertical spaces: the top margin matches the bottom margin and the rows are
// evenly distributed rather than bunched at the centre. `gap-y` is the single source of that
// row spacing (the disclaimer carries no margin of its own); the `clamp` ceiling keeps the bar
// from ballooning in height on 4K, exactly as the header's bar-height `clamp` does.
//
// There are no per-band overrides: the whole footer lives in BASE. It needs no safe-area insets
// because the shell drops `viewport-fit=cover`, so the browser keeps the app clear of device
// edges and every band renders the same. `mt-auto` pins the bar to the bottom of the shell
// column when a short view leaves free space, and is a no-op when the view already fills it.
//
// The gold hairline along the top is the footer's own `::before`, the exact mirror of the
// header's `::after` bottom divider: same `bg-warcraft-gold/40`, same `left-4/right-4` inset
// to the `px-4` edge, same double-shadow bevel — so the two shell bars frame the content with
// matching golden edges. `relative` anchors the pseudo to the footer.

classes! {
    base: tw![
        "@container",
        "relative",
        "flex-none",
        "flex",
        "flex-wrap",
        "items-center",
        "justify-center",
        "tracking-wide",
        "select-none",
        "mt-auto",
        "px-4",
        "gap-x-[0.9em]",
        "gap-y-2.5",
        "py-2.5",
        "leading-heading",
        "text-center",
        "text-white/60",
        "text-xs",
        "before:content-['']",
        "before:absolute",
        "before:top-0",
        "before:left-4",
        "before:right-4",
        "before:h-px",
        "before:bg-warcraft-gold/40",
        "before:shadow-edge",
    ],
}
