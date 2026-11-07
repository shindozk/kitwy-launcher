//! Kitwy's Material You-inspired tokens for the Android game launcher only.
//! The vanilla game menus continue to use `horizon_client::ui_theme::color`.

pub(crate) mod color {
    use horizon_client::ui_theme::color::hex;

    pub const BG: u32 = hex(0x141218, 255);
    pub const SURFACE: u32 = hex(0x1D1B20, 255);
    pub const SURFACE_CONTAINER: u32 = hex(0x211F26, 255);
    pub const SURFACE_HIGH: u32 = hex(0x302D35, 255);
    pub const OUTLINE: u32 = hex(0x938F99, 72);
    pub const TEXT: u32 = hex(0xE6E1E9, 255);
    pub const MUTED: u32 = hex(0xCAC4D0, 255);
    pub const SUBTLE: u32 = hex(0xB0A9B5, 255);
    pub const PRIMARY: u32 = hex(0xD0BCFF, 255);
    pub const ON_PRIMARY: u32 = hex(0x381E72, 255);
    pub const PRIMARY_CONTAINER: u32 = hex(0x4F378B, 255);
    pub const ON_PRIMARY_CONTAINER: u32 = hex(0xEADDFF, 255);
    pub const ERROR: u32 = hex(0xFFB4AB, 255);

    // Semantic compatibility names used by the existing Android launcher views.
    pub const RAISED: u32 = SURFACE_HIGH;
    pub const HOVER: u32 = SURFACE;
    pub const BORDER: u32 = OUTLINE;
    pub const ACCENT: u32 = PRIMARY;
    pub const ON_ACCENT: u32 = ON_PRIMARY;

    pub const PILL_RADIUS: f32 = 24.0;
    pub const CARD_RADIUS: f32 = 28.0;
}

#[cfg(test)]
mod tests {
    use super::color;

    fn srgb(hex: u32) -> [f64; 3] {
        let rgb = [
            ((hex >> 16) & 0xff) as f64,
            ((hex >> 8) & 0xff) as f64,
            (hex & 0xff) as f64,
        ];
        rgb.map(|channel| {
            let value = channel / 255.0;
            if value <= 0.04045 {
                value / 12.92
            } else {
                ((value + 0.055) / 1.055).powf(2.4)
            }
        })
    }

    fn rgb(color: u32) -> u32 {
        let packed = color & 0x00ff_ffff;
        ((packed & 0xff) << 16) | (packed & 0xff00) | ((packed >> 16) & 0xff)
    }

    fn contrast(foreground: u32, background: u32) -> f64 {
        let luminance = |color| {
            let [r, g, b] = srgb(rgb(color));
            0.2126 * r + 0.7152 * g + 0.0722 * b
        };
        let (a, b) = (luminance(foreground), luminance(background));
        (a.max(b) + 0.05) / (a.min(b) + 0.05)
    }

    #[test]
    fn launcher_text_and_actions_meet_material_contrast_targets() {
        for surface in [
            color::BG,
            color::SURFACE,
            color::SURFACE_CONTAINER,
            color::SURFACE_HIGH,
        ] {
            assert!(contrast(color::TEXT, surface) >= 4.5);
            assert!(contrast(color::MUTED, surface) >= 4.5);
            assert!(contrast(color::PRIMARY, surface) >= 3.0);
        }
        assert!(contrast(color::ON_PRIMARY, color::PRIMARY) >= 4.5);
        assert!(contrast(color::ON_PRIMARY_CONTAINER, color::PRIMARY_CONTAINER) >= 4.5);
    }
}
