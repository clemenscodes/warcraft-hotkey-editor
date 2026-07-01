use crate::classes;

const BASE: &[&str] = &[
    "hidden",
    "flex-row",
    "items-center",
    "justify-end",
    "gap-[0.65rem]",
    "min-w-0",
];

const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &["laptop:flex"];
const DESKTOP: &[&str] = &["desktop:flex"];
const QHD: &[&str] = &["qhd:flex"];
const UHD: &[&str] = &["uhd:flex"];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
