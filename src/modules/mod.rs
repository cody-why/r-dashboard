#![allow(non_snake_case)]

pub mod demo_data;

/// USERS for dashboard
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub name: String,
    pub email: String,
    pub title: String,
    pub title2: String,
    pub status: String,
    pub role: String,
}

/// simple data for talbes
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleTableData {
    pub city: String,
    pub totalOrders: String,
}
#[derive(Debug, Clone, PartialEq)]
pub struct PaginatedTableData {
    pub picture: String,
    pub name: String,
    pub role: String,
    pub created: String,
    pub status: String,
    pub statusColor: String,
}
#[derive(Debug, Clone, PartialEq)]
pub struct WideTableData {
    pub name: String,
    pub email: String,
    pub title: String,
    pub title2: String,
    pub status: String,
    pub role: String,
}
