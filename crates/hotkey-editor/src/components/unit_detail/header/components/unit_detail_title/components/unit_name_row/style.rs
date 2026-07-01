use crate::classes;

const BASE: &[&str] = &[
    "flex",
    "items-center",
    "justify-between",
    "gap-[clamp(0.75rem,0.8vw,1rem)]",
    "min-h-0",
];
const MOBILE: &[&str] = &["mobile:flex-col", "mobile:items-stretch", "mobile:gap-[0.75rem]"];
const TABLET: &[&str] = &["tablet:gap-[1.5rem]", "tablet:min-h-[4.2rem]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
