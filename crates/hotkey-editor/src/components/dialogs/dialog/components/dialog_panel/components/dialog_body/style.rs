use crate::classes;

const BASE: &[&str] = &[
    "flex-1",
    "min-h-0",
    "flex",
    "flex-col",
    "gap-6",
    "pt-body-pt",
    "px-body-px",
    "pb-body-pb",
    "overflow-y-auto",
    "scrollbar-gold",
];
const MOBILE: &[&str] = &[
    "mobile:pt-body-phone-pt",
    "mobile:px-body-phone-px",
    "mobile:pb-body-phone-pb",
];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
