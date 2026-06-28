/// A named viewport size for the preview iframe. Widths sit inside the bands
/// of `docs/CSS_VIEWPORTS.md` so a preset triggers exactly that band's rules
/// inside the iframe.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ViewportPreset {
    label: &'static str,
    width: u32,
    height: u32,
}

impl ViewportPreset {
    pub const fn new(label: &'static str, width: u32, height: u32) -> Self {
        Self {
            label,
            width,
            height,
        }
    }

    pub fn label(&self) -> &'static str {
        self.label
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn defaults() -> Vec<Self> {
        let phone = Self::new("Phone", 390, 844);
        let large_phone = Self::new("Large phone", 600, 900);
        let tablet = Self::new("Tablet", 900, 1024);
        let desktop = Self::new("Desktop", 1440, 900);
        let full_hd = Self::new("Full HD", 1920, 1080);
        let wide = Self::new("Wide", 2200, 1200);
        let ultra_4k = Self::new("4K", 3840, 2160);
        vec![phone, large_phone, tablet, desktop, full_hd, wide, ultra_4k]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_cover_each_band() {
        let presets = ViewportPreset::defaults();
        let widths: Vec<u32> = presets.iter().map(ViewportPreset::width).collect();
        assert_eq!(widths, vec![390, 600, 900, 1440, 1920, 2200, 3840]);
        assert!(presets[0].width() <= 480);
        assert!(presets[3].width() >= 1100);
    }
}
