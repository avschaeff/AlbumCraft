//! Light and dark themes: one set of colour tokens, read with `t()`, and the egui style built from them.
//!
//! Two styles, each light and dark: Calm, after PhotoCraft's "Studio" themes (flat surfaces,
//! hairline borders, rounded controls and a single blue accent), and Bright (gradients on the
//! bars and the sidebar, a green main button, colour icons from `crate::color_icons`). Never
//! hard-code a UI colour in a view; add a token here instead.

use eframe::egui::{self, Color32, CornerRadius, Stroke};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Tokens {
    pub dark: bool,
    pub toolbar_bg: Color32,
    /// Left folder list.
    pub left_bg: Color32,
    /// The lightbox / main content area.
    pub lightbox_bg: Color32,
    /// Folder header strips, status bar.
    pub header_bg: Color32,
    pub status_bg: Color32,
    /// The selection bar at the bottom.
    pub tray_bg: Color32,
    /// Cards, inputs, popups.
    pub surface: Color32,
    pub surface_hover: Color32,
    pub border: Color32,
    pub border_strong: Color32,
    pub text: Color32,
    pub text_strong: Color32,
    pub text_weak: Color32,
    pub accent: Color32,
    pub select_bg: Color32,
    pub select_border: Color32,
    /// Light accent wash used for hovered tiles.
    pub hover_wash: Color32,
    /// Behind the picture in the editor.
    pub edit_bg: Color32,
    pub star: Color32,
    pub star_outline: Color32,
    /// Pick flag and reject mark.
    pub pick: Color32,
    pub reject: Color32,
    pub placeholder: Color32,
    pub good: Color32,
    pub bad: Color32,
    /// The built-in map: sea, land, lakes, country borders.
    pub map_water: Color32,
    pub map_land: Color32,
    pub map_border: Color32,
    /// Bars (top bar, the bars at the bottom, the editor's bars): from top to bottom.
    pub band_top: Color32,
    pub band_bottom: Color32,
    /// The sidebar and the editor's tool panel, from top to bottom.
    pub side_top: Color32,
    pub side_bottom: Color32,
    /// The selected row of the sidebar, from top to bottom.
    pub select_top: Color32,
    pub select_bottom: Color32,
    /// The main button (Import), from top to bottom.
    pub primary_top: Color32,
    pub primary_bottom: Color32,
}

const LIGHT: Tokens = Tokens {
    dark: false,
    toolbar_bg: Color32::from_rgb(244, 245, 248),
    left_bg: Color32::from_rgb(246, 247, 250),
    lightbox_bg: Color32::from_rgb(255, 255, 255),
    header_bg: Color32::from_rgb(241, 243, 247),
    status_bg: Color32::from_rgb(244, 245, 248),
    tray_bg: Color32::from_rgb(236, 238, 243),
    surface: Color32::from_rgb(255, 255, 255),
    surface_hover: Color32::from_rgb(236, 240, 248),
    border: Color32::from_rgb(218, 222, 230),
    border_strong: Color32::from_rgb(190, 196, 208),
    text: Color32::from_rgb(38, 42, 52),
    text_strong: Color32::from_rgb(20, 24, 34),
    text_weak: Color32::from_rgb(110, 117, 132),
    accent: Color32::from_rgb(47, 123, 245),
    select_bg: Color32::from_rgb(214, 228, 252),
    select_border: Color32::from_rgb(47, 123, 245),
    hover_wash: Color32::from_rgb(236, 242, 253),
    edit_bg: Color32::from_rgb(226, 229, 235),
    star: Color32::from_rgb(250, 190, 20),
    star_outline: Color32::from_rgb(170, 120, 0),
    pick: Color32::from_rgb(46, 160, 90),
    reject: Color32::from_rgb(214, 64, 64),
    placeholder: Color32::from_rgb(226, 229, 235),
    good: Color32::from_rgb(30, 130, 60),
    bad: Color32::from_rgb(200, 50, 50),
    map_water: Color32::from_rgb(196, 218, 236),
    map_land: Color32::from_rgb(244, 241, 232),
    map_border: Color32::from_rgb(178, 168, 190),
    band_top: Color32::from_rgb(255, 255, 255),
    band_bottom: Color32::from_rgb(255, 255, 255),
    side_top: Color32::from_rgb(246, 247, 250),
    side_bottom: Color32::from_rgb(246, 247, 250),
    select_top: Color32::from_rgb(214, 228, 252),
    select_bottom: Color32::from_rgb(214, 228, 252),
    primary_top: Color32::from_rgb(47, 123, 245),
    primary_bottom: Color32::from_rgb(47, 123, 245),
};

