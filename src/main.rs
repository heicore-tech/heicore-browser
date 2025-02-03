use winit::event_loop::EventLoop;
use winit::window::Window;
use winit::application::ApplicationHandler;
use winit::event_loop::ActiveEventLoop;
use wry::WebViewBuilder;
use winit::window::WindowId;
use winit::event::WindowEvent;
use winit::window::Icon;

#[derive(Default)]
struct App {
  window: Option<Window>,
  webview: Option<wry::WebView>,
}

impl ApplicationHandler for App {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    let icon = load_icon(include_bytes!("../resources/icon.png"));
    let window = event_loop.create_window(Window::default_attributes()
    .with_title("Winit window")
    .with_transparent(true)
    .with_window_icon(Some(icon.clone()))).unwrap();
    let webview = WebViewBuilder::new()
      .with_url("https://heicore.tql.moe")
      .build(&window)
      .unwrap();

    self.window = Some(window);
    self.webview = Some(webview);
  }

  fn window_event(&mut self, _event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {}
}
fn load_icon(bytes: &[u8]) -> Icon {
  let (icon_rgba, icon_width, icon_height) = {
      let image = image::load_from_memory(bytes).unwrap().into_rgba8();
      let (width, height) = image.dimensions();
      let rgba = image.into_raw();
      (rgba, width, height)
  };
  Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}



fn main() {
let event_loop = EventLoop::new().unwrap();
let mut app = App::default();
event_loop.run_app(&mut app).unwrap();
}
