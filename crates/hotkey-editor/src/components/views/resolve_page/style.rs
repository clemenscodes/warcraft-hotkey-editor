use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw!["flex", "flex-col", "flex-[1_1_0]", "min-h-0"];
const MOBILE: &[TailwindClass] = tw!["mobile:flex-none"];
const TABLET: &[TailwindClass] = tw!["tablet:flex-none"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
