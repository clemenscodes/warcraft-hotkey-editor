use crate::classes;

const BASE: &[&str] = &["flex", "flex-col", "gap-[0.6rem]"];
const MOBILE: &[&str] = &["mobile:gap-[0.3rem]"];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
