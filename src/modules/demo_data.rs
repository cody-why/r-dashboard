/*
 * @Author: plucky
 * @Date: 2022-10-14 18:11:55
 * @LastEditTime: 2022-10-14 23:48:07
 * @Description: 
 */

use dioxus::prelude::Atom;

use super::*;


pub struct UseTableData{
    pub simpleTableData: Vec<SimpleTableData>,
    pub paginatedTableData: Vec<PaginatedTableData>,
    pub wideTableData: Vec<WideTableData>,
    
}

/// 表格测试数据 for tables
pub static USE_TABLE_DATA: Atom<UseTableData> = |_| {
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
        ],
        wideTableData: (0..5).map(|_i| {
            WideTableData {
                name: "John Doe".into(),
                email: "john@example.com".into(),
                title: "Software Engineer".into(),
                title2: "Web dev".into(),
                status: "Active".into(),
                role: "Owner".into(),
            }
        }).collect(),
    }
};