use std::time::Duration;

pub trait Update {
    fn update(&mut self, dt: Duration);
}