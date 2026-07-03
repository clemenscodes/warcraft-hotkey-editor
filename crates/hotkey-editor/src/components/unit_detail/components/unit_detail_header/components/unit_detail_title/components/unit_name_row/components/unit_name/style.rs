use crate::classes;

const BASE: &[&str] = &[
    "font-friz-quadrata",
    "text-[clamp(2.2rem,0.85vw+1.1rem,3rem)]/[1.1]",
    "font-normal",
    "text-warcraft-gold",
    "[text-shadow:1px_1px_0_rgba(0,0,0,0.92)]",
    "m-0",
    "tracking-[0.03em]",
];
const MOBILE: &[&str] = &[
    "mobile:flex-auto",
    "mobile:min-w-0",
    "mobile:text-[clamp(17px,4.8vw,22px)]",
    "mobile:leading-[1.2]",
    "mobile:text-left",
    "mobile:[overflow-wrap:break-word]",
    "mobile:[word-break:break-word]",
];
const TABLET: &[&str] = &[
    "tablet:flex-auto",
    "tablet:min-w-0",
    "tablet:text-[clamp(17px,4.8vw,22px)]",
    "tablet:leading-[1.2]",
    "tablet:text-left",
    "tablet:[overflow-wrap:break-word]",
    "tablet:[word-break:break-word]",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
