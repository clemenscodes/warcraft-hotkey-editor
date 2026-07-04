use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw!["flex", "justify-center", "gap-[0.6rem]"];
const MOBILE: &[TailwindClass] = tw!["mobile:gap-[0.3rem]"];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
