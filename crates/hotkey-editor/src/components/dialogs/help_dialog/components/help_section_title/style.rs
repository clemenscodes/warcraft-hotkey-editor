use crate::classes;

const BASE: &[&str] = &[
    "m-0",
    "uppercase",
    "tracking-widest",
    "text-[2.2rem]/[1.2]",
    "text-warcraft-gold",
    "[text-shadow:1px_1px_0_#000]",
];
const MOBILE: &[&str] = &["mobile:text-[1.9rem]/[1.2]", "mobile:text-center"];
const TABLET: &[&str] = &["tablet:text-[1.9rem]/[1.2]", "tablet:text-center"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