const DARK: Tokens = Tokens {
    dark: true,
    toolbar_bg: Color32::from_rgb(40, 41, 46),
    left_bg: Color32::from_rgb(35, 36, 40),
    lightbox_bg: Color32::from_rgb(27, 28, 31),
    header_bg: Color32::from_rgb(38, 39, 44),
    status_bg: Color32::from_rgb(34, 35, 39),
    tray_bg: Color32::from_rgb(40, 41, 46),
    surface: Color32::from_rgb(46, 47, 53),
    surface_hover: Color32::from_rgb(58, 60, 68),
    border: Color32::from_rgb(58, 60, 68),
    border_strong: Color32::from_rgb(82, 85, 96),
    text: Color32::from_rgb(220, 223, 230),
    text_strong: Color32::from_rgb(245, 246, 250),
    text_weak: Color32::from_rgb(145, 150, 165),
    accent: Color32::from_rgb(76, 148, 255),
    select_bg: Color32::from_rgb(36, 62, 104),
    select_border: Color32::from_rgb(76, 148, 255),
    hover_wash: Color32::from_rgb(50, 56, 70),
    edit_bg: Color32::from_rgb(20, 21, 24),
    star: Color32::from_rgb(250, 190, 20),
    star_outline: Color32::from_rgb(230, 170, 20),
    pick: Color32::from_rgb(80, 200, 120),
    reject: Color32::from_rgb(240, 90, 90),
    placeholder: Color32::from_rgb(52, 54, 60),
    good: Color32::from_rgb(90, 200, 120),
    bad: Color32::from_rgb(240, 100, 100),
    map_water: Color32::from_rgb(28, 38, 52),
    map_land: Color32::from_rgb(58, 60, 64),
    map_border: Color32::from_rgb(98, 96, 110),
    band_top: Color32::from_rgb(27, 28, 31),
    band_bottom: Color32::from_rgb(27, 28, 31),
    side_top: Color32::from_rgb(35, 36, 40),
    side_bottom: Color32::from_rgb(35, 36, 40),
    select_top: Color32::from_rgb(36, 62, 104),
    select_bottom: Color32::from_rgb(36, 62, 104),
    primary_top: Color32::from_rgb(76, 148, 255),
    primary_bottom: Color32::from_rgb(76, 148, 255),
};

/// The Bright style, light: sky-blue gradients, a green main button, colour icons.
const BRIGHT_LIGHT: Tokens = Tokens {
    dark: false,
    toolbar_bg: Color32::from_rgb(242, 247, 253),
    left_bg: Color32::from_rgb(238, 244, 252),
    lightbox_bg: Color32::from_rgb(255, 255, 255),
    header_bg: Color32::from_rgb(234, 242, 252),
    status_bg: Color32::from_rgb(232, 240, 250),
    tray_bg: Color32::from_rgb(228, 237, 249),
    surface: Color32::from_rgb(255, 255, 255),
    surface_hover: Color32::from_rgb(228, 239, 253),
    border: Color32::from_rgb(204, 216, 234),
    border_strong: Color32::from_rgb(170, 188, 214),
    text: Color32::from_rgb(34, 42, 58),
    text_strong: Color32::from_rgb(16, 24, 40),
    text_weak: Color32::from_rgb(102, 114, 136),
    accent: Color32::from_rgb(24, 118, 230),
    select_bg: Color32::from_rgb(200, 224, 254),
    select_border: Color32::from_rgb(24, 118, 230),
    hover_wash: Color32::from_rgb(226, 238, 254),
    edit_bg: Color32::from_rgb(222, 230, 242),
    star: Color32::from_rgb(255, 184, 0),
    star_outline: Color32::from_rgb(196, 120, 0),
    pick: Color32::from_rgb(34, 168, 84),
    reject: Color32::from_rgb(226, 56, 70),
    placeholder: Color32::from_rgb(222, 230, 242),
    good: Color32::from_rgb(24, 140, 64),
    bad: Color32::from_rgb(208, 44, 54),
    map_water: Color32::from_rgb(180, 214, 242),
    map_land: Color32::from_rgb(246, 243, 230),
    map_border: Color32::from_rgb(170, 160, 196),
    band_top: Color32::from_rgb(253, 254, 255),
    band_bottom: Color32::from_rgb(222, 234, 249),
    side_top: Color32::from_rgb(246, 250, 255),
    side_bottom: Color32::from_rgb(224, 235, 249),
    select_top: Color32::from_rgb(214, 234, 255),
    select_bottom: Color32::from_rgb(168, 206, 252),
    primary_top: Color32::from_rgb(74, 196, 104),
    primary_bottom: Color32::from_rgb(28, 148, 72),
};

