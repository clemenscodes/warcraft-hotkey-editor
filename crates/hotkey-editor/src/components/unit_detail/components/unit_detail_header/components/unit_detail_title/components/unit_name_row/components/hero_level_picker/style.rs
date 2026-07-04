use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw!["relative"];
const MOBILE: &[TailwindClass] = tw!["mobile:shrink-0", "mobile:w-[95px]"];
const TABLET: &[TailwindClass] = tw!["tablet:shrink-0", "tablet:w-[95px]"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
