use crate::{classes, styling::TailwindClass, tw};

// The footer is the app's bottom chrome — the full-bleed mirror of the header at the other
// end of the shell, and, like the header, the query container (`@container`) every `cqi`
// length beneath it could resolve against. Like the header bar it owns its own defining
// dimension in `vw`: here that dimension is the font size, `clamp`ed with a floor so the fine
// print never shrinks to nothing on a small laptop and a ceiling so it never balloons on 4K —
// exactly the bar-height `clamp` the header uses, one axis over. That single font size is the
// footer's one knob: every glyph, icon, horizontal gap, and the disclaimer below expresses its
// length in `em`, so the whole footer scales as one drawing off it. Change the `clamp` and the
// credit, heart, link icons, separators, and disclaimer all rescale together.
//
// The bar owns its vertical rhythm with the same `clamp`ed `vw` as its height would take: `py`
// (top/bottom) equals `gap-y` (between the three rows — credit, links, disclaimer), so the rows
// sit in four equal vertical spaces — the top margin matches the bottom margin and the rows are
// evenly distributed rather than bunched at the centre. `gap-y` is the single source of that row
// spacing (the disclaimer carries no margin of its own). The `clamp` ceiling is what keeps the
// bar from ballooning in height on 4K, exactly as the header's bar-height `clamp` does.
//
// BASE is the laptop-and-up truth. The two touch bands override the font ramp for their
// narrower widths and swap the symmetric `vw` padding for safe-area insets, so the bar clears
// a notch on the sides and a home indicator at the bottom while staying centered and wrapping.
const BASE: &[TailwindClass] = tw![
    "@container",
    "flex-none",
    "flex",
    "flex-wrap",
    "items-center",
    "justify-center",
    "gap-x-[0.9em]",
    "gap-y-[clamp(0.4rem,0.85vw,0.8rem)]",
    "px-4",
    "py-[clamp(0.4rem,0.85vw,0.8rem)]",
    "text-[clamp(0.65rem,0.78vw,0.88rem)]",
    "tracking-wide",
    "text-white/60",
    "select-none",
];
const MOBILE: &[TailwindClass] = tw![
    "mobile:px-[max(0.5rem,env(safe-area-inset-left))]",
    "mobile:pt-2",
    "mobile:pb-[max(0.5rem,env(safe-area-inset-bottom))]",
    "mobile:text-center",
    "mobile:leading-[1.3]",
    "mobile:text-[clamp(0.62rem,2.9vw,0.85rem)]",
];
const TABLET: &[TailwindClass] = tw![
    "tablet:px-[max(0.5rem,env(safe-area-inset-left))]",
    "tablet:pt-2",
    "tablet:pb-[max(0.5rem,env(safe-area-inset-bottom))]",
    "tablet:text-center",
    "tablet:leading-[1.3]",
    "tablet:text-[clamp(0.7rem,1.25vw,1.05rem)]",
    "tablet:mt-auto",
];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
