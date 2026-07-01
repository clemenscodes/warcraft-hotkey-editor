use crate::classes;
const BASE: &[&str] = &[
    "w-[72px]",
    "h-[72px]",
    "border",
    "border-warcraft-blue",
    "rounded-[6px]",
    "object-cover",
    "group-hover:border-[var(--race-color,#ffce63)]",
    "group-hover:shadow-[0_0_8px_var(--race-color-soft,rgba(255,206,99,0.5))]",
];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
