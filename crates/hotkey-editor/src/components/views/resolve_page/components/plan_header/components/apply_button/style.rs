use crate::classes;
const BASE: &[&str] = &[
    "flex-none",
    "m-0",
    "px-[1.6rem]",
    "py-[0.55rem]",
    "border",
    "border-warcraft-gold",
    "rounded-[8px]",
    "cursor-pointer",
    "font-friz-quadrata",
    "text-[1.5rem]",
    "text-warcraft-gold",
    "[background:linear-gradient(180deg,#2a5085_0%,#1a3a5c_100%)]",
    "[text-shadow:1px_1px_0_rgba(0,0,0,0.92)]",
    "[transition:box-shadow_0.12s_ease,background_0.12s_ease]",
    "hover:[background:linear-gradient(180deg,#356dac_0%,#1f4a72_100%)]",
    "hover:[box-shadow:0_0_12px_rgba(255,206,99,0.4)]",
    "disabled:opacity-60",
    "disabled:cursor-wait",
];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
