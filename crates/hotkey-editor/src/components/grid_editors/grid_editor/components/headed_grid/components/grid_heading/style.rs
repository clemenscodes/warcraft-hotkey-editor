use crate::classes;

const BASE: &[&str] = &[
    "mx-0",
    "mt-[0.5rem]",
    "mb-[0.75rem]",
    "font-friz-quadrata",
    "text-[20px]",
    "font-normal",
    "uppercase",
    "tracking-[0.08em]",
    "text-warcraft-gold",
    "[text-shadow:1px_1px_0_rgba(0,0,0,0.92)]",
];

const MOBILE: &[&str] = &["mobile:text-[16px]"];
const TABLET: &[&str] = &["tablet:text-[18px]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &["desktop:text-[22px]"];
const QHD: &[&str] = &["qhd:text-[25px]"];
const UHD: &[&str] = &["uhd:text-[30px]"];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
