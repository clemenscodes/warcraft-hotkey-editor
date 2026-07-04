use crate::classes;

const BASE: &[&str] = &[
    "inline-flex",
    "items-center",
    "justify-center",
    "w-[9.4cqi]",
    "h-[9.4cqi]",
    "leading-none",
    "[&_svg]:block",
    "[&_svg]:w-full",
    "[&_svg]:h-full",
];

const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
