use crate::components::shell::toasts::ToastType;
use crate::{classes, states, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "flex-none",
    "flex",
    "items-center",
    "justify-center",
    "w-12",
    "h-12",
    "rounded-full",
    "self-center",
    "[&>svg]:w-8",
    "[&>svg]:h-8",
];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}

const SUCCESS: &[TailwindClass] = tw!["[background-color:#6dd49a2e]", "text-[#6dd49a]"];
const ERROR: &[TailwindClass] = tw!["[background-color:#ff7a7a33]", "text-[#ff7a7a]"];
const WARNING: &[TailwindClass] = tw!["[background-color:#ffb34733]", "text-[#ffb347]"];
const INFO: &[TailwindClass] = tw!["[background-color:#6aa1ff2e]", "text-[#6aa1ff]"];
states! {
    ToastType, Success => SUCCESS, Error => ERROR, Warning => WARNING, Info => INFO
}
