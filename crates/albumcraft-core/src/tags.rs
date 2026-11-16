//! The library's tags as a whole: their counts, their hierarchy (`Store.tag_parents`: a tag
//! under another, "Church" under "Buildings") and the changes the tag manager makes (rename,
//! merge, delete, move under another tag). Tags are compared ignoring letter case everywhere;
//! the spelling kept is the one already in the library.
//!
//! A tag chosen in a search also finds the pictures of the tags under it (`family`).

use crate::library::Store;
use std::collections::{BTreeMap, HashMap, HashSet};

/// The way tags are compared.
pub fn key(tag: &str) -> String {
    tag.trim().to_lowercase()
}

/// A tag of the tree: its pictures, and with those of the tags under it.
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub name: String,
    /// Pictures with this tag itself.
    pub own: usize,
    /// Pictures with this tag or one under it (each counted once).
    pub total: usize,
    pub children: Vec<Node>,
}

/// Every tag in the library with how many pictures have it, by `key`: (spelling, count).
/// Tags only named in the hierarchy (a group with no pictures of its own) count 0. When pictures
/// spell a tag differently, the first spelling in sort order is the one kept.
pub fn counts(store: &Store) -> HashMap<String, (String, usize)> {
    let mut out: HashMap<String, (String, usize)> = HashMap::new();
    for m in store.meta.values() {
        // a picture with the same tag twice in different case counts once
        let mut seen = HashSet::new();
        for t in &m.tags {
            let k = key(t);
            if k.is_empty() || !seen.insert(k.clone()) {
                continue;
            }
            // the same spelling every time when pictures differ in case ("Church" before "church"),
            // whatever order the pictures come in
            let e = out.entry(k).or_insert_with(|| (t.trim().to_string(), 0));
            if t.trim() < e.0.as_str() {
                e.0 = t.trim().to_string();
            }
            e.1 += 1;
        }
    }
    for (child, parent) in &store.tag_parents {
        for t in [child, parent] {
            let k = key(t);
            if !k.is_empty() {
                out.entry(k).or_insert_with(|| (t.trim().to_string(), 0));
            }
        }
    }
    out
}

/// The parent of each tag, by `key` (tags of the library only; a missing parent is the top).
fn parent_keys(store: &Store) -> HashMap<String, String> {
    store.tag_parents.iter().filter(|(_, p)| !key(p).is_empty()).map(|(c, p)| (key(c), key(p))).collect()
}

/// `tag` and every tag under it, by `key`.
pub fn family(store: &Store, tag: &str) -> HashSet<String> {
    family_in(&parent_keys(store), tag)
}

fn family_in(parents: &HashMap<String, String>, tag: &str) -> HashSet<String> {
    let mut children: HashMap<&str, Vec<&str>> = HashMap::new();
    for (c, p) in parents {
        children.entry(p.as_str()).or_default().push(c.as_str());
    }
    let mut out = HashSet::new();
    let mut todo = vec![key(tag)];
    while let Some(t) = todo.pop() {
        if !out.insert(t.clone()) {
            continue;
        }
        if let Some(cs) = children.get(t.as_str()) {
            todo.extend(cs.iter().map(|c| c.to_string()));
        }
    }
    out
}

/// The tags as a tree, each level in name order.
pub fn tree(store: &Store) -> Vec<Node> {
    let counts = counts(store);
    let parents = parent_keys(store);
    // pictures per tag family, each picture once
    let mut by_tag: HashMap<String, HashSet<&str>> = HashMap::new();
    for (k, m) in &store.meta {
        for t in &m.tags {
            by_tag.entry(key(t)).or_default().insert(k.as_str());
        }
    }
    let mut children: HashMap<String, Vec<String>> = HashMap::new();
    let mut tops = vec![];
    for k in counts.keys() {
        match parents.get(k).filter(|p| counts.contains_key(*p) && *p != k) {
            Some(p) => children.entry(p.clone()).or_default().push(k.clone()),
            None => tops.push(k.clone()),
        }
    }
    fn build(k: &str, counts: &HashMap<String, (String, usize)>, children: &HashMap<String, Vec<String>>, by_tag: &HashMap<String, HashSet<&str>>, seen: &mut HashSet<String>) -> Node {
        seen.insert(k.to_string());
        let mut kids: Vec<Node> = children.get(k).into_iter().flatten().filter(|c| !seen.contains(*c)).cloned().collect::<Vec<_>>().iter().map(|c| build(c, counts, children, by_tag, seen)).collect();
        kids.sort_by_key(|n| n.name.to_lowercase());
        let (name, own) = counts.get(k).cloned().unwrap_or_default();
        let mut pictures: HashSet<&str> = by_tag.get(k).cloned().unwrap_or_default();
        let mut todo: Vec<&str> = children.get(k).into_iter().flatten().map(String::as_str).collect();
        let mut done = HashSet::new();
        while let Some(c) = todo.pop() {
            if done.insert(c) {
                pictures.extend(by_tag.get(c).into_iter().flatten().copied());
                todo.extend(children.get(c).into_iter().flatten().map(String::as_str));
            }
        }
        Node { name, own, total: pictures.len(), children: kids }
    }
    let mut seen = HashSet::new();
    let mut out: Vec<Node> = tops.iter().map(|k| build(k, &counts, &children, &by_tag, &mut seen)).collect();
    // a loop of parents written by hand: its tags still show, at the top
    for k in counts.keys() {
        if !seen.contains(k) {
            out.push(build(k, &counts, &children, &by_tag, &mut seen));
        }
    }
    out.sort_by_key(|n| n.name.to_lowercase());
    out
}

