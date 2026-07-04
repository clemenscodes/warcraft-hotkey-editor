use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw!["inline-flex", "w-[0.8rem]", "shrink-0", "text-[0.9rem]"];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
