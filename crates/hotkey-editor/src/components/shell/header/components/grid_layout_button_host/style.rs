use crate::classes;

const BASE: &[&str] = &["hidden", "items-center", "justify-center"];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &["laptop:flex"];
const DESKTOP: &[&str] = &["desktop:flex"];
const QHD: &[&str] = &["qhd:flex"];
const UHD: &[&str] = &["uhd:flex"];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
