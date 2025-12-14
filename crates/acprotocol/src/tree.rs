use serde_json::Value;
use std::collections::HashSet;

/// A generic tree node that can represent JSON structures hierarchically
#[derive(Clone)]
pub struct TreeNode {
    pub key: String,
    pub value: String,
    pub children: Vec<TreeNode>,
    pub expanded: bool,
}

impl TreeNode {
    /// Create a tree node from a JSON value
    pub fn from_json(key: &str, value: &Value) -> Self {
        let (value_str, children) = match value {
            Value::Object(obj) => {
                let val = "{...}".to_string();
                let children: Vec<TreeNode> =
                    obj.iter().map(|(k, v)| TreeNode::from_json(k, v)).collect();
                (val, children)
            }
            Value::Array(arr) => {
                let val = format!("[{}]", arr.len());
                let children: Vec<TreeNode> = arr
                    .iter()
                    .enumerate()
                    .map(|(i, v)| TreeNode::from_json(&format!("[{}]", i), v))
                    .collect();
                (val, children)
            }
            _ => (value.to_string(), Vec::new()),
        };

        TreeNode {
            key: key.to_string(),
            value: value_str,
            children,
            expanded: false,
        }
    }

    /// Generate display lines with tree structure, respecting expanded/collapsed state
    pub fn get_display_lines(
        &self,
        depth: usize,
        expanded_set: &HashSet<String>,
        path: String,
    ) -> Vec<(String, String)> {
        let mut lines = Vec::new();
        let current_path = if path.is_empty() {
            self.key.clone()
        } else {
            format!("{}.{}", path, self.key)
        };

        let prefix = "  ".repeat(depth);
        let expanded = expanded_set.contains(&current_path);
        let toggle = if !self.children.is_empty() {
            if expanded { "▼" } else { "▶" }
        } else {
            " "
        };

        lines.push((
            format!("{}{} {}: {}", prefix, toggle, self.key, self.value),
            current_path.clone(),
        ));

        if expanded && !self.children.is_empty() {
            for child in &self.children {
                lines.extend(child.get_display_lines(
                    depth + 1,
                    expanded_set,
                    current_path.clone(),
                ));
            }
        }

        lines
    }
}

/// Collect all paths that have child nodes (expandable paths)
pub fn collect_all_expandable_paths(node: &TreeNode, path: String, expanded: &mut HashSet<String>) {
    let current_path = if path.is_empty() {
        node.key.clone()
    } else {
        format!("{}.{}", path, node.key)
    };

    // Add current node if it has children
    if !node.children.is_empty() {
        expanded.insert(current_path.clone());
    }

    // Recursively process children
    for child in &node.children {
        collect_all_expandable_paths(child, current_path.clone(), expanded);
    }
}
