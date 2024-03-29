use std::sync::RwLock;

use auto_launch::{AutoLaunch, AutoLaunchBuilder};
use lazy_static::lazy_static;
use serde::{Serialize, Serializer};
use tauri::{utils::platform::current_exe, App, Manager, Runtime, State};
lazy_static! {
    pub static ref AUTO_LAUNCH_MANAGER: RwLock<Option<AutoLaunchManager>> = RwLock::new(None);
}

#[derive(Debug, Copy, Clone)]
pub enum MacosLauncher {
    LaunchAgent,
    AppleScript,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Anyhow(String),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub struct AutoLaunchManager(AutoLaunch);

impl AutoLaunchManager {
    pub fn enable(&self) -> Result<()> {
        self.0
            .enable()
            .map_err(|e| e.to_string())
            .map_err(Error::Anyhow)
    }

    pub fn disable(&self) -> Result<()> {
        self.0
            .disable()
            .map_err(|e| e.to_string())
            .map_err(Error::Anyhow)
    }

    pub fn is_enabled(&self) -> Result<bool> {
        self.0
            .is_enabled()
            .map_err(|e| e.to_string())
            .map_err(Error::Anyhow)
    }
}
pub type Result<T> = std::result::Result<T, Error>;

pub trait ManagerExt<R: Runtime> {
    fn autolaunch(&self) -> State<'_, AutoLaunchManager>;
}

impl<R: Runtime, T: Manager<R>> ManagerExt<R> for T {
    fn autolaunch(&self) -> State<'_, AutoLaunchManager> {
        self.state::<AutoLaunchManager>()
    }
}

pub fn init_auto_launch_manager(
    app: &mut App,
    macos_launcher: MacosLauncher,
    args: Option<Vec<&'static str>>,
)->Result<()> {
    let mut builder = AutoLaunchBuilder::new();
    builder.set_app_name(&app.package_info().name);
    if let Some(args) = args {
        builder.set_args(&args);
    }
    builder.set_use_launch_agent(matches!(macos_launcher, MacosLauncher::LaunchAgent));

    let current_exe = current_exe()?;

    #[cfg(windows)]
    builder.set_app_path(&current_exe.display().to_string());
    #[cfg(target_os = "macos")]
    {
        // on macOS, current_exe gives path to /Applications/Example.app/MacOS/Example
        // but this results in seeing a Unix Executable in macOS login items
        // It must be: /Applications/Example.app
        // If it didn't find exactly a single occurance of .app, it will default to
        // exe path to not break it.
        let exe_path = current_exe.canonicalize()?.display().to_string();
        let parts: Vec<&str> = exe_path.split(".app/").collect();
        let app_path = if parts.len() == 2 {
            format!("{}.app", parts.first().unwrap())
        } else {
            exe_path
        };
        info!("auto_start path {}", &app_path);
        builder.set_app_path(&app_path);
    }
    #[cfg(target_os = "linux")]
    if let Some(appimage) = app
        .env()
        .appimage
        .and_then(|p| p.to_str().map(|s| s.to_string()))
    {
        builder.set_app_path(&appimage);
    } else {
        builder.set_app_path(&current_exe.display().to_string());
    }
    let mut manager = AUTO_LAUNCH_MANAGER.write().unwrap();
    *manager = Some(AutoLaunchManager(builder.build().unwrap()));
    return Ok(());
}
