use crate::classes;
const BASE: &[&str] = &[
    "absolute",
    "left-[50%]",
    "top-[50%]",
    "[transform:translate(-50%,-50%)]",
    "flex-none",
    "text-[#b8a86a]",
    "text-[2.8rem]",
    "leading-[1]",
    "[text-shadow:1px_1px_0_#000]",
];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
