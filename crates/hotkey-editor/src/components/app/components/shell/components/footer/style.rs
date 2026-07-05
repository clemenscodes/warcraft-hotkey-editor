use crate::{classes, styling::TailwindClass, tw};

// The footer is the app's bottom chrome — the full-bleed mirror of the header at the other
// end of the shell, and, like the header, a query container (`@container`) any `cqi` length
// beneath it could resolve against. It is fine print, so its size barely changes across the
// whole width range: one `text-[clamp(…)]` — a rem floor so it stays legible on a phone, a
// rem ceiling so it never balloons on 4K, a gentle `vw` between — carries every band. That
// single font size is the footer's one knob: every glyph, icon, and horizontal gap below
// expresses its length in `em`, so the whole footer scales as one drawing off it.
//
// The vertical rhythm is one `clamp`ed `vw`, used for both `py` (top/bottom) and `gap-y`
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
const BASE: &[TailwindClass] = tw![
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
    "gap-y-[clamp(0.4rem,0.85vw,0.8rem)]",
    "py-[clamp(0.4rem,0.85vw,0.8rem)]",
    "leading-[1.3]",
    "text-center",
    "text-white/60",
    "text-[clamp(0.7rem,0.8vw,0.9rem)]",
    "before:content-['']",
    "before:absolute",
    "before:top-0",
    "before:left-4",
    "before:right-4",
    "before:h-px",
    "before:bg-warcraft-gold/40",
    "before:[box-shadow:0_1px_0_color-mix(in_oklab,var(--color-warcraft-shadow)_70%,transparent),0_2px_0_color-mix(in_oklab,var(--color-warcraft-gold)_10%,transparent)]",
];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
