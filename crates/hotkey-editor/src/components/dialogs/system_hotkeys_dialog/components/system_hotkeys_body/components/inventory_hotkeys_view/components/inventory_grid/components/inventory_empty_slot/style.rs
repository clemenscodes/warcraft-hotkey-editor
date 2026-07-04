use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "flex",
    "flex-col",
    "items-center",
    "justify-center",
    "gap-[0.45rem]",
    "px-[0.6rem]",
    "py-[0.85rem]",
    "cursor-default",
    "border-2",
    "border-dashed",
    "border-[rgba(255,206,99,0.18)]",
    "text-[2.4rem]",
    "text-[rgba(255,206,99,0.25)]",
    "[background:rgba(8,14,30,0.5)]",
];

const MOBILE: &[TailwindClass] = tw!["mobile:aspect-[1/0.85]", "mobile:text-[1.4rem]"];
const TABLET: &[TailwindClass] = tw!["tablet:aspect-[1/0.85]", "tablet:text-[1.4rem]"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
