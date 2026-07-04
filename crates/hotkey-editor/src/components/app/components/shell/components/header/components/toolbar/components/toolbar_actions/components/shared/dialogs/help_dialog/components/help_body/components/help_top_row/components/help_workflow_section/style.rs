use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw!["flex", "flex-col", "gap-[1.2rem]", "flex-1", "min-w-0"];
const MOBILE: &[TailwindClass] = tw!["mobile:flex-none"];
const TABLET: &[TailwindClass] = tw!["tablet:flex-none"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
