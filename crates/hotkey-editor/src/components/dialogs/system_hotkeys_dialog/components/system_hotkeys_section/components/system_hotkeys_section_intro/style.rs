use crate::classes;

// The muted-gold caption above a category editor.
const BASE: &[&str] = &[
    "m-0",
    "max-w-[90rem]",
    "text-center",
    "font-friz-quadrata",
    "uppercase",
    "tracking-[0.1em]",
    "text-[2rem]",
    "leading-snug",
    "text-warcraft-gold/75",
    "[text-shadow:1px_1px_0_#000]",
];
const MOBILE: &[&str] = &[
    "mobile:max-w-full",
    "mobile:px-[0.25rem]",
    "mobile:text-[clamp(11px,3vw,14px)]",
    "mobile:tracking-[0.04em]",
    "mobile:leading-[1.35]",
];
const TABLET: &[&str] = &[
    "tablet:max-w-full",
    "tablet:px-[0.25rem]",
    "tablet:text-[clamp(11px,3vw,14px)]",
    "tablet:tracking-[0.04em]",
    "tablet:leading-[1.35]",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
