use crate::classes;

const BASE: &[&str] = &["flex", "items-stretch", "gap-10", "grow", "min-w-0"];
const MOBILE: &[&str] = &["mobile:flex-col", "mobile:gap-[0.6rem]"];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
