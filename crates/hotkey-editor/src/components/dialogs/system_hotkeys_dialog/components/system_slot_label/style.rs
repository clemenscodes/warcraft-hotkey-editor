use crate::classes;

// The slot caption: muted-gold Friz Quadrata, sized below the key so the binding
// wins the eye. Tightens on small viewports, more so in compact cells.
const BASE: &[&str] = &[
    "font-friz-quadrata",
    "uppercase",
    "tracking-[0.18em]",
    "text-[2.4rem]",
    "leading-none",
    "text-warcraft-gold/55",
    "[text-shadow:1px_1px_0_#000]",
];
const MOBILE: &[&str] = &[
    "mobile:text-[clamp(10px,2.6vw,12px)]",
    "mobile:tracking-[0.08em]",
    "mobile:data-[compact=true]:text-[clamp(9px,2.2vw,11px)]",
];
const TABLET: &[&str] = &[
    "tablet:text-[clamp(10px,2.6vw,12px)]",
    "tablet:tracking-[0.08em]",
    "tablet:data-[compact=true]:text-[clamp(9px,2.2vw,11px)]",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
