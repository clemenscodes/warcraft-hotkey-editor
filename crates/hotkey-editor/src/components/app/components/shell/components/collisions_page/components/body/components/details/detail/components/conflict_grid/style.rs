use crate::{classes, styling::TailwindClass, tw};
const BASE: &[TailwindClass] = tw![
    "grid",
    "grid-cols-[repeat(auto-fill,minmax(450px,1fr))]",
    "gap-6",
    "flex-[1_1_0]",
    "min-h-0",
    "overflow-y-auto",
    "content-start",
    "p-[1rem_0.75rem_1rem_0]",
    "[scrollbar-width:thin]",
    "[scrollbar-color:color-mix(in_oklab,var(--color-warcraft-gold)_45%,transparent)_transparent]",
];
const MOBILE: &[TailwindClass] = tw![
    "mobile:grid-cols-[minmax(0,1fr)]",
    "mobile:flex-none",
    "mobile:min-h-[auto]",
    "mobile:overflow-y-visible",
    "mobile:p-[1rem_0]",
];
const TABLET: &[TailwindClass] = tw![
    "tablet:grid-cols-[minmax(0,1fr)]",
    "tablet:flex-none",
    "tablet:min-h-[auto]",
    "tablet:overflow-y-visible",
    "tablet:p-[1rem_0]",
];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
