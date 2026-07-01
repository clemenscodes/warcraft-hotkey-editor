use crate::classes;

// The muted italic note shown in place of a hotkey field for passive abilities.
const BASE: &[&str] = &["m-0", "text-[1.45rem]", "italic", "text-[#7b818d]"];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
