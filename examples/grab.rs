use rdev::{grab, simulate, Event, EventType, Key};

fn main() {
    // This will block.
    if let Err(error) = grab(callback) {
        println!("Error: {:?}", error)
    }
}

fn callback(event: Event) -> Option<Event> {
    match event.event_type {
        EventType::SimulatedKeyRelease(_) | EventType::SimulatedKeyPress(_) => {
            println!("{:?}", event.event_type);
            Some(event)
        }

        EventType::KeyPress(Key::Tab) => {
            println!("Pressed TAB: {:?}", event.name.unwrap_or_default());

            simulate(&EventType::KeyPress(Key::KeyA)).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(20));
            simulate(&EventType::KeyRelease(Key::KeyA)).unwrap();
            None
        }
        EventType::KeyRelease(_) | EventType::KeyPress(_) => {
            println!("{:?}", event.event_type);
            Some(event)
        }

        _ => Some(event),
    }
}
