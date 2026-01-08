mod controller;
mod wifi;

pub use controller::Controller;
pub use wifi::WifiAdapter;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn controller_initial_state_is_false() {
        let ctrl = Controller::new();
        assert_eq!(ctrl.state(), false);
    }

    #[test]
    fn controller_toggle_changes_state() {
        let mut ctrl = Controller::new();

        let s1 = ctrl.toggle();
        assert_eq!(s1, true);

        let s2 = ctrl.toggle();
        assert_eq!(s2, false);
    }
}
