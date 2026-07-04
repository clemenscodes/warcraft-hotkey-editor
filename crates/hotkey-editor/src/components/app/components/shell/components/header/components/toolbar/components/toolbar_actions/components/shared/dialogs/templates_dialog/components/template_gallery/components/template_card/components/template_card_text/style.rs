use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw!["flex", "flex-col", "gap-2"];
const MOBILE: &[TailwindClass] = tw!["mobile:gap-[4px]"];
const TABLET: &[TailwindClass] = tw!["tablet:gap-[4px]"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
