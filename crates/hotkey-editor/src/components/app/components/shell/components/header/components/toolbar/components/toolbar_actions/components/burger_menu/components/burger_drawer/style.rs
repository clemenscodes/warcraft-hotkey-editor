use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "fixed",
    "top-0",
    "right-0",
    "h-dvh",
    "max-h-dvh",
    "z-[71]",
    "w-[min(74vw,280px)]",
    "[background:linear-gradient(170deg,#0c1d30_0%,#070e1c_100%)]",
    "border-l",
    "border-l-[rgba(255,206,99,0.3)]",
    "[box-shadow:-6px_0_40px_rgba(0,0,0,0.85)]",
    "flex",
    "flex-col",
    "translate-x-0",
    "starting:translate-x-full",
    "transition-transform",
    "duration-[220ms]",
    "ease-[cubic-bezier(0.16,1,0.3,1)]",
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