/// The spelling of `tag` in the library, if it is there.
fn spelling(store: &Store, tag: &str) -> Option<String> {
    counts(store).remove(&key(tag)).map(|(name, _)| name)
}

/// Rename `from` to `to` on every picture and in the hierarchy. When `to` is a tag already, the
/// two become one (a merge): pictures that had both keep it once. Tags under `from` are then
/// under `to`. Returns the keys of the pictures changed.
pub fn rename(store: &mut Store, from: &str, to: &str) -> Result<Vec<String>, String> {
    let to = to.trim();
    if to.is_empty() {
        return Err(crate::msg!("tag-empty-name", "A tag needs a name.").into());
    }
    if to.contains(',') {
        return Err(crate::msg!("tag-no-comma", "A tag can't contain a comma.").into());
    }
    let (fk, tk) = (key(from), key(to));
    if spelling(store, from).is_none() {
        return Err(crate::msg!("tag-not-found", "There is no tag {tag}.", tag = from.to_string()).into());
    }
    // merging into a tag under it would put it under itself
    if fk != tk && family(store, from).contains(&tk) {
        return Err(crate::msg!("tag-into-own-child", "{tag} is under {from}: move it out first.", tag = to.to_string(), from = from.to_string()).into());
    }
    // the existing spelling when it is a merge, else the new one (also for a change of case)
    let target = if fk == tk { to.to_string() } else { spelling(store, to).unwrap_or_else(|| to.to_string()) };
    let mut changed = vec![];
    for (k, m) in store.meta.iter_mut() {
        let had = m.tags.iter().any(|t| key(t) == fk);
        if !had {
            continue;
        }
        let mut out: Vec<String> = vec![];
        for t in m.tags.drain(..) {
            let t = if key(&t) == fk { target.clone() } else { t };
            if !out.iter().any(|o| key(o) == key(&t)) {
                out.push(t);
            }
        }
        m.tags = out;
        m.not_tags.retain(|t| key(t) != fk);
        changed.push(k.clone());
    }
    // the hierarchy: its own place (kept unless the other one has one), and the tags under it
    let old: BTreeMap<String, String> = std::mem::take(&mut store.tag_parents);
    let own_parent = old.iter().find(|(c, _)| key(c) == fk).map(|(_, p)| p.clone());
    let target_parent = old.iter().find(|(c, _)| key(c) == tk && fk != tk).map(|(_, p)| p.clone());
    for (c, p) in old {
        if key(&c) == fk || key(&c) == tk {
            continue;
        }
        let p = if key(&p) == fk { target.clone() } else { p };
        store.tag_parents.insert(c, p);
    }
    if let Some(p) = target_parent.or(own_parent).filter(|p| key(p) != tk && key(p) != fk) {
        store.tag_parents.insert(target.clone(), p);
    } else if counts(store).get(&tk).is_none_or(|(_, n)| *n == 0) && changed.is_empty() {
        // a group with no pictures keeps existing under its new name
        store.tag_parents.insert(target.clone(), String::new());
    }
    changed.sort();
    Ok(changed)
}

/// Take `tag` off every picture and out of the hierarchy; the tags under it go to its parent.
/// Returns the keys of the pictures changed.
pub fn delete(store: &mut Store, tag: &str) -> Vec<String> {
    let k = key(tag);
    let mut changed = vec![];
    for (pk, m) in store.meta.iter_mut() {
        if m.tags.iter().any(|t| key(t) == k) {
            m.tags.retain(|t| key(t) != k);
            changed.push(pk.clone());
        }
    }
    let parent = store.tag_parents.iter().find(|(c, _)| key(c) == k).map(|(_, p)| p.clone()).unwrap_or_default();
    store.tag_parents.retain(|c, _| key(c) != k);
    for p in store.tag_parents.values_mut() {
        if key(p) == k {
            *p = parent.clone();
        }
    }
    changed.sort();
    changed
}

