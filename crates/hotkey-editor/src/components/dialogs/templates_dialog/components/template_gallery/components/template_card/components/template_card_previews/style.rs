use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw!["flex", "flex-row", "flex-nowrap", "items-start", "gap-8"];
const MOBILE: &[TailwindClass] = tw!["mobile:gap-[8px]"];
const TABLET: &[TailwindClass] = tw!["tablet:gap-[8px]"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
