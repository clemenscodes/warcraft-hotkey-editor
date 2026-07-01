use crate::classes;

const BASE: &[&str] = &[
    "fixed",
    "top-0",
    "right-0",
    "h-dvh",
    "max-h-dvh",
    "z-[71]",
    "w-[min(85vw,320px)]",
    "[background:linear-gradient(170deg,#0c1d30_0%,#070e1c_100%)]",
    "border-l",
    "border-l-[rgba(255,206,99,0.3)]",
    "[box-shadow:-6px_0_40px_rgba(0,0,0,0.85)]",
    "flex",
    "flex-col",
    "[animation:burger-slide-in_0.22s_cubic-bezier(0.16,1,0.3,1)]",
    "[padding-top:env(safe-area-inset-top)]",
    "[padding-bottom:env(safe-area-inset-bottom)]",
];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
