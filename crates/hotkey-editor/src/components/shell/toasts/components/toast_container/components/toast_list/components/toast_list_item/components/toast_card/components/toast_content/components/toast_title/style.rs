use crate::components::shell::toasts::ToastType;
use crate::{classes, states};

const BASE: &[&str] = &[
    "text-warcraft-gold",
    "uppercase",
    "tracking-[0.06em]",
    "text-[1.9rem]",
    "leading-[1.2]",
    "[text-shadow:1px_1px_0_#000]",
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

const SUCCESS: &[&str] = &["text-[#6dd49a]"];
const ERROR: &[&str] = &["text-[#ff9090]"];
const WARNING: &[&str] = &["text-[#ffb347]"];
const INFO: &[&str] = &[];
states! {
    ToastType, Success => SUCCESS, Error => ERROR, Warning => WARNING, Info => INFO
}
