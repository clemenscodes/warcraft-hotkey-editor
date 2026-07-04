use crate::classes;

// The brand's box, and the query context every `cqi` length inside the brand resolves
// against. On mobile/tablet it grows to fill the space left beside the toolbar so the
// wordmark is as large as fits; on laptop and up it takes a viewport-responsive capped
// width. Either way the whole brand scales as one drawing off this box — one SVG, no
// truncation.
const BASE: &[&str] = &["[container-type:inline-size]", "min-w-0"];
const MOBILE: &[&str] = &["mobile:flex-auto"];
const TABLET: &[&str] = &["tablet:flex-auto"];
const LAPTOP: &[&str] = &["laptop:w-[clamp(11rem,42vw,48rem)]"];
const DESKTOP: &[&str] = &["desktop:w-[clamp(11rem,42vw,48rem)]"];
const QHD: &[&str] = &["qhd:w-[clamp(11rem,42vw,48rem)]"];
const UHD: &[&str] = &["uhd:w-[clamp(11rem,42vw,48rem)]"];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
