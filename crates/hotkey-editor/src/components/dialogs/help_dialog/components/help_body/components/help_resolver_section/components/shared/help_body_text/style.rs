use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw!["m-0", "text-[1.6rem]/[1.6]", "text-warcraft-text-primary"];
const MOBILE: &[TailwindClass] = tw!["mobile:text-[1.4rem]/[1.5]"];
const TABLET: &[TailwindClass] = tw!["tablet:text-[1.4rem]/[1.5]"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
