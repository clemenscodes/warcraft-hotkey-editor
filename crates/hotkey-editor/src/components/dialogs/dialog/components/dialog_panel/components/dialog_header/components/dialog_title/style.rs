use crate::classes;

const BASE: &[&str] = &[
    "m-0",
    "font-friz-quadrata",
    "uppercase",
    "tracking-title",
    "text-dialog-title",
    "text-warcraft-gold",
    "text-shadow-title",
];
const MOBILE: &[&str] = &[
    "mobile:text-dialog-title-sm",
    "mobile:tracking-title-sm",
    "mobile:whitespace-nowrap",
    "mobile:overflow-hidden",
    "mobile:text-ellipsis",
    "mobile:min-w-0",
    "mobile:max-w-full",
];
const TABLET: &[&str] = &[
    "tablet:text-dialog-title-sm",
    "tablet:tracking-title-sm",
    "tablet:whitespace-nowrap",
    "tablet:overflow-hidden",
    "tablet:text-ellipsis",
    "tablet:min-w-0",
    "tablet:max-w-full",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
