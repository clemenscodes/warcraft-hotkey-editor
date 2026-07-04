use crate::classes;

// The bar's own layout scales with the viewport. On laptop and up it is a three-column
// grid (brand | centered layout button | toolbar) whose column gap and symmetric vertical
// padding are expressed in `vw`, so the whole bar grows coherently from laptop through 4K
// and `items-center` centers every child in the bar (the padding is equal top and bottom,
// never bottom-only, or the row floats off-center). The children (brand, layout button,
// toolbar buttons) carry their own `cqi` scaling off the boxes this grid hands them.
// Below laptop it collapses to a flex row (brand left, toolbar right) sized for touch.
const BASE: &[&str] = &[
    "relative",
    "z-50",
    "items-center",
    "flex-none",
    "border-b",
    "border-b-[rgba(255,206,99,0.4)]",
    "[box-shadow:0_1px_0_rgba(0,0,0,0.7),0_2px_0_rgba(255,206,99,0.1)]",
];

const MOBILE: &[&str] = &[
    "mobile:flex",
    "mobile:flex-row",
    "mobile:justify-between",
    "mobile:sticky",
    "mobile:top-0",
    "mobile:z-[60]",
    "mobile:[padding-top:max(0.5rem,env(safe-area-inset-top))]",
    "mobile:pb-2",
    "mobile:pl-2",
    "mobile:pr-2",
    "mobile:border-b-[rgba(255,206,99,0.3)]",
    "mobile:min-h-14",
    "mobile:max-w-full",
    "mobile:w-full",
    "mobile:[background-color:#050a1a]",
    "mobile:[background-image:radial-gradient(ellipse_90%_60%_at_50%_0%,#18365b_0%,transparent_60%),linear-gradient(180deg,#0a1a35_0%,#050a1a_100%)]",
    "mobile:bg-no-repeat",
    "mobile:[background-attachment:fixed]",
    "mobile:[background-size:100%_100%]",
];

const TABLET: &[&str] = &[
    "tablet:flex",
    "tablet:flex-row",
    "tablet:justify-between",
    "tablet:sticky",
    "tablet:top-0",
    "tablet:z-[60]",
    "tablet:[padding-top:max(0.5rem,env(safe-area-inset-top))]",
    "tablet:pb-2",
    "tablet:pl-2",
    "tablet:pr-2",
    "tablet:border-b-[rgba(255,206,99,0.3)]",
    "tablet:min-h-14",
    "tablet:max-w-full",
    "tablet:w-full",
    "tablet:[background-color:#050a1a]",
    "tablet:[background-image:radial-gradient(ellipse_90%_60%_at_50%_0%,#18365b_0%,transparent_60%),linear-gradient(180deg,#0a1a35_0%,#050a1a_100%)]",
    "tablet:bg-no-repeat",
    "tablet:[background-attachment:fixed]",
    "tablet:[background-size:100%_100%]",
];

const LAPTOP: &[&str] = &[
    "laptop:grid",
    "laptop:[grid-template-columns:minmax(0,1fr)_auto_minmax(0,1fr)]",
    "laptop:gap-[1vw]",
    "laptop:p-0",
    "laptop:py-[0.33vw]",
];

const DESKTOP: &[&str] = &[
    "desktop:grid",
    "desktop:[grid-template-columns:minmax(0,1fr)_auto_minmax(0,1fr)]",
    "desktop:gap-[1vw]",
    "desktop:p-0",
    "desktop:py-[0.33vw]",
];

const QHD: &[&str] = &[
    "qhd:grid",
    "qhd:[grid-template-columns:minmax(0,1fr)_auto_minmax(0,1fr)]",
    "qhd:gap-[1vw]",
    "qhd:p-0",
    "qhd:py-[0.33vw]",
];

const UHD: &[&str] = &[
    "uhd:grid",
    "uhd:[grid-template-columns:minmax(0,1fr)_auto_minmax(0,1fr)]",
    "uhd:gap-[1vw]",
    "uhd:p-0",
    "uhd:py-[0.33vw]",
];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
