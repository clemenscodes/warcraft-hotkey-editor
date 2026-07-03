use crate::classes;

const BASE: &[&str] = &[
    "w-full",
    "m-0",
    "px-6",
    "py-5",
    "rounded-md",
    "border",
    "border-[rgba(255,180,0,0.45)]",
    "bg-[rgba(60,40,0,0.45)]",
    "text-center",
    "font-friz-quadrata",
    "uppercase",
    "tracking-[0.08em]",
    "text-[1.75rem]/[1.625]",
    "text-warcraft-gold/85",
    "[text-shadow:1px_1px_0_#000]",
];

const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
