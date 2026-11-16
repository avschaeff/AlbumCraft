//! The colour icons of the Bright style: SVG files from Fluent UI System Icons (MIT,
//! `assets/icons/`, see its README), drawn instead of the one-colour glyphs of `crate::icons`
//! where one is listed in `SVG`. Views keep naming icons by their glyph (`icons::PRINT`): the
//! helpers here draw the colour one when the Bright style is on and the glyph otherwise.
//!
//! The SVGs are drawn with `resvg` at the size they are shown in screen pixels, once per size
//! (`texture`), and kept as textures.

use crate::icons;
use eframe::egui::{self, Color32, Pos2, Rect, Vec2};
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex, PoisonError};

macro_rules! svgs {
    ($($glyph:ident => $file:literal,)*) => {
        /// (glyph of `crate::icons`, its colour icon).
        pub const SVG: &[(&str, &[u8])] = &[$((icons::$glyph, include_bytes!(concat!("../assets/icons/", $file)))),*];
    };
}

svgs! {
    ADD => "add.svg",
    ALBUM => "album.svg",
    AUTO_FIX => "auto-fix.svg",
    COLLAGE => "collage.svg",
    DESKTOP => "desktop.svg",
    DRIVE => "drive.svg",
    EDIT => "edit.svg",
    EXPORT => "export.svg",
    FLAG => "flag.svg",
    FOLDER => "folder.svg",
    INFO => "info.svg",
    LOCATION => "location.svg",
    MAIL => "mail.svg",
    MAP => "map.svg",
    MOVIE => "movie.svg",
    MUSIC => "music.svg",
    PALETTE => "palette.svg",
    PERSON => "person.svg",
    PHOTOS => "photos.svg",
    PLAY => "play.svg",
    PRINT => "print.svg",
    SEARCH => "search.svg",
    STAR => "star.svg",
    TAG => "tag.svg",
    WB_SUNNY => "sunny.svg",
    WEB => "web.svg",
}

/// The colour icon of `glyph`, when the Bright style is on and it has one.
fn svg_of(glyph: &str) -> Option<&'static [u8]> {
    if !crate::theme::bright() {
        return None;
    }
    SVG.iter().find(|(g, _)| *g == glyph).map(|(_, svg)| *svg)
}

/// Drawn icons, by glyph and size in pixels (None: the SVG could not be read).
type Textures = HashMap<(&'static str, u32), Option<egui::TextureHandle>>;
static TEXTURES: LazyLock<Mutex<Textures>> = LazyLock::new(Default::default);

/// The texture of `glyph`'s colour icon at `px` pixels square.
fn texture(ctx: &egui::Context, glyph: &str, px: u32) -> Option<egui::TextureHandle> {
    let (key, svg) = SVG.iter().find(|(g, _)| *g == glyph).map(|(g, s)| (*g, *s))?;
    let px = px.clamp(8, 512);
    let mut all = TEXTURES.lock().unwrap_or_else(PoisonError::into_inner);
    all.entry((key, px)).or_insert_with(|| render(svg, px).map(|img| ctx.load_texture(format!("icon-{key}-{px}"), img, egui::TextureOptions::LINEAR))).clone()
}

/// Draw an SVG into a square picture of `px` pixels.
pub fn render(svg: &[u8], px: u32) -> Option<egui::ColorImage> {
    let tree = resvg::usvg::Tree::from_data(svg, &resvg::usvg::Options::default()).ok()?;
    let mut pixmap = resvg::tiny_skia::Pixmap::new(px, px)?;
    let size = tree.size();
    let scale = px as f32 / size.width().max(size.height());
    resvg::render(&tree, resvg::tiny_skia::Transform::from_scale(scale, scale), &mut pixmap.as_mut());
    Some(egui::ColorImage::from_rgba_premultiplied([px as usize, px as usize], pixmap.data()))
}

/// Draw `glyph` centred on `center`, `size` points tall: the colour icon in the Bright style
/// (faded when `color` is the weak text colour, i.e. disabled), else the glyph in `color`.
pub fn paint(ctx: &egui::Context, painter: &egui::Painter, glyph: &str, center: Pos2, size: f32, color: Color32) {
    if svg_of(glyph).is_some() {
        let side = size * 1.05;
        let px = (side * ctx.pixels_per_point()).round() as u32;
        if let Some(tex) = texture(ctx, glyph, px) {
            let rect = Rect::from_center_size(center, Vec2::splat(side));
            let tint = if color == crate::theme::t().text_weak { Color32::from_white_alpha(120) } else { Color32::WHITE };
            painter.image(tex.id(), rect, Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), tint);
            return;
        }
    }
    painter.text(center, egui::Align2::CENTER_CENTER, glyph, egui::FontId::proportional(size), color);
}

