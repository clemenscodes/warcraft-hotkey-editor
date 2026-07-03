use crate::classes;

const BASE: &[&str] = &[
    "flex",
    "flex-col",
    "gap-2",
    "self-stretch",
    "flex-[0_0_34rem]",
    "w-[34rem]",
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
const QHD: &[&str] = &["qhd:flex-[0_0_46rem]", "qhd:w-[46rem]"];
const UHD: &[&str] = &["uhd:flex-[0_0_62rem]", "uhd:w-[62rem]"];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
