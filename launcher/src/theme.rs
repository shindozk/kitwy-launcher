//! Kitwy Launcher's local Material You design tokens. This stays local to the launcher so the
//! Android game and the shared Horizon website retain their existing visual language.
use gpui::{rgb, Hsla, WindowAppearance};
use horizon_launcher::backend::Appearance;
use horizon_launcher::themes::Accent;
use horizon_theme as tokens;
use std::time::Duration;

/// Launcher-specific dimensions and palette built on the shared `Night` data shape.
pub mod night {
    pub use horizon_theme::night::over;
    pub use horizon_theme::night::Night;
    pub use horizon_theme::night::{CAPTION, DISPLAY, PLAY, TITLE, UI, UNCARDED_STOPS};

    pub const FONT: &str = "Geist";
    pub const DISPLAY_FONT: &str = "Geist";
    /// Rounded Material surfaces replace the former stepped pixel corners.
    pub const PIXEL_CORNER: f32 = 8.;
    pub const RADIUS_CONTROL: f32 = 16.;

    pub const NIGHT: Night = Night {
        sky: [0x141218; 6],
        page: 0x141218,
        side: 0x1d1b20,
        card: 0x211f26,
        card_alpha: 1.,
        card_hi: 0x302d35,
        card_hi_alpha: 1.,
        stroke: 0x938f99,
        stroke_alpha: 0.22,
        rule_alpha: 0.16,
        ink: 0xe6e1e9,
        ink_soft: 0xcac4d0,
        muted: 0xb0a9b5,
        accent: 0xd0bcff,
        accent_hi: 0xd0bcff,
        on_accent: 0x381e72,
        success: 0xa8dab5,
        warn: 0xf5c174,
        danger: 0xffb4ab,
        dark: true,
    };

    pub const DAWN: Night = Night {
        sky: [0xf8f6fa; 6],
        page: 0xf8f6fa,
        side: 0xf2eff5,
        card: 0xfffbff,
        card_alpha: 1.,
        card_hi: 0xeee8f1,
        card_hi_alpha: 1.,
        stroke: 0x79747e,
        stroke_alpha: 0.22,
        rule_alpha: 0.14,
        ink: 0x1d1b20,
        ink_soft: 0x49454f,
        muted: 0x625d67,
        accent: 0x6750a4,
        accent_hi: 0x6750a4,
        on_accent: 0xffffff,
        success: 0x386a20,
        warn: 0x745b00,
        danger: 0xb3261e,
        dark: false,
    };
}

pub use tokens::{Step, CAPTION, DISPLAY, HEADING, LABEL, MICRO, TITLE, UI};
pub const FONT: &str = night::FONT;
pub const DISPLAY_FONT: &str = night::DISPLAY_FONT;
pub const MONO: &str = tokens::MONO;
/// Legacy helper name retained while its face now uses Geist rather than a pixel font.
pub const PIXEL: &str = night::FONT;

pub use tokens::{BLOCK_BEVEL as BEVEL, BLOCK_LIP as LIP};
pub use tokens::{CONTROL_LARGE, DISABLED, FIELD, PRESSED, RISE, ROW, SEGMENT};

pub const RADIUS: f32 = 12.;
pub const RADIUS_PLATE: f32 = 20.;
pub const CONTROL: f32 = 40.;
pub const TOPBAR: f32 = 64.;
pub const SIDEBAR: f32 = 260.;
pub const PAGE_GUTTER: f32 = 32.;

pub const QUICK: Duration = tokens::QUICK.duration();
pub const MOVE: Duration = tokens::MOVE.duration();
pub const ENTER: Duration = tokens::ENTER.duration();
pub const STAGGER: Duration = tokens::STAGGER;

#[derive(Clone, Copy)]
pub struct Theme {
    pub page: Hsla,
    pub plate_soft: Hsla,
    pub plate: Hsla,
    pub surface: Hsla,
    pub rule: Hsla,
    pub ink: Hsla,
    pub ink_soft: Hsla,
    pub muted: Hsla,
    pub accent: Hsla,
    pub on_accent: Hsla,
    pub danger: Hsla,
    pub success: Hsla,
    pub warn: Hsla,
    pub side: Hsla,
    pub panel: Hsla,
    pub raised: Hsla,
    pub button: Hsla,
    pub button_bevel: Hsla,
    pub well: Hsla,
    pub bevel: Hsla,
    pub lip: Hsla,
    pub outline: Hsla,
    pub face: Hsla,
    pub face_bevel: Hsla,
    pub face_lip: Hsla,
    pub on_face: Hsla,
    pub night: night::Night,
    pub dark: bool,
}

