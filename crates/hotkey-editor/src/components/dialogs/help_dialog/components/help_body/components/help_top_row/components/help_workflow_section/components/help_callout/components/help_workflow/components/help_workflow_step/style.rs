use crate::classes;

const BASE: &[&str] = &["text-[1.7rem]/[1.55]", "text-warcraft-text-primary"];
const MOBILE: &[&str] = &["mobile:text-[1.45rem]/[1.45]"];
const TABLET: &[&str] = &["tablet:text-[1.45rem]/[1.45]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
