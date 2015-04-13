use std::any::Any;

use input::Button;
use { GenericEvent, PRESS };

/// The press of a button
pub trait PressEvent {
    /// Creates a press event.
    fn from_button(button: Button, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a press event.
    fn press<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(Button) -> U;
    /// Returns press arguments.
    fn press_args(&self) -> Option<Button> {
        self.press(|button| button)
    }
}

impl<T: GenericEvent> PressEvent for T {
    fn from_button(button: Button, old_event: &Self) -> Option<Self> {
        GenericEvent::from_args(PRESS, &button as &Any, old_event)
    }

    fn press<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(Button) -> U
    {
        if self.event_id() != PRESS {
            return None;
        }
        self.with_args(|any| {
            if let Some(&button) = any.downcast_ref::<Button>() {
                Some(f(button))
            } else {
                panic!("Expected Button")
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_input_press() {
        use input::{ Button, Key, Input };

        let e = Input::Press(Button::Keyboard(Key::S));
        let button = Button::Keyboard(Key::A);
        let x: Option<Input> = PressEvent::from_button(button, &e);
        let y: Option<Input> = x.clone().unwrap().press(|button|
            PressEvent::from_button(button, x.as_ref().unwrap())).unwrap();
        assert_eq!(x, y);
    }

    #[bench]
    fn bench_input_press(bencher: &mut Bencher) {
        use input::{ Button, Input, Key };

        let e = Input::Press(Button::Keyboard(Key::S));
        let button = Button::Keyboard(Key::A);
        bencher.iter(|| {
            let _: Option<Input> = PressEvent::from_button(button, &e);
        });
    }

    #[test]
    fn test_event_press() {
        use Event;
        use input::{ Button, Key, Input };

        let e = Event::Input(Input::Press(Button::Keyboard(Key::S)));
        let button = Button::Keyboard(Key::A);
        let x: Option<Event> = PressEvent::from_button(button, &e);
        let y: Option<Event> = x.clone().unwrap().press(|button|
            PressEvent::from_button(button, x.as_ref().unwrap())).unwrap();
        assert_eq!(x, y);
    }

    #[bench]
    fn bench_event_press(bencher: &mut Bencher) {
        use Event;
        use input::{ Button, Key, Input };

        let e = Event::Input(Input::Press(Button::Keyboard(Key::S)));
        let button = Button::Keyboard(Key::A);
        bencher.iter(|| {
            let _: Option<Event> = PressEvent::from_button(button, &e);
        });
    }
}