/// The Bright style, dark: deep blue-grey gradients, a green main button, colour icons.
const BRIGHT_DARK: Tokens = Tokens {
    dark: true,
    toolbar_bg: Color32::from_rgb(38, 43, 54),
    left_bg: Color32::from_rgb(32, 37, 48),
    lightbox_bg: Color32::from_rgb(25, 28, 36),
    header_bg: Color32::from_rgb(38, 44, 57),
    status_bg: Color32::from_rgb(30, 34, 44),
    tray_bg: Color32::from_rgb(36, 41, 53),
    surface: Color32::from_rgb(44, 50, 64),
    surface_hover: Color32::from_rgb(58, 67, 86),
    border: Color32::from_rgb(60, 68, 88),
    border_strong: Color32::from_rgb(86, 96, 120),
    text: Color32::from_rgb(222, 227, 238),
    text_strong: Color32::from_rgb(246, 248, 252),
    text_weak: Color32::from_rgb(146, 156, 178),
    accent: Color32::from_rgb(96, 166, 255),
    select_bg: Color32::from_rgb(40, 76, 132),
    select_border: Color32::from_rgb(96, 166, 255),
    hover_wash: Color32::from_rgb(48, 62, 88),
    edit_bg: Color32::from_rgb(18, 20, 27),
    star: Color32::from_rgb(255, 190, 20),
    star_outline: Color32::from_rgb(236, 172, 20),
    pick: Color32::from_rgb(84, 210, 126),
    reject: Color32::from_rgb(246, 96, 104),
    placeholder: Color32::from_rgb(50, 56, 70),
    good: Color32::from_rgb(92, 206, 128),
    bad: Color32::from_rgb(246, 104, 110),
    map_water: Color32::from_rgb(26, 40, 60),
    map_land: Color32::from_rgb(56, 60, 68),
    map_border: Color32::from_rgb(100, 98, 120),
    band_top: Color32::from_rgb(50, 57, 73),
    band_bottom: Color32::from_rgb(32, 36, 47),
    side_top: Color32::from_rgb(40, 46, 60),
    side_bottom: Color32::from_rgb(27, 31, 41),
    select_top: Color32::from_rgb(62, 110, 186),
    select_bottom: Color32::from_rgb(38, 72, 132),
    primary_top: Color32::from_rgb(64, 184, 100),
    primary_bottom: Color32::from_rgb(30, 132, 66),
};

static IS_DARK: AtomicBool = AtomicBool::new(false);
static IS_BRIGHT: AtomicBool = AtomicBool::new(false);

/// The Bright style is on (colour icons, gradients).
pub fn bright() -> bool {
    IS_BRIGHT.load(Ordering::Relaxed)
}

/// The look of the interface, besides light or dark (saved in the library file as `Store.style`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Style {
    /// Flat surfaces and one-colour icons.
    Calm,
    /// Gradients and colour icons.
    Bright,
}

impl Style {
    pub fn from_name(s: &str) -> Style {
        if s == "bright" { Style::Bright } else { Style::Calm }
    }
    pub fn name(self) -> &'static str {
        match self {
            Style::Calm => "calm",
            Style::Bright => "bright",
        }
    }
    pub fn label(self) -> String {
        match self {
            Style::Calm => crate::i18n::tr!("style-calm"),
            Style::Bright => crate::i18n::tr!("style-bright"),
        }
    }
}

