use crate::classes;

const BASE: &[&str] = &[
    "m-0",
    "font-friz-quadrata",
    "uppercase",
    "tracking-[0.1em]",
    "text-[2.1rem]/[1.35]",
    "text-warcraft-gold/85",
];

const MOBILE: &[&str] = &[
    "mobile:text-[clamp(13px,3.5vw,16px)]",
    "mobile:tracking-[0.05em]",
];

const TABLET: &[&str] = &[
    "tablet:text-[clamp(13px,3.5vw,16px)]",
    "tablet:tracking-[0.05em]",
];

const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
