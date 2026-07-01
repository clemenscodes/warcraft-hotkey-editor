use crate::classes;

// The centred column that wraps a system-hotkeys category editor: its intro line
// above, the editor below.
const BASE: &[&str] = &["flex", "flex-col", "items-center", "gap-8", "w-full"];
const MOBILE: &[&str] = &["mobile:gap-[0.85rem]"];
const TABLET: &[&str] = &["tablet:gap-[0.85rem]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
