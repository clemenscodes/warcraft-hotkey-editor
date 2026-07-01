use crate::classes;

// The override panel card: the gold-edged column that holds the header and the
// ability sections. Content-sized on desktop; a fixed-height scrollless block on the
// mobile panel. Class `.tile-override-card` is load-bearing (a scroll-into-view
// effect queries it).
const BASE: &[&str] = &[
    "flex",
    "flex-col",
    "items-stretch",
    "flex-[0_0_auto]",
    "gap-5",
    "p-[2rem_2.25rem]",
    "overflow-hidden",
    "border",
    "border-warcraft-gold",
    "rounded-[10px]",
    "bg-[linear-gradient(135deg,rgba(40,30,8,0.55)_0%,rgba(15,12,4,0.55)_100%)]",
    "shadow-[0_0_12px_rgba(255,206,99,0.18)]",
];

const MOBILE: &[&str] = &[
    "mobile:w-full",
    "mobile:max-w-full",
    "mobile:min-w-0",
    "mobile:box-border",
    "mobile:flex-nowrap",
    "mobile:justify-start",
    "mobile:gap-[6px_10px]",
    "mobile:p-[10px_12px]",
    "mobile:h-[300px]",
];

const TABLET: &[&str] = &[
    "tablet:w-full",
    "tablet:max-w-full",
    "tablet:min-w-0",
    "tablet:box-border",
    "tablet:flex-nowrap",
    "tablet:justify-start",
    "tablet:gap-[6px_10px]",
    "tablet:p-[10px_12px]",
    "tablet:h-[300px]",
];

const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