/// Fill `rect` (corners rounded by `radius`) with a vertical gradient from `top` to `bottom`.
pub fn paint_gradient(painter: &egui::Painter, rect: egui::Rect, radius: f32, top: Color32, bottom: Color32) {
    if top == bottom {
        painter.rect_filled(rect, radius, top);
        return;
    }
    let r = radius.min(rect.width() / 2.0).min(rect.height() / 2.0).max(0.0);
    // the outline, corners in a few steps; convex, so a fan from the centre fills it
    let mut points = vec![];
    let steps = if r > 0.0 { 6 } else { 0 };
    let corners = [
        (egui::pos2(rect.right() - r, rect.top() + r), -90.0f32),
        (egui::pos2(rect.right() - r, rect.bottom() - r), 0.0),
        (egui::pos2(rect.left() + r, rect.bottom() - r), 90.0),
        (egui::pos2(rect.left() + r, rect.top() + r), 180.0),
    ];
    for (c, start) in corners {
        for i in 0..=steps {
            let a = (start + 90.0 * i as f32 / steps.max(1) as f32).to_radians();
            points.push(c + egui::vec2(a.cos(), a.sin()) * r);
        }
    }
    let colour_at = |y: f32| {
        let f = ((y - rect.top()) / rect.height().max(1.0)).clamp(0.0, 1.0);
        top.lerp_to_gamma(bottom, f)
    };
    let mut mesh = egui::Mesh::default();
    mesh.colored_vertex(rect.center(), colour_at(rect.center().y));
    for p in &points {
        mesh.colored_vertex(*p, colour_at(p.y));
    }
    let n = points.len() as u32;
    for i in 0..n {
        mesh.add_triangle(0, 1 + i, 1 + (i + 1) % n);
    }
    painter.add(egui::Shape::mesh(mesh));
}

/// Paint a panel's gradient behind its content: called first in the panel's closure, with the
/// margin of its frame. Nothing in the Calm style (its frame's fill is the colour).
pub fn panel_gradient(ui: &egui::Ui, margin: egui::Margin, top: Color32, bottom: Color32) {
    if !bright() {
        return;
    }
    let rect = ui.max_rect() + margin;
    let painter = ui.ctx().layer_painter(ui.layer_id()).with_clip_rect(rect);
    paint_gradient(&painter, rect, 0.0, top, bottom);
}

/// The tokens of the theme in use.
pub fn t() -> &'static Tokens {
    match (IS_BRIGHT.load(Ordering::Relaxed), IS_DARK.load(Ordering::Relaxed)) {
        (false, false) => &LIGHT,
        (false, true) => &DARK,
        (true, false) => &BRIGHT_LIGHT,
        (true, true) => &BRIGHT_DARK,
    }
}

/// How the person chose the appearance (saved in the library file).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    System,
    Light,
    Dark,
}

impl Mode {
    pub fn from_name(s: &str) -> Mode {
        match s {
            "light" => Mode::Light,
            "dark" => Mode::Dark,
            _ => Mode::System,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Mode::System => "system",
            Mode::Light => "light",
            Mode::Dark => "dark",
        }
    }
    pub fn label(self) -> String {
        match self {
            Mode::System => crate::i18n::tr!("theme-system"),
            Mode::Light => crate::i18n::tr!("theme-light"),
            Mode::Dark => crate::i18n::tr!("theme-dark"),
        }
    }
}

/// Resolves the mode against the system theme, and rebuilds the egui style when the result changed.
pub fn apply(ctx: &egui::Context, mode: Mode, style: Style, force: bool) {
    let bright = style == Style::Bright;
    let style_changed = bright != IS_BRIGHT.swap(bright, Ordering::Relaxed);
    let pref = match mode {
        Mode::System => egui::ThemePreference::System,
        Mode::Light => egui::ThemePreference::Light,
        Mode::Dark => egui::ThemePreference::Dark,
    };
    if ctx.options(|o| o.theme_preference) != pref {
        ctx.set_theme(pref);
    }
    // "Same as system" when the system does not say: light (egui's own fallback is dark)
    if ctx.options(|o| o.fallback_theme) != egui::Theme::Light {
        ctx.options_mut(|o| o.fallback_theme = egui::Theme::Light);
    }
    let dark = ctx.theme() == egui::Theme::Dark;
    if force || style_changed || dark != IS_DARK.load(Ordering::Relaxed) {
        IS_DARK.store(dark, Ordering::Relaxed);
        setup_style(ctx);
    }
}

