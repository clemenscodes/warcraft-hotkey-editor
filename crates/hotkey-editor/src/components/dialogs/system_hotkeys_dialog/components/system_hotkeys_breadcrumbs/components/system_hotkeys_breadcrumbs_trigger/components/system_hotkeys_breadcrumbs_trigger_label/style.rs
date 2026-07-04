use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![];

const MOBILE: &[TailwindClass] = tw![
    "mobile:[flex:1_1_auto]",
    "mobile:text-left",
    "mobile:whitespace-nowrap",
    "mobile:overflow-hidden",
    "mobile:text-ellipsis",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:[flex:1_1_auto]",
    "tablet:text-left",
    "tablet:whitespace-nowrap",
    "tablet:overflow-hidden",
    "tablet:text-ellipsis",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
