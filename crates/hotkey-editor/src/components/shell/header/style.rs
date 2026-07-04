use crate::{classes, styling::TailwindClass, tw};

// The bar's own layout scales with the viewport. On laptop and up it is a three-column
// grid (brand | centered layout button | toolbar) whose column gap and symmetric vertical
// padding are expressed in `vw`, so the whole bar grows coherently from laptop through 4K
// and `items-center` centers every child in the bar (the padding is equal top and bottom,
// never bottom-only, or the row floats off-center). The children (brand, layout button,
// toolbar buttons) carry their own `cqi` scaling off the boxes this grid hands them.
// Below laptop it collapses to a flex row (brand left, toolbar right) sized for touch.
//
// The horizontal inset is one always-on `px-4`, so the bar's content is indented by the
// same amount on every band. Padding never clips an element's background, so the bar's
// fill (its own gradient below laptop, the app's fixed backdrop above) stays edge-to-edge
// while only the content is inset. Vertical padding stays band-specific: safe-area top +
// `pb-2` below laptop, symmetric `vw` on laptop and up.
//
// The gold divider that separates the bar from the app is the header's own `::after`, not
// a full-width `border-b`: it is absolutely positioned along the bottom and inset
// `left-4 right-4` to the same `px-4` edge as the content, so the line ends exactly where
// the content ends while the bar's fill still spans edge-to-edge. The double drop-shadow
// under it is the original bevel, now riding the inset line.
const BASE: &[TailwindClass] = tw![
    "relative",
    "z-50",
    "items-center",
    "flex-none",
    "px-4",
    "after:content-['']",
    "after:absolute",
    "after:bottom-0",
    "after:left-4",
    "after:right-4",
    "after:h-px",
    "after:bg-[rgba(255,206,99,0.4)]",
    "after:[box-shadow:0_1px_0_rgba(0,0,0,0.7),0_2px_0_rgba(255,206,99,0.1)]",
];

const MOBILE: &[TailwindClass] = tw![
    "mobile:flex",
    "mobile:flex-row",
    "mobile:justify-between",
    "mobile:sticky",
    "mobile:top-0",
    "mobile:z-[60]",
    "mobile:[padding-top:max(0.5rem,env(safe-area-inset-top))]",
    "mobile:pb-2",
    "mobile:after:bg-[rgba(255,206,99,0.3)]",
    "mobile:min-h-14",
    "mobile:max-w-full",
    "mobile:w-full",
    "mobile:[background-color:#050a1a]",
    "mobile:[background-image:radial-gradient(ellipse_90%_60%_at_50%_0%,#18365b_0%,transparent_60%),linear-gradient(180deg,#0a1a35_0%,#050a1a_100%)]",
    "mobile:bg-no-repeat",
    "mobile:[background-attachment:fixed]",
    "mobile:[background-size:100%_100%]",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:flex",
    "tablet:flex-row",
    "tablet:justify-between",
    "tablet:sticky",
    "tablet:top-0",
    "tablet:z-[60]",
    "tablet:[padding-top:max(0.5rem,env(safe-area-inset-top))]",
    "tablet:pb-2",
    "tablet:after:bg-[rgba(255,206,99,0.3)]",
    "tablet:min-h-14",
    "tablet:max-w-full",
    "tablet:w-full",
    "tablet:[background-color:#050a1a]",
    "tablet:[background-image:radial-gradient(ellipse_90%_60%_at_50%_0%,#18365b_0%,transparent_60%),linear-gradient(180deg,#0a1a35_0%,#050a1a_100%)]",
    "tablet:bg-no-repeat",
    "tablet:[background-attachment:fixed]",
    "tablet:[background-size:100%_100%]",
];

// From laptop up the bar is both a container-query context (`container-type:inline-size`,
// so its children size in `cqi` off the bar, not the raw viewport) and its own single
// height knob: a `vw`-scaled `min-height` with a `4rem` floor (above the mobile/tablet
// `min-h-14`) and an `8.5rem` ceiling, growing generously with the viewport through 4K.
// `items-stretch` hands that same row height to every column, so the layout button and
// the toolbar buttons fill it and render at one shared height. `py-[0.7vw]` then insets
// that fill so the buttons sit shorter than the bar with breathing room above and below —
// this padding, not any button, is the one knob for the button-to-bar height ratio.
// Nothing inside carries a fixed size — change the `min-height` (bar height) or the `py`
// (button ratio) and the whole bar rescales together.
const LAPTOP: &[TailwindClass] = tw![
    "laptop:grid",
    "laptop:[grid-template-columns:minmax(0,1fr)_auto_minmax(0,1fr)]",
    "laptop:items-stretch",
    "laptop:[container-type:inline-size]",
    "laptop:min-h-[clamp(4rem,4.2vw,8.5rem)]",
    "laptop:gap-[1vw]",
    "laptop:py-[0.7vw]",
];

const DESKTOP: &[TailwindClass] = tw![
    "desktop:grid",
    "desktop:[grid-template-columns:minmax(0,1fr)_auto_minmax(0,1fr)]",
    "desktop:items-stretch",
    "desktop:[container-type:inline-size]",
    "desktop:min-h-[clamp(4rem,4.2vw,8.5rem)]",
    "desktop:gap-[1vw]",
    "desktop:py-[0.7vw]",
];

const QHD: &[TailwindClass] = tw![
    "qhd:grid",
    "qhd:[grid-template-columns:minmax(0,1fr)_auto_minmax(0,1fr)]",
    "qhd:items-stretch",
    "qhd:[container-type:inline-size]",
    "qhd:min-h-[clamp(4rem,4.2vw,8.5rem)]",
    "qhd:gap-[1vw]",
    "qhd:py-[0.7vw]",
];

const UHD: &[TailwindClass] = tw![
    "uhd:grid",
    "uhd:[grid-template-columns:minmax(0,1fr)_auto_minmax(0,1fr)]",
    "uhd:items-stretch",
    "uhd:[container-type:inline-size]",
    "uhd:min-h-[clamp(4rem,4.2vw,8.5rem)]",
    "uhd:gap-[1vw]",
    "uhd:py-[0.7vw]",
];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
