use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw!["flex", "flex-col", "items-center", "gap-8", "w-full"];
const MOBILE: &[TailwindClass] = tw!["mobile:gap-[0.85rem]"];
const TABLET: &[TailwindClass] = tw!["tablet:gap-[0.85rem]"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
