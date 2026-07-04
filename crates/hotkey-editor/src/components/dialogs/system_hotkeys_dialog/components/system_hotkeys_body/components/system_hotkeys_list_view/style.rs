use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "list-none",
    "m-0",
    "p-0",
    "w-full",
    "max-w-[110rem]",
    "mx-auto",
    "flex",
    "flex-col",
];

const MOBILE: &[TailwindClass] = tw!["mobile:max-w-full", "mobile:[touch-action:pan-y]"];
const TABLET: &[TailwindClass] = tw!["tablet:max-w-full", "tablet:[touch-action:pan-y]"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
