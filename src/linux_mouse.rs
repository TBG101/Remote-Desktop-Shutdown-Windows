use evdev::uinput::VirtualDeviceBuilder;
use evdev::Synchronization;
use evdev::{uinput::VirtualDevice, AttributeSet, EventType, InputEvent, Key, RelativeAxisType};
use std::thread;
use std::time::Duration;

/// Struct to manage the virtual mouse device
pub struct VirtualMouse {
    device: VirtualDevice,
    acummulated_position: (i32, i32),
}

impl VirtualMouse {
    /// Create a new virtual mouse device
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut keys = AttributeSet::<Key>::new();
        let mut relative_axes = AttributeSet::<RelativeAxisType>::new();

        // Enable mouse buttons
        keys.insert(Key::BTN_LEFT);
        keys.insert(Key::BTN_RIGHT);
        keys.insert(Key::BTN_MIDDLE);

        // Enable relative axes (movement and scroll)
        relative_axes.insert(RelativeAxisType::REL_X);
        relative_axes.insert(RelativeAxisType::REL_Y);
        relative_axes.insert(RelativeAxisType::REL_WHEEL);

        let device = VirtualDeviceBuilder::new()?
            .name("Rust Virtual Mouse")
            .with_keys(&keys)?
            .with_relative_axes(&relative_axes)?
            .build()?;

        Ok(Self {
            device: device,
            acummulated_position: (0, 0),
        })
    }

    fn flush() -> InputEvent {
        InputEvent::new(EventType::SYNCHRONIZATION, Synchronization::SYN_REPORT.0, 0)
    }

    /// Move the mouse cursor by a relative amount (x, y)
    pub fn move_mouse(&mut self, x: i32, y: i32) -> Result<(), Box<dyn std::error::Error>> {
        if x.abs() < 2 && y.abs() < 2 {
            self.acummulated_position.0 += x;
            self.acummulated_position.1 += y;
            return Ok(());
        }
        if self.acummulated_position.0 != 0 || self.acummulated_position.1 != 0 {
            let move_x = InputEvent::new(
                EventType::RELATIVE,
                RelativeAxisType::REL_X.0,
                self.acummulated_position.0 + x,
            );
            let move_y = InputEvent::new(
                EventType::RELATIVE,
                RelativeAxisType::REL_Y.0,
                self.acummulated_position.1 + y,
            );
            self.device.emit(&[move_x, move_y])?;
            self.acummulated_position = (0, 0);
            return Ok(());
        }

        let move_x = InputEvent::new(EventType::RELATIVE, RelativeAxisType::REL_X.0, x);
        let move_y: InputEvent = InputEvent::new(EventType::RELATIVE, RelativeAxisType::REL_Y.0, y);
        self.device.emit(&[move_x, move_y])?;

        Ok(())
    }

    /// Press a mouse button (e.g., Key::BTN_LEFT, Key::BTN_RIGHT, Key::BTN_MIDDLE)
    pub fn button_down(&mut self, button: Key) -> Result<(), Box<dyn std::error::Error>> {
        let event = InputEvent::new(EventType::KEY, button.0, 1);
        let flush: InputEvent = Self::flush();
        self.device.emit(&[event, flush])?;
        Ok(())
    }

    /// Release a mouse button
    pub fn button_up(&mut self, button: Key) -> Result<(), Box<dyn std::error::Error>> {
        let event = InputEvent::new(EventType::KEY, button.0, 0);
        let flush: InputEvent = Self::flush();

        self.device.emit(&[event, flush])?;
        Ok(())
    }

    /// Simulate a mouse click (press and release)
    pub fn button_click(&mut self, button: Key) -> Result<(), Box<dyn std::error::Error>> {
        self.button_down(button)?;
        thread::sleep(Duration::from_millis(50)); // Short delay to simulate a click
        self.button_up(button)?;
        Ok(())
    }

    /// Simulate mouse scrolling (positive for up, negative for down)
    pub fn scroll(&mut self, amount: i32) -> Result<(), Box<dyn std::error::Error>> {
        print!("mouse scroll: {}", amount);
        let event = InputEvent::new(
            EventType::RELATIVE,
            RelativeAxisType::REL_WHEEL_HI_RES.0,
            amount * 2,
        );

        let flush: InputEvent = Self::flush();
        self.device.emit(&[event, flush])?;
        Ok(())
    }
}
