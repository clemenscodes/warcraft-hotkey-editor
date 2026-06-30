use crate::classes;

const BASE: &[&str] = &[
    "px-[2.2rem]",
    "py-[1.8rem]",
    "border",
    "border-warcraft-gold/45",
    "rounded-xl",
    "bg-[linear-gradient(180deg,rgba(40,30,8,0.45)_0%,rgba(15,12,4,0.35)_100%)]",
    "shadow-[inset_0_0_0_1px_rgba(255,206,99,0.08),0_0_18px_rgba(255,206,99,0.12)]",
];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
