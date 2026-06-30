use crate::classes;

const BASE: &[&str] = &[
    "absolute",
    "right-close-right",
    "top-1/2",
    "-translate-y-1/2",
    "w-close-size",
    "h-close-size",
    "text-close",
    "font-friz-quadrata",
    "dialog-close-control",
    "kb-focus:text-white",
    "kb-focus:text-shadow-kb-focus",
];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
