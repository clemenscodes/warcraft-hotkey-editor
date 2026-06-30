use crate::classes;

const BASE: &[&str] = &[
    "flex",
    "items-center",
    "justify-end",
    "flex-none",
    "gap-4",
    "pt-footer-pt",
    "px-gutter",
    "pb-footer-pb",
    "border-t",
    "border-warcraft-gold-soft",
];
const MOBILE: &[&str] = &["mobile:justify-center", "mobile:px-footer-phone-px"];
const TABLET: &[&str] = &["tablet:justify-center"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
