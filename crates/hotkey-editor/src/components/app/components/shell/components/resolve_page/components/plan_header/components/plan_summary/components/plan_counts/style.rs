use crate::{classes, styling::TailwindClass, tw};
const BASE: &[TailwindClass] = tw!["text-[1.35rem]", "text-warcraft-gold", "text-shadow-drop",];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
