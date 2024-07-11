/*
 * @Date: 2022-10-14 18:11:55
 * @LastEditTime: 2024-07-10 22:53:56
 * @Description:
 */

use dioxus::signals::{GlobalSignal, Signal};

use crate::components::{
    menu::icons,
    menu::{build_tree, MenuItem},
};

use super::*;

#[derive(Clone, PartialEq)]
pub struct UseTableData {
    pub simpleTableData: Vec<SimpleTableData>,
    pub paginatedTableData: Vec<PaginatedTableData>,
}

/// 表格测试数据 for tables
pub static USE_TABLE_DATA: GlobalSignal<UseTableData> = Signal::global(|| {
    UseTableData{
        simpleTableData: vec![
            SimpleTableData{
                city: "New York".to_string(),
                totalOrders: "200,120".to_string(),
            },
            SimpleTableData{
                city: "Manchester".to_string(),
                totalOrders: "632,310".to_string(),
            },
            SimpleTableData{
                city: "London".to_string(),
                totalOrders: "1,200,120".to_string(),
            },
        ],
        paginatedTableData: vec![
            PaginatedTableData{
                picture: "https://images.unsplash.com/photo-1494790108377-be9c29b29330?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2.2&w=160&h=160&q=80".to_string(),
                name: "Vera Carpenter".to_string(),
                role: "Admin".to_string(),
                created: "Jan 21, 2020".to_string(),
                status: "Active".to_string(),
                statusColor: "green".to_string(),
            },
            PaginatedTableData{
                picture: "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2.2&w=160&h=160&q=80".to_string(),
                name: "Blake Bowman".to_string(),
                role: "Editor".to_string(),
                created: "Jan 01, 2020".to_string(),
                status: "Active".to_string(),
                statusColor: "green".to_string(),
            },
            PaginatedTableData{
                picture: "https://images.unsplash.com/photo-1540845511934-7721dd7adec3?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2.2&w=160&h=160&q=80".to_string(),
                name: "Dana Moore".to_string(),
                role: "Editor".to_string(),
                created: "Jan 10, 2020".to_string(),
                status: "Suspended".to_string(),
                statusColor: "orange".to_string(),
            },
            PaginatedTableData{
                picture: "https://images.unsplash.com/photo-1522609925277-66fea332c575?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2.2&h=160&w=160&q=80".to_string(),
                name: "Alonzo Cox".to_string(),
                role: "Admin".to_string(),
                created: "Jan 18, 2020".to_string(),
                status: "Inactive".to_string(),
                statusColor: "red".to_string(),
            },
        ]
    }
});

// test data
pub static USERS: GlobalSignal<Vec<User>> = Signal::global(|| {
    (0..5)
        .map(|_i| User {
            name: "John Doe".into(),
            email: "john@example.com".into(),
            title: "Software Engineer".into(),
            title2: "Web dev".into(),
            status: if _i % 2 == 0 { "Active" } else { "Inactive" }.into(),
            role: "Owner".into(),
        })
        .collect()
});

// test data
pub static MENUS: GlobalSignal<Vec<MenuItem>> = Signal::global(|| {
    let data = vec![
        MenuItem {
            id: 1,
            key: "/dashboard".to_string(),
            label: "Dashboard".to_string(),
            // icon: "icon_chart".to_string(),
            icon: icons::icon_chart(),
            parent_id: 0,
            children: vec![],
        },
        MenuItem {
            id: 2,
            key: "/ui-elements".to_string(),
            label: "UI Elements".to_string(),
            // icon: "icon_element".to_string(),
            icon: icons::icon_element(),
            parent_id: 0,
            children: vec![],
        },
        MenuItem {
            id: 3,
            key: "/tables".to_string(),
            label: "Tables".to_string(),
            // icon: "icon_table".to_string(),
            icon: icons::icon_table(),
            parent_id: 0,
            children: vec![],
        },
        MenuItem {
            id: 4,
            key: "/forms".to_string(),
            label: "Forms".to_string(),
            // icon: "icon_form".to_string(),
            icon: icons::icon_form(),
            parent_id: 0,
            children: vec![],
        },
        MenuItem {
            id: 5,
            key: "/cards".to_string(),
            label: "Cards".to_string(),
            // icon: "icon_card".to_string(),
            icon: icons::icon_card(),
            parent_id: 0,
            children: vec![],
        },
        MenuItem {
            id: 6,
            key: "/modal".to_string(),
            label: "Modal".to_string(),
            // icon: "icon_model".to_string(),
            icon: icons::icon_model(),
            parent_id: 0,
            children: vec![],
        },
        MenuItem {
            id: 7,
            key: "#".to_string(),
            label: "Test".to_string(),
            // icon: "icon_element".to_string(),
            icon: icons::icon_element(),
            parent_id: 0,
            children: vec![],
        },
        MenuItem {
            id: 8,
            key: "/blank1".to_string(),
            label: "Blank".to_string(),
            // icon: "icon_blank".to_string(),
            icon: icons::icon_blank(),
            parent_id: 7,
            children: vec![],
        },
        MenuItem {
            id: 9,
            key: "/blank2".to_string(),
            label: "Blank".to_string(),
            // icon: "icon_blank".to_string(),
            icon: icons::icon_blank(),
            parent_id: 7,
            children: vec![],
        },
    ];

    build_tree(data, 0)
});
