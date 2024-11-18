use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowAttributes},
};

struct App {
    window: Option<Window>,
}

impl App {
    fn new() -> Self {
        Self { window: None }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(
                WindowAttributes::default()
                    .with_inner_size(PhysicalSize::new(640, 480))
                    .with_min_inner_size(PhysicalSize::new(320, 240))
                    .with_max_inner_size(PhysicalSize::new(1280, 960)),
            )
            .unwrap();
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::CursorEntered { .. } => println!("Mouse Enter"),
            WindowEvent::CursorLeft { .. } => println!("Mouse Left"),
            _ => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut App::new()).unwrap();
}
