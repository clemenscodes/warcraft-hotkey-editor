use crate::classes;

const BASE: &[&str] = &[
    "grid",
    "grid-cols-[repeat(4,minmax(0,140px))]",
    "gap-[6px]",
    "w-fit",
    "max-w-full",
    "overflow-visible",
];

const MOBILE: &[&str] = &["mobile:grid-cols-[repeat(4,minmax(0,116px))]"];
const TABLET: &[&str] = &["tablet:grid-cols-[repeat(4,minmax(0,128px))]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &["desktop:grid-cols-[repeat(4,minmax(0,156px))]"];
const QHD: &[&str] = &["qhd:grid-cols-[repeat(4,minmax(0,172px))]"];
const UHD: &[&str] = &["uhd:grid-cols-[repeat(4,minmax(0,200px))]"];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
