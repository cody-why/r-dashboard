/*
 * @Date: 2024-07-10 09:14:38
 * @LastEditTime: 2024-07-10 22:48:32
 */

pub mod icons;
mod index;
use dioxus::dioxus_core::Element;
pub use index::*;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MenuVo {
    pub id: i32,
    pub icon: String,
    pub name: String,
    pub path: String,
    pub api_url: String,
    pub menu_type: i32,
    pub parent_id: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MenuItem {
    pub id: i32,
    pub key: String,
    pub label: String,
    pub icon: Element,
    pub parent_id: i32,
    pub children: Vec<MenuItem>,
}

impl From<MenuVo> for MenuItem {
    fn from(item: MenuVo) -> Self {
        MenuItem {
            id: item.id,
            key: item.path,
            label: item.name,
            icon: icons::icon_blank(),
            parent_id: item.parent_id,
            children: Vec::new(),
        }
    }
}

pub fn build_tree(data: Vec<MenuItem>, pid: i32) -> Vec<MenuItem> {
    let mut result = Vec::new();

    for item in data.iter() {
        if item.parent_id == pid {
            let mut node = item.clone();
            node.children = build_tree(data.clone(), item.id);
            result.push(node);
        }
    }

    result
}

#[test]
fn tree_test() {
    // let data = vec![
    //     MenuItem {
    //         id: 1,
    //         key: "item1".to_string(),
    //         label: "Item 1".to_string(),
    //         icon: "icon1".to_string(),
    //         parent_id: 0,
    //         children: vec![],
    //     },
    //     MenuItem {
    //         id: 2,
    //         key: "item2".to_string(),
    //         label: "Item 2".to_string(),
    //         icon: "icon2".to_string(),
    //         parent_id: 1,
    //         children: vec![],
    //     },
    //     MenuItem {
    //         id: 3,
    //         key: "item3".to_string(),
    //         label: "Item 3".to_string(),
    //         icon: "icon3".to_string(),
    //         parent_id: 1,
    //         children: vec![],
    //     },
    // ];

    // let tree = build_tree(data, 0);
    // dbg!("{:#?}", tree);
}
