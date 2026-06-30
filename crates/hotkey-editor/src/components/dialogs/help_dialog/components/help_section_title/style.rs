use crate::classes;

const BASE: &[&str] = &[
    "m-0",
    "uppercase",
    "tracking-widest",
    "text-heading",
    "text-warcraft-gold",
    "text-shadow-emboss",
];
const MOBILE: &[&str] = &["mobile:text-heading-sm", "mobile:text-center"];
const TABLET: &[&str] = &["tablet:text-heading-sm", "tablet:text-center"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
