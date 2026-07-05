use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "w-20",
    "h-20",
    "shrink-0",
    "object-cover",
    "border",
    "border-warcraft-blue",
    "rounded-[3px]",
    "bg-warcraft-bg-panel/70",
    "text-transparent",
    "text-[0]",
    "leading-[0]",
    "[image-rendering:auto]",
];

const MOBILE: &[TailwindClass] = tw![
    "mobile:w-[clamp(62px,16vw,78px)]",
    "mobile:h-[clamp(62px,16vw,78px)]",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:w-[clamp(62px,16vw,78px)]",
    "tablet:h-[clamp(62px,16vw,78px)]",
];

const LAPTOP: &[TailwindClass] = tw![
    "laptop:w-[clamp(40px,4vw+16px,64px)]",
    "laptop:h-[clamp(40px,4vw+16px,64px)]",
];

const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
