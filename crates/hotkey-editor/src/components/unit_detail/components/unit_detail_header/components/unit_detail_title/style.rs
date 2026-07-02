use crate::classes;

const BASE: &[&str] = &[
    "min-w-0",
    "overflow-x-clip",
    "flex",
    "flex-col",
    "gap-[0.45rem]",
];
const MOBILE: &[&str] = &["mobile:items-center"];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
