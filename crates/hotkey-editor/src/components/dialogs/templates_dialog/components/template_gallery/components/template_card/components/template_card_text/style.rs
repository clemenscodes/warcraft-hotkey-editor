use crate::classes;

const BASE: &[&str] = &["flex", "flex-col", "gap-2"];
const MOBILE: &[&str] = &["mobile:gap-[4px]"];
const TABLET: &[&str] = &["tablet:gap-[4px]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
