use crate::classes;

const BASE: &[&str] = &["m-0", "text-[1.6rem]/[1.6]", "text-warcraft-text-primary"];
const MOBILE: &[&str] = &["mobile:text-[1.4rem]/[1.5]"];
const TABLET: &[&str] = &["tablet:text-[1.4rem]/[1.5]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
