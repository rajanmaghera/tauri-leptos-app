mod app;

use app::*;
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}

#[cfg(test)]
mod tests {

    #[test]
    fn basic_equality() {
        assert_eq!(1, 2);
    }
}
