use crate::{classes, styling::TailwindClass, tw};
const BASE: &[TailwindClass] = tw![
    "absolute",
    "left-[50%]",
    "top-[50%]",
    "[transform:translate(-50%,-50%)]",
    "flex-none",
    "text-warcraft-gold",
    "text-[2.8rem]",
    "leading-[1]",
    "text-shadow-drop",
];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
