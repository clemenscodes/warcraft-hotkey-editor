use crate::classes;
const BASE: &[&str] = &[
    "w-[72px]",
    "h-[72px]",
    "border",
    "border-[#2a5085]",
    "rounded-[7px]",
    "object-cover",
    "group-[:not(:disabled):hover]:[border-color:var(--race-color,#ffce63)]",
    "group-[:not(:disabled):hover]:[box-shadow:0_0_8px_var(--race-color-soft,rgba(255,206,99,0.5))]",
];
const MOBILE: &[&str] = &[
    "mobile:w-[max(40px,min(72px,9vw))]",
    "mobile:h-[max(40px,min(72px,9vw))]",
];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
