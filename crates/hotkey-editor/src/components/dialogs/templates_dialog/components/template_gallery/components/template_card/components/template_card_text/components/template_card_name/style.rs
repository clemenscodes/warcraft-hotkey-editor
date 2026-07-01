use crate::classes;

const BASE: &[&str] = &[
    "m-0",
    "font-friz-quadrata",
    "text-[2.75rem]",
    "uppercase",
    "tracking-[0.08em]",
    "text-inherit",
    "[text-shadow:1px_1px_0_#000]",
];

const MOBILE: &[&str] = &[
    "mobile:text-[clamp(17px,5vw,24px)]",
    "mobile:tracking-[0.06em]",
    "mobile:text-warcraft-gold",
];

const TABLET: &[&str] = &[
    "tablet:text-[clamp(17px,5vw,24px)]",
    "tablet:tracking-[0.06em]",
    "tablet:text-warcraft-gold",
];

const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
