use crate::classes;

const BASE: &[&str] = &["relative"];
const MOBILE: &[&str] = &["mobile:shrink-0", "mobile:w-[95px]"];
const TABLET: &[&str] = &["tablet:shrink-0", "tablet:w-[95px]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
