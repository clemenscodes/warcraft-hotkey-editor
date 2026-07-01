use crate::classes;

const BASE: &[&str] = &["flex", "flex-row", "items-start", "gap-[3.2rem]"];
const MOBILE: &[&str] = &["mobile:flex-col", "mobile:gap-[2.6rem]"];
const TABLET: &[&str] = &["tablet:flex-col", "tablet:gap-[2.6rem]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
