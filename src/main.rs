mod app;
mod listener;
mod sprites;
mod state;

use std::path::PathBuf;

use app::App;
use listener::GlobalInput;
use sprites::SpriteSet;
use winit::event_loop::{ControlFlow, EventLoop};
#[cfg(target_os = "linux")]
use winit::platform::x11::EventLoopBuilderExtX11;

fn resolve_sprites_dir() -> PathBuf {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let beside_exe = dir.join("assets/sprites");
            if beside_exe.exists() {
                return beside_exe;
            }
        }
    }

    let cwd = PathBuf::from("assets/sprites");
    if cwd.exists() {
        return cwd;
    }

    panic!("找不到 assets/sprites 資料夾，請確認它與執行檔放在同一層目錄");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sprites_dir = resolve_sprites_dir();
    let sprites = SpriteSet::load_all(sprites_dir.to_str().expect("路徑包含無效字元"), 128);

    let mut builder = EventLoop::<GlobalInput>::with_user_event();
    #[cfg(target_os = "linux")]
    builder.with_x11();

    let event_loop = builder.build()?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let proxy = event_loop.create_proxy();
    listener::start(move |ev| {
        let _ = proxy.send_event(ev);
    });

    let mut app = App::new(sprites);
    event_loop.run_app(&mut app)?;
    Ok(())
}
