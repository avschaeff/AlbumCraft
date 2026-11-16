//! The sidebar's folders as they are on disk (`Store.folder_view` "tree"): one tree per library
//! folder, with the folders in between that hold no pictures themselves.

/// A folder of the tree.
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    /// The name shown (for a library folder: its whole path's last part).
    pub name: String,
    pub path: String,
    /// Its place in `App.folders` when it holds pictures itself.
    pub folder: Option<usize>,
    /// Pictures in it and everything under it.
    pub total: usize,
    pub children: Vec<Node>,
}

/// The parts of `path` under `root` (None: not under it).
fn parts_under<'a>(path: &'a str, root: &str) -> Option<Vec<&'a str>> {
    let root_trim = root.trim_end_matches(['/', '\\']);
    let rest = path.strip_prefix(root_trim)?;
    if !(rest.is_empty() || rest.starts_with(['/', '\\'])) {
        return None;
    }
    Some(rest.split(['/', '\\']).filter(|s| !s.is_empty()).collect())
}

fn last_part(path: &str) -> String {
    path.trim_end_matches(['/', '\\']).rsplit(['/', '\\']).find(|s| !s.is_empty()).unwrap_or(path).to_string()
}

/// The tree of `folders` (path and number of pictures, by their index) under each of `roots`, in
/// the order of `roots`; folders of no root come last, each on its own. Children are in name
/// order.
pub fn build(roots: &[String], folders: &[(&str, usize)]) -> Vec<Node> {
    let mut out: Vec<Node> = roots.iter().map(|r| Node { name: last_part(r), path: r.clone(), folder: None, total: 0, children: vec![] }).collect();
    let mut loose = vec![];
    for (fi, &(path, n)) in folders.iter().enumerate() {
        // the longest root that holds it (roots never hold one another, but be safe)
        let found = roots.iter().enumerate().filter_map(|(ri, r)| parts_under(path, r).map(|p| (ri, p))).max_by_key(|(ri, _)| roots[*ri].len());
        let Some((ri, parts)) = found else {
            loose.push(Node { name: last_part(path), path: path.to_string(), folder: Some(fi), total: n, children: vec![] });
            continue;
        };
        let sep = if path.contains('\\') && !path.contains('/') { '\\' } else { '/' };
        let mut node = &mut out[ri];
        node.total += n;
        for part in parts {
            let at = match node.children.iter().position(|c| c.name == part) {
                Some(i) => i,
                None => {
                    let child_path = format!("{}{sep}{part}", node.path.trim_end_matches(['/', '\\']));
                    node.children.push(Node { name: part.to_string(), path: child_path, folder: None, total: 0, children: vec![] });
                    node.children.len() - 1
                }
            };
            node = &mut node.children[at];
            node.total += n;
        }
        node.folder = Some(fi);
    }
    fn sort(nodes: &mut [Node]) {
        nodes.sort_by_key(|n| n.name.to_lowercase());
        for n in nodes {
            sort(&mut n.children);
        }
    }
    for root in &mut out {
        sort(&mut root.children);
    }
    loose.sort_by_key(|n| n.name.to_lowercase());
    out.extend(loose);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn folders_as_on_disk() {
        let roots = vec!["/p/Photos".to_string(), "/media/card/".to_string()];
        let folders = [("/p/Photos/2009/Summer", 9), ("/p/Photos", 2), ("/p/Photos/2009/Autumn", 4), ("/p/Photos/2010", 1), ("/media/card/DCIM", 3), ("/elsewhere/x", 5)];
        let tree = build(&roots, &folders);
        assert_eq!(tree.len(), 3);
        let photos = &tree[0];
        assert_eq!((photos.name.as_str(), photos.folder, photos.total), ("Photos", Some(1), 16));
        let names: Vec<&str> = photos.children.iter().map(|c| c.name.as_str()).collect();
        assert_eq!(names, ["2009", "2010"]);
        // a folder without pictures of its own, between the library folder and the ones with
        let y2009 = &photos.children[0];
        assert_eq!((y2009.folder, y2009.total, y2009.path.as_str()), (None, 13, "/p/Photos/2009"));
        let inner: Vec<(&str, Option<usize>)> = y2009.children.iter().map(|c| (c.name.as_str(), c.folder)).collect();
        assert_eq!(inner, [("Autumn", Some(2)), ("Summer", Some(0))]);
        assert_eq!((tree[1].name.as_str(), tree[1].total, tree[1].children[0].folder), ("card", 3, Some(4)));
        // not under a library folder: on its own at the end
        assert_eq!((tree[2].name.as_str(), tree[2].folder), ("x", Some(5)));
    }

    #[test]
    fn windows_paths() {
        let tree = build(&["C:\\Pics".to_string()], &[("C:\\Pics\\2020\\Trip", 3)]);
        assert_eq!(tree[0].children[0].children[0].path, "C:\\Pics\\2020\\Trip");
        // a name that only starts like the library folder is not inside it
        let tree = build(&["/p/a".to_string()], &[("/p/ab", 1)]);
        assert_eq!((tree.len(), tree[0].total, tree[1].folder), (2, 0, Some(0)));
    }
}
