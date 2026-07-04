use crate::classes;

// The brand's box, and the query context every `cqi` length inside the brand resolves
// against. Its width is definite and viewport-proportional per band, never `flex-auto`
// (which grabs all free space and blows the wordmark up on tablet). On phones and
// tablets it is a fraction of the viewport; on laptop and up it is a capped `vw` box
// that sits left-aligned in its grid track beside the centered layout button. Either
// way the whole brand scales as one drawing off this box — one SVG, no truncation.
const BASE: &[&str] = &["[container-type:inline-size]", "min-w-0"];
const MOBILE: &[&str] = &["mobile:w-[65vw]"];
const TABLET: &[&str] = &["tablet:w-[40vw]"];
const LAPTOP: &[&str] = &["laptop:w-[clamp(12rem,20vw,56rem)]"];
const DESKTOP: &[&str] = &["desktop:w-[clamp(12rem,20vw,56rem)]"];
const QHD: &[&str] = &["qhd:w-[clamp(12rem,20vw,56rem)]"];
const UHD: &[&str] = &["uhd:w-[clamp(12rem,20vw,56rem)]"];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
