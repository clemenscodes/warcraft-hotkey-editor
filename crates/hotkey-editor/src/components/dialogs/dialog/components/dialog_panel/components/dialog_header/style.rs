use crate::classes;

const BASE: &[&str] = &[
    "relative",
    "flex",
    "items-center",
    "justify-center",
    "gap-6",
    "flex-none",
    "pt-header-pt",
    "px-gutter",
    "pb-header-pb",
    "border-b",
    "border-warcraft-gold-soft",
    "shadow-dialog-header",
];
const MOBILE: &[&str] = &["mobile:gap-2", "mobile:px-gutter-phone"];
const TABLET: &[&str] = &["tablet:gap-2", "tablet:px-gutter-tablet"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
