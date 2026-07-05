use crate::{classes, styling::TailwindClass, tw};
const BASE: &[TailwindClass] = tw!["text-[#7b818d]", "text-[1.35rem]", "flex-none"];
const MOBILE: &[TailwindClass] =
    tw!["mobile:text-[max(0.5rem,min(1.35rem,calc((100vw_-_88px)/35)))]"];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