fn setup_style(ctx: &egui::Context) {
    let k = t();
    let mut v = if k.dark { egui::Visuals::dark() } else { egui::Visuals::light() };
    let r = CornerRadius::same(6);
    v.panel_fill = k.lightbox_bg;
    v.window_fill = k.surface;
    v.window_stroke = Stroke::new(1.0, k.border);
    v.window_corner_radius = CornerRadius::same(10);
    v.menu_corner_radius = CornerRadius::same(8);
    v.extreme_bg_color = if k.dark { Color32::from_rgb(24, 25, 28) } else { Color32::WHITE };
    v.faint_bg_color = k.header_bg;
    v.code_bg_color = k.header_bg;
    v.hyperlink_color = k.accent;
    v.override_text_color = None;
    v.selection.bg_fill = k.select_bg;
    v.selection.stroke = Stroke::new(1.0, k.select_border);

    v.widgets.noninteractive.bg_fill = k.surface;
    v.widgets.noninteractive.weak_bg_fill = k.surface;
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, k.border);
    v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, k.text);
    v.widgets.noninteractive.corner_radius = r;

    v.widgets.inactive.weak_bg_fill = k.surface;
    v.widgets.inactive.bg_fill = if k.dark { Color32::from_rgb(70, 73, 84) } else { Color32::from_rgb(200, 205, 216) };
    v.widgets.inactive.bg_stroke = Stroke::new(1.0, k.border);
    v.widgets.inactive.fg_stroke = Stroke::new(1.0, k.text);
    v.widgets.inactive.corner_radius = r;

    v.widgets.hovered.weak_bg_fill = k.surface_hover;
    v.widgets.hovered.bg_fill = k.surface_hover;
    v.widgets.hovered.bg_stroke = Stroke::new(1.0, k.accent);
    v.widgets.hovered.fg_stroke = Stroke::new(1.5, k.text_strong);
    v.widgets.hovered.corner_radius = r;

    v.widgets.active.weak_bg_fill = k.select_bg;
    v.widgets.active.bg_fill = k.select_bg;
    v.widgets.active.bg_stroke = Stroke::new(1.0, k.accent);
    v.widgets.active.fg_stroke = Stroke::new(1.5, k.text_strong);
    v.widgets.active.corner_radius = r;

    v.widgets.open.weak_bg_fill = k.surface_hover;
    v.widgets.open.bg_fill = k.surface_hover;
    v.widgets.open.bg_stroke = Stroke::new(1.0, k.border_strong);
    v.widgets.open.fg_stroke = Stroke::new(1.0, k.text_strong);
    v.widgets.open.corner_radius = r;

    v.popup_shadow = egui::Shadow {
        offset: [0, 4],
        blur: 16,
        spread: 0,
        color: Color32::from_black_alpha(if k.dark { 120 } else { 40 }),
    };

    ctx.set_visuals(v);
    ctx.all_styles_mut(|s| {
        s.spacing.item_spacing = egui::vec2(6.0, 5.0);
        s.spacing.button_padding = egui::vec2(10.0, 4.0);
        s.text_styles.insert(egui::TextStyle::Body, egui::FontId::proportional(13.5));
        s.text_styles.insert(egui::TextStyle::Button, egui::FontId::proportional(13.5));
    });
}

/// The Material Symbols subset behind `crate::icons` (see `assets/fonts/`).
pub const ICON_FONT: &[u8] = include_bytes!("../assets/fonts/MaterialSymbolsOutlined-subset.ttf");

/// egui's fonts, with the bundled monospace font (Hack) as a fallback for ordinary text too (it
/// has symbols such as › − × that the main font lacks), and the icon font (`crate::icons`) last.
pub fn set_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    let icons = egui::FontData::from_static(ICON_FONT).tweak(egui::FontTweak {
        scale: 1.15,
        y_offset_factor: 0.12,
        ..Default::default()
    });
    fonts.font_data.insert("icons".to_owned(), std::sync::Arc::new(icons));
    // a language with a script the built-in fonts lack (Chinese, Japanese, Korean) uses a font of
    // the system, after the others, so that Latin text keeps its look
    let script_font = crate::i18n::first_existing(&crate::i18n::script_font_candidates(&crate::i18n::current_language()))
        .and_then(|(path, index)| std::fs::read(path).ok().map(|bytes| (bytes, index)));
    if let Some((bytes, index)) = script_font {
        let mut data = egui::FontData::from_owned(bytes);
        data.index = index;
        fonts.font_data.insert("script".to_owned(), std::sync::Arc::new(data));
    }
    let has_script_font = fonts.font_data.contains_key("script");
    if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
        if !family.iter().any(|f| f == "Hack") {
            family.push("Hack".to_owned());
        }
        if has_script_font {
            family.push("script".to_owned());
        }
        family.push("icons".to_owned());
    }
    if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Monospace) {
        family.push("icons".to_owned());
    }
    ctx.set_fonts(fonts);
}

