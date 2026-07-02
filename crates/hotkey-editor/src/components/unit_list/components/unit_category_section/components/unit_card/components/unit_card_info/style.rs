use crate::classes;

const BASE: &[&str] = &["flex", "flex-col", "gap-[0.45rem]", "min-w-0", "flex-1"];

const MOBILE: &[&str] = &[
    "mobile:items-start",
    "mobile:text-left",
    "mobile:gap-1",
    "mobile:overflow-hidden",
];

const TABLET: &[&str] = &[
    "tablet:items-start",
    "tablet:text-left",
    "tablet:gap-1",
    "tablet:overflow-hidden",
];

const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
