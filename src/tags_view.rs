//! Tags in the window: the library's tags as chips to put on or take off pictures
//! (`tag_picker`, in the Tags dialog and the editor's Tags section), the top bar's Tags chip
//! with the tag tree (`tags_menu`: a click shows that tag's pictures, and those of the tags
//! under it), and the tag manager (`Dialog::ManageTags`: rename, merge, delete, put a tag under
//! another). The rules are core's `tags`.

use crate::app::*;
use crate::i18n::tr;
use crate::icons;
use crate::tags::{self, Node};
use eframe::egui::{self, RichText};
use std::sync::Arc;

/// The tag manager's state (`Dialog::ManageTags`).
#[derive(Clone, Debug, Default)]
pub struct ManageTags {
    /// The tag shown on the right.
    pub selected: Option<String>,
    /// Its name being edited.
    pub name: String,
    /// The tag to merge it into.
    pub merge_into: String,
    /// A new tag's name.
    pub new_tag: String,
    /// Shown tags contain this.
    pub filter: String,
    /// Why the last change was refused.
    pub error: String,
}

/// How many of the pictures have a tag.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Has {
    None,
    Some,
    All,
}

/// The tree's rows in order: (depth, node).
fn flatten<'a>(nodes: &'a [Node], depth: usize, out: &mut Vec<(usize, &'a Node)>) {
    for n in nodes {
        out.push((depth, n));
        flatten(&n.children, depth + 1, out);
    }
}

