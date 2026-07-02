use crate::classes;

const BASE: &[&str] = &[
    "group",
    "flex",
    "items-baseline",
    "gap-[0.55rem]",
    "text-[clamp(1.35rem,0.95rem+0.32vw,1.7rem)]/[1.2]",
    "[text-shadow:1px_1px_0_#000]",
    "min-w-0",
    "data-[regen=true]:mt-[-0.2rem]",
    "data-[regen=true]:pl-[1.25rem]",
    "data-[primary=true]:[text-shadow:1px_1px_0_#000,0_0_8px_rgba(255,206,99,0.35)]",
];
const MOBILE: &[&str] = &["mobile:text-[2rem]/[1.3]"];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
