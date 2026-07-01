use crate::classes;

const BASE: &[&str] = &["grid", "grid-cols-2", "gap-9", "w-full"];
const MOBILE: &[&str] = &["mobile:grid-cols-1", "mobile:gap-[10px]"];
const TABLET: &[&str] = &["tablet:grid-cols-1", "tablet:gap-[10px]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
