use crate::classes;

const BASE: &[&str] = &[
    "flex-[0_0_auto]",
    "ml-auto",
    "font-friz-quadrata",
    "text-[1.2rem]",
    "italic",
    "text-[rgba(192,200,218,0.55)]",
    "[text-shadow:1px_1px_0_#000]",
];
const MOBILE: &[&str] = &["mobile:text-[1.55rem]"];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
