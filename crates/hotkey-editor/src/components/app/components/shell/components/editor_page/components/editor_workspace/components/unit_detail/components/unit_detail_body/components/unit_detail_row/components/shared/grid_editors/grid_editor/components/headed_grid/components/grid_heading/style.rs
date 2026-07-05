use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "mx-0",
    "mt-[0.5rem]",
    "mb-[0.75rem]",
    "text-[20px]",
    "font-normal",
    "uppercase",
    "tracking-[0.08em]",
    "text-warcraft-gold",
    "text-shadow-drop-92",
];

const MOBILE: &[TailwindClass] = tw!["mobile:text-[16px]"];
const TABLET: &[TailwindClass] = tw!["tablet:text-[18px]"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw!["desktop:text-[22px]"];
const QHD: &[TailwindClass] = tw!["qhd:text-[25px]"];
const UHD: &[TailwindClass] = tw!["uhd:text-[30px]"];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
