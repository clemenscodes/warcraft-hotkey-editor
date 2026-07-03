use crate::classes;

const BASE: &[&str] = &[
    "flex",
    "flex-col",
    "min-w-0",
    "gap-[0.45rem]",
    "overflow-x-clip",
];
const MOBILE: &[&str] = &[
    "mobile:flex-1",
    "mobile:items-start",
    "mobile:gap-[3px]",
    "mobile:text-left",
    "mobile:overflow-visible",
];
const TABLET: &[&str] = &[
    "tablet:flex-1",
    "tablet:items-start",
    "tablet:gap-[3px]",
    "tablet:text-left",
    "tablet:overflow-visible",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
