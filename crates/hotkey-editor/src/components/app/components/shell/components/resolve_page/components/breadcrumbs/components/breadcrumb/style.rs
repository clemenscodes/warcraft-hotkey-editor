use crate::{classes, styling::TailwindClass, tw};
const BASE: &[TailwindClass] = tw![
    "group",
    "m-0",
    "p-[0.25rem_0.15rem]",
    "bg-transparent",
    "border-none",
    "cursor-pointer",
    "inline-flex",
    "items-center",
    "gap-[0.45rem]",
    "font-friz-quadrata",
    "text-[#9aa3b2]",
    "transition-colors",
    "duration-[120ms]",
    "hover:text-[#e0d8c8]",
    "data-[active=true]:text-warcraft-gold",
    "data-[active=true]:[text-shadow:1px_1px_0_#000]",
];
const MOBILE: &[TailwindClass] = tw!["mobile:flex-none", "mobile:snap-start"];
const TABLET: &[TailwindClass] = tw!["tablet:flex-none", "tablet:snap-start"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
