use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "flex",
    "flex-col",
    "gap-2",
    "self-stretch",
    "flex-[0_0_34rem]",
    "w-[34rem]",
];
const MOBILE: &[TailwindClass] = tw![
    "mobile:flex-row",
    "mobile:flex-none",
    "mobile:w-full",
    "mobile:gap-[0.5rem]",
];
const TABLET: &[TailwindClass] = tw!["tablet:flex-[0_0_18rem]", "tablet:w-72"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw!["qhd:flex-[0_0_46rem]", "qhd:w-[46rem]"];
const UHD: &[TailwindClass] = tw!["uhd:flex-[0_0_62rem]", "uhd:w-[62rem]"];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
