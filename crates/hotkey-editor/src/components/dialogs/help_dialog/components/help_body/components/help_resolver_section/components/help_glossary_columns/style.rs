use crate::classes;

const BASE: &[&str] = &["flex", "flex-row", "items-start", "gap-columns"];
const MOBILE: &[&str] = &["mobile:flex-col", "mobile:gap-section"];
const TABLET: &[&str] = &["tablet:flex-col", "tablet:gap-section"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
