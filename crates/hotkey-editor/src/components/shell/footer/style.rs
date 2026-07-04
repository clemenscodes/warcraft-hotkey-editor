use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "flex-none",
    "flex",
    "flex-wrap",
    "items-center",
    "justify-center",
    "gap-x-3",
    "gap-y-1",
    "pt-5",
    "pb-3",
    "text-sm",
    "tracking-wide",
    "text-white/60",
    "select-none",
];

const MOBILE: &[TailwindClass] = tw![
    "mobile:pt-2",
    "mobile:px-[max(0.5rem,env(safe-area-inset-left))]",
    "mobile:pb-[max(0.5rem,env(safe-area-inset-bottom))]",
    "mobile:text-center",
    "mobile:leading-[1.3]",
    "mobile:text-[11px]",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:pt-2",
    "tablet:px-[max(0.5rem,env(safe-area-inset-left))]",
    "tablet:pb-[max(0.5rem,env(safe-area-inset-bottom))]",
    "tablet:text-center",
    "tablet:leading-[1.3]",
    "tablet:text-xs",
    "tablet:mt-auto",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
