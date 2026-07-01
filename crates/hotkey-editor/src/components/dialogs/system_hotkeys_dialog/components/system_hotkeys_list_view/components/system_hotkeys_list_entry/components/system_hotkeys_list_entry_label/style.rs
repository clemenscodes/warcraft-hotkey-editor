use crate::classes;

const BASE: &[&str] = &[
    "font-friz-quadrata",
    "uppercase",
    "tracking-[0.08em]",
    "text-[2.8rem]",
    "leading-tight",
    "text-[#d6dcec]",
    "[text-shadow:1px_1px_0_#000]",
];

const MOBILE: &[&str] = &[
    "mobile:[flex:1_1_auto]",
    "mobile:min-w-0",
    "mobile:text-[clamp(12px,3.4vw,15px)]",
    "mobile:tracking-[0.04em]",
    "mobile:leading-[1.25]",
    "mobile:whitespace-normal",
    "mobile:[overflow-wrap:break-word]",
    "mobile:[word-break:break-word]",
];

const TABLET: &[&str] = &[
    "tablet:[flex:1_1_auto]",
    "tablet:min-w-0",
    "tablet:text-[clamp(12px,3.4vw,15px)]",
    "tablet:tracking-[0.04em]",
    "tablet:leading-[1.25]",
    "tablet:whitespace-normal",
    "tablet:[overflow-wrap:break-word]",
    "tablet:[word-break:break-word]",
];

const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
