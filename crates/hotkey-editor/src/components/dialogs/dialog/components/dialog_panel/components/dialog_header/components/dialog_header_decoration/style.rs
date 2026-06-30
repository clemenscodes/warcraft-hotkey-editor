use crate::classes;

const BASE: &[&str] = &[
    "block",
    "h-decoration-h",
    "w-auto",
    "flex-none",
    "decoration-flourish",
];
const MOBILE: &[&str] = &["mobile:w-decoration-phone-w"];
const TABLET: &[&str] = &["tablet:w-decoration-tablet-w"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
