use crate::{classes, styling::TailwindClass, tw};

// The object id under the name, in a monospace face; smaller on the mobile panel.
const BASE: &[TailwindClass] = tw!["text-[1.4rem]", "text-[#7b818d]"];

const MOBILE: &[TailwindClass] = tw![
    "mobile:m-0",
    "mobile:text-[12px]",
    "mobile:leading-[1.2]",
    "mobile:overflow-hidden",
    "mobile:whitespace-nowrap",
    "mobile:text-ellipsis",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:m-0",
    "tablet:text-[12px]",
    "tablet:leading-[1.2]",
    "tablet:overflow-hidden",
    "tablet:whitespace-nowrap",
    "tablet:text-ellipsis",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
