use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "text-[1.05rem]",
    "leading-[1.25]",
    "pb-[0.1rem]",
    "overflow-hidden",
    "text-ellipsis",
    "whitespace-nowrap",
    "min-w-0",
];

const MOBILE: &[TailwindClass] = tw![
    "mobile:block",
    "mobile:w-full",
    "mobile:text-[11px]",
    "mobile:leading-[1.2]",
    "mobile:data-[selected=true]:text-white",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:block",
    "tablet:w-full",
    "tablet:text-[11px]",
    "tablet:leading-[1.2]",
    "tablet:data-[selected=true]:text-white",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw!["desktop:text-[1.35rem]"];
const QHD: &[TailwindClass] = tw!["qhd:text-[1.35rem]"];
const UHD: &[TailwindClass] = tw!["uhd:text-[1.35rem]"];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
