use crate::classes;

const BASE: &[&str] = &["flex", "flex-col", "gap-[0.45rem]", "flex-none"];
const MOBILE: &[&str] = &[
    "mobile:flex-row",
    "mobile:items-baseline",
    "mobile:gap-[0.6rem]",
];
const TABLET: &[&str] = &[
    "tablet:flex-row",
    "tablet:items-baseline",
    "tablet:gap-[0.6rem]",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
