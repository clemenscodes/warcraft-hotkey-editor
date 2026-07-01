use crate::classes;

const BASE: &[&str] = &[
    "m-0",
    "font-friz-quadrata",
    "font-normal",
    "text-[3.2rem]",
    "leading-[1.1]",
    "tracking-[0.04em]",
    "text-warcraft-gold",
    "whitespace-normal",
    "break-words",
    "text-left",
    "[text-shadow:1px_1px_0_rgba(0,0,0,0.92),0_0_14px_rgba(255,206,99,0.18)]",
];

const MOBILE: &[&str] = &[
    "mobile:text-[clamp(15px,4.5vw,22px)]",
    "mobile:leading-[1.15]",
    "mobile:flex-auto",
    "mobile:min-w-0",
    "mobile:overflow-hidden",
    "mobile:text-ellipsis",
    "mobile:whitespace-nowrap",
];

const TABLET: &[&str] = &[
    "tablet:text-[clamp(15px,4.5vw,22px)]",
    "tablet:leading-[1.15]",
    "tablet:flex-auto",
    "tablet:min-w-0",
    "tablet:overflow-hidden",
    "tablet:text-ellipsis",
    "tablet:whitespace-nowrap",
];

const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
