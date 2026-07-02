use crate::classes;
const BASE: &[&str] = &["font-mono", "text-[#7b818d]", "text-[1.35rem]", "flex-none"];
const MOBILE: &[&str] = &["mobile:text-[max(0.5rem,min(1.35rem,calc((100vw_-_88px)/35)))]"];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
