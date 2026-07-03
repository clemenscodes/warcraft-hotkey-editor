use crate::classes;

// The label column of the alt-state header; clamps its width so long names ellipsize
// rather than push the controls off the row.
const BASE: &[&str] = &["min-w-0"];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