impl App {
    /// Every tag with its number of pictures, most used first (cached until the notes change).
    pub fn tag_counts(&mut self) -> Arc<Vec<(String, usize)>> {
        if let Some((rev, c)) = &self.tag_counts_cache
            && *rev == self.list_rev
        {
            return c.clone();
        }
        let mut list: Vec<(String, usize)> = tags::counts(&self.store).into_values().collect();
        list.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.to_lowercase().cmp(&b.0.to_lowercase())));
        let list = Arc::new(list);
        self.tag_counts_cache = Some((self.list_rev, list.clone()));
        list
    }

    /// The tags as a tree (cached until the notes change).
    pub fn tag_tree(&mut self) -> Arc<Vec<Node>> {
        if let Some((rev, t)) = &self.tag_tree_cache
            && *rev == self.list_rev
        {
            return t.clone();
        }
        let t = Arc::new(tags::tree(&self.store));
        self.tag_tree_cache = Some((self.list_rev, t.clone()));
        t
    }

    /// Put `tag` on these pictures (`on`) or take it off.
    pub fn set_tag(&mut self, ids: &[usize], tag: &str, on: bool) {
        let tag = tag.trim();
        if tag.is_empty() {
            return;
        }
        // the library's spelling when it has the tag already
        let spelt = tags::counts(&self.store).remove(&tags::key(tag)).map(|(s, _)| s).unwrap_or_else(|| tag.to_string());
        for &id in ids {
            if id >= self.photos.len() {
                continue;
            }
            let m = self.meta_mut(id);
            if on {
                if !m.tags.iter().any(|t| tags::key(t) == tags::key(&spelt)) {
                    m.tags.push(spelt.clone());
                }
                m.not_tags.retain(|t| tags::key(t) != tags::key(&spelt));
            } else {
                m.tags.retain(|t| tags::key(t) != tags::key(&spelt));
            }
        }
    }

    fn has_tag(&self, ids: &[usize], tag: &str) -> Has {
        let k = tags::key(tag);
        let n = ids.iter().filter(|&&id| self.meta(id).is_some_and(|m| m.tags.iter().any(|t| tags::key(t) == k))).count();
        match n {
            0 => Has::None,
            n if n == ids.len() => Has::All,
            _ => Has::Some,
        }
    }

    /// The library's tags as chips, most used first, those containing `filter`, at most `max`:
    /// ticked when all of `ids` have it, a dash when some do (`skip_had`: tags all of them have
    /// are left out, they are shown elsewhere). A click puts the tag on all of them, or takes it
    /// off when they all have it.
    pub fn tag_picker(&mut self, ui: &mut egui::Ui, ids: &[usize], filter: &str, max: usize, skip_had: bool) {
        if ids.is_empty() {
            return;
        }
        let f = filter.trim().to_lowercase();
        let all = self.tag_counts();
        let shown: Vec<&(String, usize)> = all.iter().filter(|(t, n)| *n > 0 && (f.is_empty() || t.to_lowercase().contains(&f)) && !(skip_had && self.has_tag(ids, t) == Has::All)).collect();
        if shown.is_empty() {
            if all.is_empty() {
                ui.label(RichText::new(tr!("tags-none-yet")).size(12.0).color(t().text_weak));
            }
            return;
        }
        let mut toggle: Option<(String, bool)> = None;
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(6.0, 6.0);
            for (tag, n) in shown.iter().take(max) {
                let has = self.has_tag(ids, tag);
                let mark = match has {
                    Has::All => icons::CHECK,
                    Has::Some => icons::REMOVE,
                    Has::None => icons::ADD,
                };
                let text = RichText::new(format!("{mark} {tag}")).size(12.5).color(if has == Has::None { t().text } else { t().accent });
                let mut chip = egui::Button::new(text).corner_radius(12);
                chip = if has == Has::None { chip.fill(egui::Color32::TRANSPARENT) } else { chip.fill(t().select_bg) };
                let n = *n;
                let hint = match has {
                    Has::All => tr!("tags-chip-remove", n = n),
                    Has::Some => tr!("tags-chip-some", n = n),
                    Has::None => tr!("tags-chip-add", n = n),
                };
                if ui.add(chip).on_hover_text(hint).clicked() {
                    toggle = Some((tag.clone(), has != Has::All));
                }
            }
            if shown.len() > max {
                let n = shown.len() - max;
                ui.label(RichText::new(tr!("tags-more", n = n)).size(12.0).color(t().text_weak));
            }
        });
        if let Some((tag, on)) = toggle {
            self.set_tag(ids, &tag, on);
        }
    }

    /// The top bar's Tags popup: a filter, the tag tree with the pictures of each (a click shows
    /// them), and the tag manager.
    pub fn tags_menu(&mut self, ui: &mut egui::Ui) {
        ui.set_min_width(240.0);
        let tree = self.tag_tree();
        if tree.is_empty() {
            ui.label(RichText::new(tr!("tags-none-yet")).size(12.5).color(t().text_weak));
        } else {
            ui.add(egui::TextEdit::singleline(&mut self.tag_filter).hint_text(tr!("tags-filter")).desired_width(f32::INFINITY));
            ui.add_space(2.0);
            let f = self.tag_filter.trim().to_lowercase();
            let mut rows = vec![];
            flatten(&tree, 0, &mut rows);
            let current: Vec<String> = self.search_terms.iter().filter_map(|s| if let SearchTerm::Tag(t) = s { Some(tags::key(t)) } else { None }).collect();
            let mut chosen = None;
            egui::ScrollArea::vertical().max_height(360.0).show(ui, |ui| {
                for (depth, n) in rows {
                    // filtered: a flat list of the tags that match
                    let depth = if f.is_empty() { depth } else { 0 };
                    if !f.is_empty() && !n.name.to_lowercase().contains(&f) {
                        continue;
                    }
                    let on = current.contains(&tags::key(&n.name));
                    let text = format!("{}{}", "    ".repeat(depth), n.name);
                    let resp = ui.add(egui::Button::selectable(on, egui::Atoms::new((RichText::new(text), egui::Atom::grow(), RichText::new(crate::app::fmt_count(n.total)).size(12.0).color(t().text_weak)))).min_size(egui::vec2(ui.available_width(), 0.0)));
                    if resp.clicked() {
                        chosen = Some(n.name.clone());
                    }
                }
            });
            if let Some(tag) = chosen {
                // one tag at a time from here: it replaces the tag chosen before
                self.search_terms.retain(|s| !matches!(s, SearchTerm::Tag(_)));
                self.search_terms.push(SearchTerm::Tag(tag));
                self.tag_filter.clear();
                ui.close();
            }
        }
        ui.separator();
        if ui.add(egui::Button::new(crate::color_icons::atoms(ui.ctx(), format!("{}  {}", icons::TAG, tr!("tags-manage")), 13.5))).clicked() {
            self.open_tag_manager(None);
            ui.close();
        }
    }

    pub fn open_tag_manager(&mut self, selected: Option<String>) {
        let name = selected.clone().unwrap_or_default();
        self.dialog = Some(Dialog::ManageTags(ManageTags { selected, name, ..Default::default() }));
    }

    /// What an Undo of a change to `tag` puts back: the tags of the pictures that have it (or
    /// `also`) and the whole hierarchy.
    fn tag_undo(&self, tag: &str, also: Option<&str>) -> Undo {
        let ks: Vec<String> = std::iter::once(tag).chain(also).map(tags::key).collect();
        let meta = self.store.meta.iter().filter(|(_, m)| m.tags.iter().chain(&m.not_tags).any(|t| ks.contains(&tags::key(t)))).map(|(k, m)| (k.clone(), m.tags.clone(), m.not_tags.clone())).collect();
        Undo::Tags { meta, parents: self.store.tag_parents.clone() }
    }

    pub(crate) fn tag_manager_dialog(&mut self, ctx: &egui::Context, mut st: ManageTags) {
        let mut close = false;
        let screen = ctx.content_rect().size();
        let size = egui::vec2((screen.x - 80.0).clamp(420.0, 760.0), (screen.y - 140.0).clamp(260.0, 480.0));
        let tree = self.tag_tree();
        let counts = self.tag_counts();
        #[allow(deprecated)]
        let modal = egui::Modal::new(egui::Id::new("manage-tags")).show(ctx, |ui| {
            ui.set_width(size.x);
            ui.heading(tr!("tags-manage-title"));
            ui.add_space(6.0);
            ui.allocate_ui_with_layout(size, egui::Layout::left_to_right(egui::Align::Min), |ui| {
                ui.set_min_size(size);
                let left = egui::vec2(size.x * 0.4, size.y);
                ui.allocate_ui_with_layout(left, egui::Layout::top_down(egui::Align::Min), |ui| {
                    ui.set_min_size(left);
                    ui.add(egui::TextEdit::singleline(&mut st.filter).hint_text(tr!("tags-filter")).desired_width(left.x));
                    ui.add_space(4.0);
                    let f = st.filter.trim().to_lowercase();
                    let mut rows = vec![];
                    flatten(&tree, 0, &mut rows);
                    egui::ScrollArea::vertical().id_salt("tag-tree").max_height(size.y - 80.0).auto_shrink([false, false]).show(ui, |ui| {
                        if rows.is_empty() {
                            ui.label(RichText::new(tr!("tags-none-yet")).size(12.5).color(t().text_weak));
                        }
                        for (depth, n) in rows {
                            let depth = if f.is_empty() { depth } else { 0 };
                            if !f.is_empty() && !n.name.to_lowercase().contains(&f) {
                                continue;
                            }
                            let on = st.selected.as_deref().is_some_and(|s| tags::key(s) == tags::key(&n.name));
                            let text = format!("{}{}", "    ".repeat(depth), n.name);
                            let atoms = egui::Atoms::new((RichText::new(text), egui::Atom::grow(), RichText::new(crate::app::fmt_count(n.total)).size(12.0).color(t().text_weak)));
                            if ui.add(egui::Button::selectable(on, atoms).min_size(egui::vec2(ui.available_width(), 0.0))).clicked() {
                                st.selected = Some(n.name.clone());
                                st.name = n.name.clone();
                                st.merge_into.clear();
                                st.error.clear();
                            }
                        }
                    });
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        let r = ui.add(egui::TextEdit::singleline(&mut st.new_tag).hint_text(tr!("tags-new")).desired_width(left.x - 70.0));
                        let enter = r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                        if (ui.add_enabled(!st.new_tag.trim().is_empty(), egui::Button::new(tr!("tags-create"))).on_hover_text(tr!("tags-create-hint")).clicked() || enter) && !st.new_tag.trim().is_empty() {
                            match tags::create(&mut self.store, &st.new_tag, None) {
                                Ok(()) => {
                                    self.dirty = true;
                                    st.selected = Some(st.new_tag.trim().to_string());
                                    st.name = st.new_tag.trim().to_string();
                                    st.new_tag.clear();
                                    st.error.clear();
                                }
                                Err(e) => st.error = e,
                            }
                        }
                    });
                });
                ui.separator();
                ui.vertical(|ui| {
                    let Some(sel) = st.selected.clone() else {
                        ui.label(RichText::new(tr!("tags-manage-intro")).color(t().text_weak));
                        return;
                    };
                    let node = {
                        let mut rows = vec![];
                        flatten(&tree, 0, &mut rows);
                        rows.into_iter().map(|(_, n)| n.clone()).find(|n| tags::key(&n.name) == tags::key(&sel))
                    };
                    let Some(node) = node else {
                        st.selected = None;
                        return;
                    };
                    ui.label(RichText::new(&node.name).size(17.0).strong().color(t().text_strong));
                    let (n, total) = (node.own, crate::app::fmt_count(node.total));
                    let sub = if node.children.is_empty() { tr!("tags-count", n = n) } else { tr!("tags-count-under", n = n, total = total) };
                    ui.label(RichText::new(sub).size(12.5).color(t().text_weak));
                    ui.add_space(8.0);
                    // rename (to an existing tag: a merge)
                    ui.label(tr!("tags-name"));
                    ui.horizontal(|ui| {
                        ui.add(egui::TextEdit::singleline(&mut st.name).desired_width(200.0));
                        let other = counts.iter().find(|(t, _)| tags::key(t) == tags::key(&st.name) && tags::key(t) != tags::key(&node.name)).map(|(t, _)| t.clone());
                        let changed = st.name.trim() != node.name;
                        let label = if other.is_some() { tr!("tags-merge") } else { tr!("tags-rename") };
                        if ui.add_enabled(changed && !st.name.trim().is_empty(), egui::Button::new(label)).clicked() {
                            let undo = self.tag_undo(&node.name, other.as_deref());
                            match tags::rename(&mut self.store, &node.name, &st.name) {
                                Ok(_) => {
                                    self.dirty = true;
                                    let new = other.clone().unwrap_or_else(|| st.name.trim().to_string());
                                    self.undoable(tr!("tags-renamed", from = node.name.as_str(), to = new.as_str()), undo);
                                    rename_in_search(&mut self.search_terms, &node.name, &new);
                                    st.selected = Some(new.clone());
                                    st.name = new;
                                    st.error.clear();
                                }
                                Err(e) => st.error = e,
                            }
                        }
                    });
                    if let Some(o) = counts.iter().find(|(t, _)| tags::key(t) == tags::key(&st.name) && tags::key(t) != tags::key(&node.name)) {
                        ui.label(RichText::new(tr!("tags-merge-note", tag = o.0.as_str())).size(12.0).color(t().text_weak));
                    }
                    ui.add_space(8.0);
                    // merge into another tag, chosen from the list
                    ui.label(tr!("tags-merge-into"));
                    ui.horizontal(|ui| {
                        let family = tags::family(&self.store, &node.name);
                        let shown = if st.merge_into.is_empty() { tr!("tags-choose") } else { st.merge_into.clone() };
                        egui::ComboBox::from_id_salt("merge-into").selected_text(shown).width(200.0).height(300.0).show_ui(ui, |ui| {
                            for (t_, _) in counts.iter().filter(|(t_, _)| !family.contains(&tags::key(t_))) {
                                ui.selectable_value(&mut st.merge_into, t_.clone(), t_);
                            }
                        });
                        if ui.add_enabled(!st.merge_into.is_empty(), egui::Button::new(tr!("tags-merge"))).on_hover_text(tr!("tags-merge-hint", tag = node.name.as_str())).clicked() {
                            let into = st.merge_into.clone();
                            let undo = self.tag_undo(&node.name, Some(&into));
                            match tags::rename(&mut self.store, &node.name, &into) {
                                Ok(_) => {
                                    self.dirty = true;
                                    self.undoable(tr!("tags-merged", from = node.name.as_str(), to = into.as_str()), undo);
                                    rename_in_search(&mut self.search_terms, &node.name, &into);
                                    st.selected = Some(into.clone());
                                    st.name = into;
                                    st.merge_into.clear();
                                    st.error.clear();
                                }
                                Err(e) => st.error = e,
                            }
                        }
                    });
                    ui.add_space(8.0);
                    // its place in the hierarchy
                    ui.label(tr!("tags-under"));
                    let family = tags::family(&self.store, &node.name);
                    let parent_now = self.store.tag_parents.iter().find(|(c, _)| tags::key(c) == tags::key(&node.name)).map(|(_, p)| p.clone()).filter(|p| !p.is_empty());
                    let mut parent = parent_now.clone();
                    egui::ComboBox::from_id_salt("tag-parent").selected_text(parent.clone().unwrap_or_else(|| tr!("tags-top"))).width(200.0).height(300.0).show_ui(ui, |ui| {
                        ui.selectable_value(&mut parent, None, tr!("tags-top"));
                        for (t_, _) in counts.iter().filter(|(t_, _)| !family.contains(&tags::key(t_))) {
                            ui.selectable_value(&mut parent, Some(t_.clone()), t_);
                        }
                    });
                    if parent.as_deref().map(tags::key) != parent_now.as_deref().map(tags::key) {
                        let undo = Undo::Tags { meta: vec![], parents: self.store.tag_parents.clone() };
                        match tags::set_parent(&mut self.store, &node.name, parent.as_deref()) {
                            Ok(()) => {
                                self.dirty = true;
                                self.undoable(tr!("tags-moved", tag = node.name.as_str()), undo);
                                st.error.clear();
                            }
                            Err(e) => st.error = e,
                        }
                    }
                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        if ui.add(egui::Button::new(crate::color_icons::atoms(ui.ctx(), format!("{}  {}", icons::PHOTOS, tr!("tags-show")), 13.5))).clicked() {
                            self.search_terms.retain(|s| !matches!(s, SearchTerm::Tag(_)));
                            self.search_terms.push(SearchTerm::Tag(node.name.clone()));
                            self.view = View::Library;
                            close = true;
                        }
                        if ui.button(RichText::new(tr!("tags-delete")).color(t().bad)).on_hover_text(tr!("tags-delete-hint", n = n)).clicked() {
                            let undo = self.tag_undo(&node.name, None);
                            tags::delete(&mut self.store, &node.name);
                            self.dirty = true;
                            self.search_terms.retain(|s| !matches!(s, SearchTerm::Tag(t) if tags::key(t) == tags::key(&node.name)));
                            self.undoable(tr!("tags-deleted", tag = node.name.as_str(), n = n), undo);
                            st.selected = None;
                            st.name.clear();
                        }
                    });
                    if !st.error.is_empty() {
                        ui.add_space(6.0);
                        ui.label(RichText::new(&st.error).color(t().bad));
                    }
                });
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(RichText::new(tr!("tags-manage-note")).size(12.0).color(t().text_weak));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(tr!("button-close")).clicked() {
                        close = true;
                    }
                });
            });
        });
        if self.dialog.is_none() && !close && !modal.should_close() {
            self.dialog = Some(Dialog::ManageTags(st));
        }
    }
}

/// A tag renamed or merged: a search chip of it follows.
fn rename_in_search(terms: &mut [SearchTerm], from: &str, to: &str) {
    for s in terms {
        if let SearchTerm::Tag(t) = s
            && tags::key(t) == tags::key(from)
        {
            *t = to.to_string();
        }
    }
}
