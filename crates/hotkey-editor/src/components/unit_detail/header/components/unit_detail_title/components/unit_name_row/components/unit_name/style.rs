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
const MOBILE: &[&str] = &["mobile:text-[2.2rem]", "mobile:text-center"];
const TABLET: &[&str] = &["tablet:text-[2.6rem]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
