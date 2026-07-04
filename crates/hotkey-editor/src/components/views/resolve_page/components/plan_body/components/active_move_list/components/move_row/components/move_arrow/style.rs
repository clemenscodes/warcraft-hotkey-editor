use crate::{classes, styling::TailwindClass, tw};
const BASE: &[TailwindClass] = tw![
    "absolute",
    "left-[50%]",
    "top-[50%]",
    "[transform:translate(-50%,-50%)]",
    "flex-none",
    "text-[#b8a86a]",
    "text-[2.8rem]",
    "leading-[1]",
    "[text-shadow:1px_1px_0_#000]",
];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