#[cfg(test)]
mod glyph_tests {
    use ab_glyph::{Font, FontRef};
    use unicode_properties::UnicodeEmoji;

    /// Every character in the user interface's text must be in the text fonts egui ships with or
    /// in the icon font (see `set_fonts`). Anything else shows as an empty box on some systems,
    /// as ☰ and ＋ did on Windows. Emoji are not used either: they come from each system's own
    /// colour emoji font and look different everywhere; icons come from `crate::icons`. Scans
    /// the string literals of the sources, up to each file's tests.
    #[test]
    fn every_ui_character_is_in_the_fonts() {
        let text_fonts: Vec<FontRef> = [epaint_default_fonts::UBUNTU_LIGHT, epaint_default_fonts::HACK_REGULAR]
            .iter()
            .filter_map(|b| FontRef::try_from_slice(b).ok())
            .collect();
        assert_eq!(text_fonts.len(), 2);
        let icons = FontRef::try_from_slice(super::ICON_FONT).unwrap();
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut missing: Vec<(String, char)> = vec![];
        for entry in std::fs::read_dir(&dir).unwrap().flatten() {
            let path = entry.path();
            if path.extension().is_none_or(|e| e != "rs") {
                continue;
            }
            let text = std::fs::read_to_string(&path).unwrap();
            let code = text.split("#[cfg(test)]").next().unwrap_or("");
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            for line in code.lines() {
                let line = line.trim_start();
                if line.starts_with("//") {
                    continue;
                }
                // characters inside "…" on this line
                let mut inside = false;
                let mut prev = ' ';
                for c in line.chars() {
                    if c == '"' && prev != '\\' {
                        inside = !inside;
                    } else if inside && !c.is_ascii() && !c.is_whitespace() {
                        let in_text = text_fonts.iter().any(|f| f.glyph_id(c).0 != 0);
                        // emoji are allowed only where the main font draws them itself (©)
                        let emoji_ok = !c.is_emoji_char() || text_fonts[0].glyph_id(c).0 != 0;
                        let found = (in_text && emoji_ok) || icons.glyph_id(c).0 != 0;
                        if !found && !missing.iter().any(|(_, m)| *m == c) {
                            missing.push((name.clone(), c));
                        }
                    }
                    prev = c;
                }
            }
        }
        // the translations (i18n/<language>/albumcraft.ftl): every character but in # comments
        let i18n = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("i18n");
        for entry in std::fs::read_dir(&i18n).unwrap().flatten() {
            let path = entry.path().join("albumcraft.ftl");
            // Chinese, Japanese and Korean are drawn with a font of the system, not these
            if !crate::i18n::script_font_candidates(entry.file_name().to_string_lossy().split('-').next().unwrap_or("")).is_empty() {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(&path) else { continue };
            let name = format!("i18n/{}", entry.file_name().to_string_lossy());
            for line in text.lines().filter(|l| !l.trim_start().starts_with('#')) {
                for c in line.chars().filter(|c| !c.is_ascii() && !c.is_whitespace()) {
                    let emoji_ok = !c.is_emoji_char() || text_fonts[0].glyph_id(c).0 != 0;
                    let found = (text_fonts.iter().any(|f| f.glyph_id(c).0 != 0) && emoji_ok) || icons.glyph_id(c).0 != 0;
                    if !found && !missing.iter().any(|(_, m)| *m == c) {
                        missing.push((name.clone(), c));
                    }
                }
            }
        }
        let list: Vec<String> = missing.iter().map(|(f, c)| format!("{c} (U+{:04X}) in {f}", *c as u32)).collect();
        assert!(missing.is_empty(), "characters the fonts don't have, or emoji (use crate::icons):\n{}", list.join("\n"));
    }

    /// Every constant of `crate::icons` is in the bundled icon font.
    #[test]
    fn every_icon_is_in_the_icon_font() {
        let icons = FontRef::try_from_slice(super::ICON_FONT).unwrap();
        for (name, s) in crate::icons::ALL {
            let mut chars = s.chars();
            let (Some(c), None) = (chars.next(), chars.next()) else { panic!("{name} is not one character") };
            assert_ne!(icons.glyph_id(c).0, 0, "{name} (U+{:04X}) is missing: run assets/fonts/subset_icons.py", c as u32);
        }
    }
}
