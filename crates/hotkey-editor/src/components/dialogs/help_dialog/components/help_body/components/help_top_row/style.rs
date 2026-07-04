use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw!["flex", "flex-row", "items-start", "gap-[3.2rem]"];
const MOBILE: &[TailwindClass] = tw!["mobile:flex-col", "mobile:gap-[2.6rem]"];
const TABLET: &[TailwindClass] = tw!["tablet:flex-col", "tablet:gap-[2.6rem]"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
