use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "flex",
    "flex-row",
    "items-center",
    "justify-end",
    "gap-[0.65rem]",
    "min-w-0",
];

const MOBILE: &[TailwindClass] = tw!["mobile:gap-1"];
const TABLET: &[TailwindClass] = tw!["tablet:gap-1"];
const LAPTOP: &[TailwindClass] = tw!["laptop:gap-[clamp(0.2rem,0.2vw,0.5rem)]"];
const DESKTOP: &[TailwindClass] = tw!["desktop:gap-[clamp(0.2rem,0.2vw,0.5rem)]"];
const QHD: &[TailwindClass] = tw!["qhd:gap-[clamp(0.2rem,0.2vw,0.5rem)]"];
const UHD: &[TailwindClass] = tw!["uhd:gap-[clamp(0.2rem,0.2vw,0.5rem)]"];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