/// The leading icon of a label written `format!("{ICON}  text")`, when it has a colour icon to
/// draw instead (Bright style), and the rest of the label.
pub fn split(label: &str) -> Option<(&'static str, &str)> {
    let first = label.chars().next()?;
    let (glyph, _) = SVG.iter().find(|(g, _)| g.starts_with(first) && g.len() == first.len_utf8())?;
    svg_of(glyph)?;
    Some((glyph, label[first.len_utf8()..].trim_start()))
}

/// A label written `format!("{ICON}  text")` laid out for painting: its colour icon apart
/// (Bright style) or all of it as text.
pub struct IconLabel {
    icon: Option<&'static str>,
    pub galley: Arc<egui::Galley>,
    icon_size: f32,
}

/// Room between a colour icon and its text.
const GAP: f32 = 6.0;

impl IconLabel {
    pub fn new(painter: &egui::Painter, label: &str, font: egui::FontId, color: Color32) -> IconLabel {
        let icon_size = font.size * 1.25;
        match split(label) {
            Some((icon, rest)) => IconLabel { icon: Some(icon), galley: painter.layout_no_wrap(rest.to_string(), font, color), icon_size },
            None => IconLabel { icon: None, galley: painter.layout_no_wrap(label.to_string(), font, color), icon_size },
        }
    }

    pub fn size(&self) -> Vec2 {
        match self.icon {
            Some(_) if self.galley.size().x == 0.0 => Vec2::splat(self.icon_size),
            Some(_) => egui::vec2(self.icon_size + GAP + self.galley.size().x, self.galley.size().y.max(self.icon_size)),
            None => self.galley.size(),
        }
    }

    /// Paint it centred on `center`.
    pub fn paint(self, ctx: &egui::Context, painter: &egui::Painter, center: Pos2, color: Color32) {
        let size = self.size();
        let left = center.x - size.x / 2.0;
        match self.icon {
            Some(icon) => {
                paint(ctx, painter, icon, Pos2::new(left + self.icon_size / 2.0, center.y), self.icon_size, color);
                let gx = left + self.icon_size + GAP;
                painter.galley(Pos2::new(gx, center.y - self.galley.size().y / 2.0), self.galley, color);
            }
            None => painter.galley(center - self.galley.size() / 2.0, self.galley, color),
        }
    }
}

/// A label written `format!("{ICON}  text")` as the atoms of an egui button: its colour icon
/// and the text (Bright style), else the text as it is. `size`: the text size.
pub fn atoms(ctx: &egui::Context, label: impl Into<egui::RichText>, size: f32) -> egui::Atoms<'static> {
    let text: egui::RichText = label.into();
    let s = text.text().to_string();
    if let Some((glyph, rest)) = split(&s) {
        let side = size * 1.3;
        if let Some(tex) = texture(ctx, glyph, (side * ctx.pixels_per_point()).round() as u32) {
            let image = egui::Image::from_texture((tex.id(), Vec2::splat(side))).fit_to_exact_size(Vec2::splat(side));
            if rest.is_empty() {
                return egui::Atoms::new(image);
            }
            return egui::Atoms::new((image, egui::RichText::new(rest).size(size)));
        }
    }
    egui::Atoms::new(text.size(size))
}

#[cfg(test)]
mod tests {
    /// Every colour icon can be read and drawn, and stands for a different glyph.
    #[test]
    fn every_colour_icon_draws() {
        for (i, (glyph, svg)) in super::SVG.iter().enumerate() {
            let img = super::render(svg, 32).unwrap_or_else(|| panic!("colour icon {i} can't be read"));
            assert!(img.pixels.iter().any(|p| p.a() > 0), "colour icon {i} draws nothing");
            assert_eq!(super::SVG.iter().filter(|(g, _)| g == glyph).count(), 1, "glyph {glyph:?} has two colour icons");
        }
    }
}
