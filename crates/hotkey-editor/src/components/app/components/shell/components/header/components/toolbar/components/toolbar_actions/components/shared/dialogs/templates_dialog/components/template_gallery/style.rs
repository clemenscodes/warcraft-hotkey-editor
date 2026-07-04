use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw!["grid", "grid-cols-2", "gap-9", "w-full"];
const MOBILE: &[TailwindClass] = tw!["mobile:grid-cols-1", "mobile:gap-[10px]"];
const TABLET: &[TailwindClass] = tw!["tablet:grid-cols-1", "tablet:gap-[10px]"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
