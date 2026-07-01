use crate::classes;

// The upload button is a layout-transparent wrapper: it renders no box of its own,
// only its hidden file input and the visible toolbar button.
const BASE: &[&str] = &["contents"];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
