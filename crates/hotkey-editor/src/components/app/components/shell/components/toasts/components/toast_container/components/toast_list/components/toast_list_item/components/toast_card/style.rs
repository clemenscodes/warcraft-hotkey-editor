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
    "kb-focus:[box-shadow:0_0_0_3px_var(--color-warcraft-highlight),0_12px_32px_color-mix(in_oklab,var(--color-warcraft-shadow)_55%,transparent)]",
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
    "border-warcraft-success",
    "[box-shadow:0_12px_32px_color-mix(in_oklab,var(--color-warcraft-shadow)_55%,transparent),0_0_24px_color-mix(in_oklab,var(--color-warcraft-success)_30%,transparent)]",
];
const ERROR: &[TailwindClass] = tw![
    "border-race-orc",
    "[box-shadow:0_12px_32px_color-mix(in_oklab,var(--color-warcraft-shadow)_55%,transparent),0_0_24px_color-mix(in_oklab,var(--color-race-orc)_35%,transparent)]",
];
const WARNING: &[TailwindClass] = tw![
    "border-warcraft-gold",
    "shadow-elevation-hl",
];
const INFO: &[TailwindClass] = tw![
    "border-race-human",
    "shadow-elevation-hl",
];
states! {
    ToastType, Success => SUCCESS, Error => ERROR, Warning => WARNING, Info => INFO
}
