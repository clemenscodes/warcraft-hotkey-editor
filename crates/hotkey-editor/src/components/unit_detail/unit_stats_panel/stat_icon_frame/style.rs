use crate::classes;

const BASE: &[&str] = &[
    "flex-[0_0_auto]",
    "self-start",
    "w-[clamp(4rem,2.7vw+1.75rem,5.75rem)]",
    "h-[clamp(4rem,2.7vw+1.75rem,5.75rem)]",
    "[filter:drop-shadow(0_1px_2px_rgba(0,0,0,0.6))]",
];
const MOBILE: &[&str] = &["mobile:w-[5rem]", "mobile:h-[5rem]"];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
