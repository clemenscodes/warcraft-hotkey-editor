use crate::classes;

const BASE: &[&str] = &["flex", "flex-col", "gap-snug", "flex-1", "min-w-0"];
const MOBILE: &[&str] = &["mobile:flex-none"];
const TABLET: &[&str] = &["tablet:flex-none"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
