use crate::classes;

const BASE: &[&str] = &[
    "flex",
    "flex-row",
    "items-center",
    "justify-end",
    "gap-[0.65rem]",
    "min-w-0",
];

const MOBILE: &[&str] = &["mobile:gap-1"];
const TABLET: &[&str] = &["tablet:gap-1"];
const LAPTOP: &[&str] = &["laptop:gap-[clamp(0.2rem,0.2vw,0.5rem)]"];
const DESKTOP: &[&str] = &["desktop:gap-[clamp(0.2rem,0.2vw,0.5rem)]"];
const QHD: &[&str] = &["qhd:gap-[clamp(0.2rem,0.2vw,0.5rem)]"];
const UHD: &[&str] = &["uhd:gap-[clamp(0.2rem,0.2vw,0.5rem)]"];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
