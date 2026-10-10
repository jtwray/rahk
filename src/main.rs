use rdev::{listen, simulate, Event, EventType, Button, SimulateError};

fn main() {
    println!("🚀 RAHK is running: side button → right click remap.");

    // Start listening for mouse events
    if let Err(error) = listen(callback) {
        eprintln!("Error listening to events: {:?}", error);
    }
}

// This function is triggered for every input event (mouse/keyboard)
fn callback(event: Event) {
    match event.event_type {
        EventType::ButtonPress(button) => {
            println!("🖱️ Button Pressed: {:?}", button);

            // Match side back button (XButton1 / Button8 / Button::Side)
            // if button == Button::Button8 || button == Button::Side {
            //     println!("↪️ Remapping to Right Click!");
            //     if let Err(e) = click_right_button() {
            //         eprintln!("❌ Failed to simulate right click: {:?}", e);
            //     }
            // }

                        // Match side back button (XButton1 / Button8 / Button::Side)
            if button == Button::Unknown(8) {
                println!("↪️ Remapping to Right Click!");
                if let Err(e) = click_right_button() {
                    eprintln!("❌ Failed to simulate right click: {:?}", e);
                }
            }
        }
        _ => {}
    }
}

// Simulate a right-click press and release
fn click_right_button() -> Result<(), SimulateError> {
    simulate(&EventType::ButtonPress(Button::Right))?;
    simulate(&EventType::ButtonRelease(Button::Right))?;
    Ok(())
}
