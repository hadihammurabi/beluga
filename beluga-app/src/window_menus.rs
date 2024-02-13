use floem::menu::{Menu, MenuItem};

pub fn window_menu() -> Menu {
    Menu::new("beluga")
        .entry({
            Menu::new("beluga")
                .action(|| {
                    printf!("click beluga");
                })
                .separator()
                .entry({
                    Menu::new("Settings").entry(MenuItem::new("Open Settings").action(|| {
                        printf!("click open settings");
                    }))
                })
                .separator()
                .entry({
                    MenuItem::new("Quit Beluga").action(|| {
                        printf!("click quit beluga");
                    })
                });
        })
        .separator()
        .entry({
            Menu::new("View").entry(MenuItem::new("Theme").action(|| printf!("click theme")))
        });
}
