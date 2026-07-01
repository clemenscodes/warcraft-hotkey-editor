use crate::classes;
const BASE: &[&str] = &[
    "m-0",
    "p-0",
    "bg-transparent",
    "border-none",
    "cursor-pointer",
    "text-[14px]",
    "text-[#9aa3b2]",
    "underline",
    "underline-offset-2",
    "hover:text-[var(--race-color,#ffce63)]",
];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
