use crate::classes;

const BASE: &[&str] = &[
    "m-0",
    "font-friz-quadrata",
    "uppercase",
    "tracking-[0.08em]",
    "text-[2.5rem]/[1]",
    "text-warcraft-gold",
    "[text-shadow:1px_1px_0_#000,0_0_18px_rgba(255,206,99,0.35)]",
];
const MOBILE: &[&str] = &[
    "mobile:text-[clamp(12px,3.2vw,18px)]/[1]",
    "mobile:tracking-[0.02em]",
    "mobile:whitespace-nowrap",
    "mobile:overflow-hidden",
    "mobile:text-ellipsis",
    "mobile:min-w-0",
    "mobile:max-w-full",
];
const TABLET: &[&str] = &[
    "tablet:text-[clamp(12px,3.2vw,18px)]/[1]",
    "tablet:tracking-[0.02em]",
    "tablet:whitespace-nowrap",
    "tablet:overflow-hidden",
    "tablet:text-ellipsis",
    "tablet:min-w-0",
    "tablet:max-w-full",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
