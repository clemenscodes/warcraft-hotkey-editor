use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "h-[80px]",
    "w-[80px]",
    "shrink-0",
    "border",
    "border-warcraft-blue",
    "rounded-[4px]",
    "object-cover",
];
const MOBILE: &[TailwindClass] = tw!["mobile:h-[66px]", "mobile:w-[66px]"];
const TABLET: &[TailwindClass] = tw!["tablet:h-[92px]", "tablet:w-[92px]"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
