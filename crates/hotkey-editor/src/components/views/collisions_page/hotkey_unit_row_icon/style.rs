use crate::classes;

const BASE: &[&str] = &[
    "h-[80px]",
    "w-[80px]",
    "shrink-0",
    "border",
    "border-warcraft-blue",
    "rounded-[4px]",
    "object-cover",
];
const MOBILE: &[&str] = &["mobile:h-[66px]", "mobile:w-[66px]"];
const TABLET: &[&str] = &["tablet:h-[92px]", "tablet:w-[92px]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
