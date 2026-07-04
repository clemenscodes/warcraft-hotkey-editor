use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw!["data-[open=true]:rotate-180"];

const MOBILE: &[TailwindClass] = tw![
    "mobile:flex-none",
    "mobile:ml-[0.6rem]",
    "mobile:text-[0.9em]",
    "mobile:leading-none",
    "mobile:[transition:transform_0.18s_ease]",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:flex-none",
    "tablet:ml-[0.6rem]",
    "tablet:text-[0.9em]",
    "tablet:leading-none",
    "tablet:[transition:transform_0.18s_ease]",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
