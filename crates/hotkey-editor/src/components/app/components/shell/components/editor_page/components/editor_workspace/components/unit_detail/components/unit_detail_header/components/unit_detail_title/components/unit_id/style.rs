use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "text-[#7b818d]",
    "text-[clamp(0.95rem,0.42vw+0.35rem,1.25rem)]",
];
const MOBILE: &[TailwindClass] = tw!["mobile:text-[13px]"];
const TABLET: &[TailwindClass] = tw!["tablet:text-[13px]"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
