use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw!["text-[1.7rem]/[1.55]", "text-warcraft-text-primary"];
const MOBILE: &[TailwindClass] = tw!["mobile:text-[1.45rem]/[1.45]"];
const TABLET: &[TailwindClass] = tw!["tablet:text-[1.45rem]/[1.45]"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
