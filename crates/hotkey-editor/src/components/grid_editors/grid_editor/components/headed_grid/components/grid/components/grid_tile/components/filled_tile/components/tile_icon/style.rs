use crate::classes;

const BASE: &[&str] = &[
    "w-full",
    "h-full",
    "object-cover",
    "text-[0px]",
    "leading-[0]",
    "text-transparent",
    "[background:radial-gradient(circle_at_center,rgba(255,206,99,0.08)_0%,rgba(255,206,99,0)_65%)]",
];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
