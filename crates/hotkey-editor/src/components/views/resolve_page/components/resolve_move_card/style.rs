use crate::classes;
const BASE: &[&str] = &[
    "flex",
    "flex-col",
    "gap-[1.2rem]",
    "p-[1.4rem_1.5rem]",
    "box-border",
    "bg-[rgba(13,31,61,0.45)]",
    "border",
    "border-[#24406a]",
    "rounded-[10px]",
    "data-[stuck=true]:border-[rgba(255,154,106,0.5)]",
];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
