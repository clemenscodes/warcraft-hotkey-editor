use crate::classes;

const BASE: &[&str] = &["text-body", "text-warcraft-text-secondary"];
const MOBILE: &[&str] = &["mobile:text-body-sm"];
const TABLET: &[&str] = &["tablet:text-body-sm"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
