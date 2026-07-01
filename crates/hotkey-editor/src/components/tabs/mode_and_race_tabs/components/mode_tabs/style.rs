use crate::classes;

const BASE: &[&str] = &[
    "flex",
    "flex-col",
    "gap-2",
    "self-stretch",
    "flex-[0_0_var(--sidebar-column-width)]",
    "w-[var(--sidebar-column-width)]",
];
const MOBILE: &[&str] = &[
    "mobile:flex-row",
    "mobile:flex-none",
    "mobile:w-full",
    "mobile:gap-[0.5rem]",
];
const TABLET: &[&str] = &["tablet:flex-[0_0_18rem]", "tablet:w-72"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
