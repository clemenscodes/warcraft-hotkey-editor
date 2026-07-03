use crate::components::shell::toasts::ToastType;
use crate::{classes, states};

const BASE: &[&str] = &[
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
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}

const SUCCESS: &[&str] = &["[background-color:#6dd49a2e]", "text-[#6dd49a]"];
const ERROR: &[&str] = &["[background-color:#ff7a7a33]", "text-[#ff7a7a]"];
const WARNING: &[&str] = &["[background-color:#ffb34733]", "text-[#ffb347]"];
const INFO: &[&str] = &["[background-color:#6aa1ff2e]", "text-[#6aa1ff]"];
states! {
    ToastType, Success => SUCCESS, Error => ERROR, Warning => WARNING, Info => INFO
}
