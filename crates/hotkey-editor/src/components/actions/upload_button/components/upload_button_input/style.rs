use crate::classes;

// The file input is visually hidden but still focusable/clickable: parked far
// offscreen at a single pixel with zero opacity.
const BASE: &[&str] = &["absolute", "left-[-9999px]", "w-px", "h-px", "opacity-0"];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
