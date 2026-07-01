use crate::classes;

const BASE: &[&str] = &["flex", "flex-row", "flex-nowrap", "items-start", "gap-8"];
const MOBILE: &[&str] = &["mobile:gap-[8px]"];
const TABLET: &[&str] = &["tablet:gap-[8px]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
