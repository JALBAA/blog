
#[derive(Hash, Serialize, Debug)]
pub struct NavItem {
    name: String,
    href: String,
    pub selected: bool,
}

#[derive(Hash, Serialize, Debug)]
pub struct NavInfo {
    pub nav_items: Vec<NavItem>,
}

pub fn get_nav_info () -> NavInfo {
    let mut nav_info = NavInfo {
        nav_items: vec![],
    };
    let nav_info_arr = [/*("主页", "/"), */("日志", "/"), ("读书", "/books")];
    for nav in &nav_info_arr {
        let mut a = NavItem {
            name: String::from(nav.0),
            href: String::from(nav.1),
            selected: true,
        };
        nav_info.nav_items.push(a);
    }
    nav_info
}