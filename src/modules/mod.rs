#![allow(non_snake_case)]

pub mod demo_data;

/// USERS for dashboard
#[derive(Debug )]
pub struct User {
    pub name: String,
    pub email: String,
    pub title: String,
    pub title2: String,
    pub status: String,
    pub role: String,
}

/// simple data for talbes
pub struct SimpleTableData {
    pub city: String,
    pub totalOrders: String,
}


pub struct PaginatedTableData {
    pub picture: String,
    pub name: String,
    pub role: String,
    pub created: String,
    pub status: String,
    pub statusColor: String,
}
 

pub struct WideTableData {
    pub name: String,
    pub email: String,
    pub title: String,
    pub title2: String,
    pub status: String,
    pub role: String,
}