use crate::components::app::components::shell::components::toasts::ToastType;
use crate::{classes, states, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "flex",
    "items-start",
    "gap-5",
    "px-8",
    "py-6",
    "rounded-xl",
    "border-2",
    "font-friz-quadrata",
    "text-warcraft-text-primary",
    "cursor-pointer",
    "outline-none",
    "[background:linear-gradient(135deg,#0c1932f7_0%,#060c1cf7_100%)]",
    "transition-all",
    "duration-[240ms]",
    "ease-[cubic-bezier(0.2,0.9,0.3,1)]",
    "starting:opacity-0",
    "starting:translate-x-8",
    "starting:scale-95",
    "kb-focus:[box-shadow:0_0_0_3px_#fff,0_12px_32px_#0000008c]",
];
const MOBILE: &[TailwindClass] = tw!["mobile:max-w-[calc(100vw-1.5rem)]"];
const TABLET: &[TailwindClass] = tw!["tablet:max-w-[calc(100vw-1.5rem)]"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}

const SUCCESS: &[TailwindClass] = tw![
    "border-[#6dd49a]",
    "[box-shadow:0_12px_32px_#0000008c,0_0_24px_#6dd49a4d]",
];
const ERROR: &[TailwindClass] = tw![
    "border-[#ff7a7a]",
    "[box-shadow:0_12px_32px_#0000008c,0_0_24px_#ff7a7a59]",
];
const WARNING: &[TailwindClass] = tw![
    "border-[#ffb347]",
    "[box-shadow:0_12px_32px_#0000008c,inset_0_1px_0_#ffffff0a]",
];
const INFO: &[TailwindClass] = tw![
    "border-[#6aa1ff]",
    "[box-shadow:0_12px_32px_#0000008c,inset_0_1px_0_#ffffff0a]",
];
states! {
    ToastType, Success => SUCCESS, Error => ERROR, Warning => WARNING, Info => INFO
}
