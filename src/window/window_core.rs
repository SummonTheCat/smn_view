use wry::{
    application::{
        dpi::LogicalSize,
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        platform::run_return::EventLoopExtRunReturn,
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
};

use crate::util::logging::{logln, logln_color, Color};

use super::structs::struct_windowconfig::WindowConfig;


pub fn start_window<F: FnOnce()>(config: WindowConfig, on_close: F) -> wry::Result<()> {
    let mut event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title(config.title)
        .with_inner_size(LogicalSize::new(800.0, 600.0))
        .build(&event_loop)?;

    let webview = WebViewBuilder::new(window)?.with_url(&config.url)?.build()?;

    logln_color("[Started: Window]", Color::Green);
    logln(&format!("{} {}", Color::BrightBlack.paint("Window starting on URL:"), Color::Blue.paint(&config.url)));

    event_loop.run_return(|event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });

    // Trigger the on_close callback once the window loop exits
    logln_color("[Ended: Window]", Color::Green);
    on_close();

    // Drop the webview cleanly after returning from the event loop
    drop(webview);

    Ok(())
}
