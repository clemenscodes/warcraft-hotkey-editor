use crate::classes;

// The category bar under the dialog header: a centred wrap of tabs on desktop; on
// small viewports it collapses to a single-row dropdown trigger + popover menu.
const BASE: &[&str] = &[
    "flex",
    "items-baseline",
    "justify-center",
    "flex-wrap",
    "gap-3",
    "flex-none",
    "px-8",
    "py-5",
    "[border-bottom:1px_solid_rgba(255,206,99,0.25)]",
];
const MOBILE: &[&str] = &[
    "mobile:relative",
    "mobile:flex-nowrap",
    "mobile:justify-stretch",
    "mobile:px-3",
    "mobile:py-2",
    "mobile:gap-0",
    "mobile:overflow-visible",
];
const TABLET: &[&str] = &[
    "tablet:relative",
    "tablet:flex-nowrap",
    "tablet:justify-stretch",
    "tablet:px-3",
    "tablet:py-2",
    "tablet:gap-0",
    "tablet:overflow-visible",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
