/*
 * @Date: 2022-10-11 23:13:17
 * @LastEditTime: 2024-07-06 00:05:28
 * @Description:
 */

use dioxus::signals::{GlobalSignal, Signal};

pub mod header;
pub mod sidebar;

// #[derive(Clone, Copy)]
// struct SidebarOpen(bool);

static SIDEBAR_OPEN: GlobalSignal<bool> = Signal::global(|| false);
