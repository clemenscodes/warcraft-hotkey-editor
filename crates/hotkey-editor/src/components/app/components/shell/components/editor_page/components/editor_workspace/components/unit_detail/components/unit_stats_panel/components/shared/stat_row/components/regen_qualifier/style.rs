use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "flex-[0_0_auto]",
    "ml-auto",
    "font-friz-quadrata",
    "text-[1.2rem]",
    "italic",
    "text-[rgba(192,200,218,0.55)]",
    "[text-shadow:1px_1px_0_#000]",
];
const MOBILE: &[TailwindClass] = tw!["mobile:text-[1.55rem]"];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
