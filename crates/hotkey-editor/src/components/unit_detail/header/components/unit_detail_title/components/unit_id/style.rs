use crate::classes;

const BASE: &[&str] = &[
    "text-[#7b818d]",
    "text-[clamp(0.95rem,0.42vw+0.35rem,1.25rem)]",
    "font-mono",
];
const MOBILE: &[&str] = &["mobile:text-[1.6rem]"];
const TABLET: &[&str] = &["tablet:text-[1.6rem]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