fn c(hex: u32) -> Hsla {
    rgb(hex).into()
}

/// Whether the launcher is pinned dark (`Some(true)`) or light, or follows the system (`None`).
pub fn pinned_dark(choice: Appearance) -> Option<bool> {
    match std::env::var("HORIZON_LAUNCHER_APPEARANCE").as_deref() {
        Ok("light") => Some(false),
        Ok("dark") => Some(true),
        _ => match choice {
            Appearance::Dark => Some(true),
            Appearance::Light => Some(false),
            Appearance::System => None,
        },
    }
}

impl Theme {
    /// Kitwy's dynamic-colour-inspired Material palette follows the player's appearance and accent.
    pub fn for_appearance(choice: Appearance, accent: Accent, system: WindowAppearance) -> Self {
        let system_dark = !matches!(
            system,
            WindowAppearance::Light | WindowAppearance::VibrantLight
        );
        let dark = pinned_dark(choice).unwrap_or(system_dark);
        let mut night = if dark { night::NIGHT } else { night::DAWN };
        let blocks = accent.apply(night.blocks());
        night.accent = blocks.face;
        night.accent_hi = blocks.accent;
        night.on_accent = blocks.on_face;
        Self::from(blocks, night)
    }

    fn from(p: tokens::Blocks, night: night::Night) -> Self {
        Self {
            page: c(p.page),
            plate_soft: c(p.raised),
            plate: c(p.button),
            surface: c(p.panel),
            rule: c(p.rule),
            ink: c(p.ink),
            ink_soft: c(p.ink_soft),
            muted: c(p.muted),
            accent: c(p.accent),
            on_accent: c(p.on_face),
            danger: c(p.danger),
            success: c(p.success),
            warn: c(p.warn),
            side: c(p.side),
            panel: c(p.panel),
            raised: c(p.raised),
            button: c(p.button),
            button_bevel: c(p.button_bevel),
            well: c(p.well),
            bevel: c(p.bevel),
            lip: c(p.lip),
            outline: c(p.outline),
            face: c(p.face),
            face_bevel: c(p.face_bevel),
            face_lip: c(p.face_lip),
            on_face: c(p.on_face),
            night,
            dark: p.dark,
        }
    }
}

pub const SELECTION: f32 = 0.22;

#[cfg(test)]
mod tests {
    use super::{night, tokens, SELECTION};

    fn blend(top: u32, under: u32, alpha: f32) -> u32 {
        let channel = |shift: u32| {
            let (a, b) = ((top >> shift & 0xff) as f32, (under >> shift & 0xff) as f32);
            ((a * alpha + b * (1. - alpha)).round() as u32) << shift
        };
        channel(16) | channel(8) | channel(0)
    }

    #[test]
    fn material_palette_keeps_text_focus_and_selection_readable() {
        for palette in [night::DAWN, night::NIGHT] {
            let look = if palette.dark { "dark" } else { "light" };
            let blocks = palette.blocks();
            for (name, bg) in [
                ("page", blocks.page),
                ("raised", blocks.raised),
                ("button", blocks.button),
                ("panel", blocks.panel),
                ("well", blocks.well),
            ] {
                let ring = tokens::contrast(blocks.accent, bg);
                assert!(ring >= 3.0, "{look} focus ring on {name}: {ring:.2}");
                let refused = tokens::contrast(blocks.danger, bg);
                assert!(refused >= 3.0, "{look} error mark on {name}: {refused:.2}");
            }
            let selection = blend(blocks.accent, blocks.panel, SELECTION);
            let selected_text = tokens::contrast(blocks.ink, selection);
            assert!(
                selected_text >= 4.5,
                "{look} selected text contrast: {selected_text:.2}"
            );
            let primary = tokens::contrast(blocks.on_face, blocks.face);
            assert!(
                primary >= 4.5,
                "{look} primary button contrast: {primary:.2}"
            );
        }
    }
}
