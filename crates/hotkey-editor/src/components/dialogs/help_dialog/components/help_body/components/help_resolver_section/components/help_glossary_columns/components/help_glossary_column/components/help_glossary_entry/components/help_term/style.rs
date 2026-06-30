use crate::classes;

const BASE: &[&str] = &[
    "m-0",
    "text-term",
    "text-warcraft-gold",
    "text-shadow-emboss",
];
const MOBILE: &[&str] = &["mobile:text-term-sm"];
const TABLET: &[&str] = &["tablet:text-term-sm"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
