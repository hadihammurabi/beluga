use floem::reactive::create_signal;
use floem::view::View;
use floem::views::{h_stack, label, v_stack, Decorators};
use floem::widgets::button;
use crate::window_menus;

const SIDEBAR_WIDTH: f64 = 140.0;

pub fn launch() -> impl View {
    floem::launch(app_view)
}

fn app_view() -> impl View {
    // Create a reactive signal with a counter value, defaulting to 0
    let (counter, set_counter) = create_signal(0);

    window_menus::window_menus();

    // Create a vertical layout
    v_stack((
        // The counter value updates automatically, thanks to reactivity
        label(move || format!("Value: {}", counter.get())),
        // Create a horizontal layout
        h_stack((
            button(|| "Increment").on_click_stop(move |_| {
                set_counter.update(|value| *value += 1);
            }),
            button(|| "Decrement").on_click_stop(move |_| {
                set_counter.update(|value| *value -= 1);
            }),
        )),
    ))
}
