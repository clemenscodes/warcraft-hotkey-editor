use crate::classes;
const BASE: &[&str] = &[
    "flex",
    "flex-col",
    "items-center",
    "gap-[18px]",
    "m-0",
    "p-[30px_20px]",
    "bg-[rgba(13,31,61,0.5)]",
    "border",
    "border-[#24406a]",
    "rounded-[8px]",
    "cursor-pointer",
    "hover:border-[var(--race-color,#ffce63)]",
    "hover:shadow-[0_0_8px_var(--race-color-soft,rgba(255,206,99,0.45))]",
];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
