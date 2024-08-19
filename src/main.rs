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

fn get_number() -> i32 {
    42
}

#[cfg(test)]
mod tests {
    use crate::get_number;

    #[test]
    fn basic_equality() {
        assert_eq!(1, 2);
    }

    #[test]
    fn basic_number_back() {
        assert_eq!(get_number(), 42);
    }
}
