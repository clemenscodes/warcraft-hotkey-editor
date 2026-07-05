use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "flex",
    "items-center",
    "justify-end",
    "flex-none",
    "gap-4",
    "pt-[1.4rem]",
    "px-[4.5rem]",
    "pb-[1.8rem]",
    "border-t",
    "border-warcraft-gold/40",
];

const MOBILE: &[TailwindClass] = tw!["mobile:justify-center", "mobile:px-[1.5rem]"];
const TABLET: &[TailwindClass] = tw!["tablet:justify-center"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
