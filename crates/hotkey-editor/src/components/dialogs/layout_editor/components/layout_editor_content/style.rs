use crate::classes;

const BASE: &[&str] = &[
    "flex",
    "flex-col",
    "items-center",
    "justify-center",
    "gap-[4rem]",
];
const MOBILE: &[&str] = &["mobile:justify-start", "mobile:gap-[20px]"];
const TABLET: &[&str] = &["tablet:justify-start", "tablet:gap-[20px]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