/// Put `tag` under `parent` (None or empty: at the top). Refused when `parent` is `tag` itself
/// or under it.
pub fn set_parent(store: &mut Store, tag: &str, parent: Option<&str>) -> Result<(), String> {
    let name = spelling(store, tag).ok_or_else(|| String::from(crate::msg!("tag-not-found", "There is no tag {tag}.", tag = tag.to_string())))?;
    let parent = parent.map(str::trim).filter(|p| !p.is_empty());
    if let Some(p) = parent
        && family(store, tag).contains(&key(p))
    {
        return Err(crate::msg!("tag-under-itself", "A tag can't go under itself or a tag under it.").into());
    }
    let parent = parent.map(|p| spelling(store, p).unwrap_or_else(|| p.to_string()));
    store.tag_parents.retain(|c, _| key(c) != key(&name));
    let has_pictures = counts(store).get(&key(&name)).is_some_and(|(_, n)| *n > 0);
    match parent {
        Some(p) => {
            // the parent stays known even with no pictures of its own
            if !store.tag_parents.keys().any(|c| key(c) == key(&p)) && counts(store).get(&key(&p)).is_none_or(|(_, n)| *n == 0) {
                store.tag_parents.insert(p.clone(), String::new());
            }
            store.tag_parents.insert(name, p);
        }
        None if !has_pictures => {
            store.tag_parents.insert(name, String::new());
        }
        None => {}
    }
    Ok(())
}

/// A new tag with no pictures yet (a group to put others under), under `parent` if given.
pub fn create(store: &mut Store, tag: &str, parent: Option<&str>) -> Result<(), String> {
    let tag = tag.trim();
    if tag.is_empty() {
        return Err(crate::msg!("tag-empty-name", "A tag needs a name.").into());
    }
    if tag.contains(',') {
        return Err(crate::msg!("tag-no-comma", "A tag can't contain a comma.").into());
    }
    if spelling(store, tag).is_some() {
        return Err(crate::msg!("tag-exists", "There is a tag {tag} already.", tag = tag.to_string()).into());
    }
    store.tag_parents.insert(tag.to_string(), String::new());
    if parent.is_some_and(|p| !p.trim().is_empty()) {
        set_parent(store, tag, parent)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store(pictures: &[(&str, &[&str])]) -> Store {
        let mut s = Store::default();
        for (k, tags) in pictures {
            s.meta.entry(k.to_string()).or_default().tags = tags.iter().map(|t| t.to_string()).collect();
        }
        s
    }

    #[test]
    fn a_parent_finds_the_pictures_of_the_tags_under_it() {
        let mut s = store(&[("a", &["Church"]), ("b", &["Temple", "church"]), ("c", &["Beach"])]);
        create(&mut s, "Buildings", None).unwrap();
        set_parent(&mut s, "church", Some("Buildings")).unwrap();
        set_parent(&mut s, "Temple", Some("buildings")).unwrap();
        assert_eq!(family(&s, "Buildings"), ["buildings", "church", "temple"].iter().map(|t| t.to_string()).collect());
        let t = tree(&s);
        let names: Vec<&str> = t.iter().map(|n| n.name.as_str()).collect();
        assert_eq!(names, ["Beach", "Buildings"]);
        let b = &t[1];
        assert_eq!((b.own, b.total), (0, 2), "b has both tags and counts once");
        assert_eq!(b.children.iter().map(|c| (c.name.as_str(), c.own)).collect::<Vec<_>>(), [("Church", 2), ("Temple", 1)]);
        // never under itself or under a tag under it
        assert!(set_parent(&mut s, "Buildings", Some("Church")).is_err());
        assert!(set_parent(&mut s, "Church", Some("church")).is_err());
    }

    #[test]
    fn merging_and_renaming() {
        let mut s = store(&[("a", &["cathedral"]), ("b", &["church", "Cathedral"]), ("c", &["sea"])]);
        s.meta.get_mut("c").unwrap().not_tags = vec!["cathedral".into()];
        create(&mut s, "Buildings", None).unwrap();
        set_parent(&mut s, "church", Some("Buildings")).unwrap();
        let changed = rename(&mut s, "Cathedral", "Church").unwrap();
        assert_eq!(changed, ["a", "b"]);
        assert_eq!(s.meta["a"].tags, ["church"], "the library's spelling");
        assert_eq!(s.meta["b"].tags, ["church"], "once");
        assert_eq!(family(&s, "Buildings").len(), 2, "church stays under Buildings");
        // a plain rename, also of the letter case
        rename(&mut s, "church", "Churches").unwrap();
        assert_eq!(s.meta["a"].tags, ["Churches"]);
        assert!(family(&s, "buildings").contains("churches"));
        rename(&mut s, "Buildings", "Architecture").unwrap();
        assert!(family(&s, "Architecture").contains("churches"), "the tags under it follow");
        assert!(rename(&mut s, "Architecture", "Churches").is_err(), "into a tag under it");
        assert!(rename(&mut s, "nothing", "x").is_err());
        assert!(rename(&mut s, "sea", "a, b").is_err());
    }

    #[test]
    fn deleting_moves_the_tags_under_it_up() {
        let mut s = store(&[("a", &["Church", "sea"]), ("b", &["Buildings"])]);
        create(&mut s, "Places", None).unwrap();
        set_parent(&mut s, "Buildings", Some("Places")).unwrap();
        set_parent(&mut s, "Church", Some("Buildings")).unwrap();
        assert_eq!(delete(&mut s, "buildings"), ["b"]);
        assert!(s.meta["b"].tags.is_empty());
        assert!(family(&s, "Places").contains("church"));
        assert!(!counts(&s).contains_key("buildings"));
    }
}
