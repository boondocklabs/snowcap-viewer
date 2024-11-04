use std::process::exit;

use colored::Colorize as _;
use custom_module::MyModule;
use snowcap::iced::{Element, Task, Theme};
use snowcap::Snowcap;
use tracing_subscriber::{self, EnvFilter};

mod custom_module;

pub fn main() -> snowcap::iced::Result {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        //.with_file(true)
        //.with_line_number(true)
        .init();

    let args: Vec<String> = std::env::args().collect();

    let filename = match args.get(1) {
        Some(filename) => filename.clone(),
        None => {
            println!("{}", "Snowcap markfup file not provided".red());
            exit(1)
        }
    };

    #[cfg(feature = "profile-with-puffin")]
    let _puffin_server = {
        println!("Puffin Profiling Enabled. Starting server.");
        let server_addr = format!("127.0.0.1:{}", puffin_http::DEFAULT_PORT);
        let _puffin_server = puffin_http::Server::new(&server_addr).unwrap();

        profiling::puffin::set_scopes_on(true);
        _puffin_server
    };

    #[cfg(feature = "profile-with-tracy")]
    {
        println!("Tracy Profiling Enabled");
        tracy_client::Client::start();
    }

    profiling::register_thread!("Main Thread");

    snowcap::iced::application("Snowcap", SnowcapViewer::update, SnowcapViewer::view)
        .theme(SnowcapViewer::theme)
        .run_with(move || {
            let viewer = SnowcapViewer::new(filename.clone());

            match viewer {
                Ok(mut viewer) => {
                    let init_task = viewer.init();
                    (
                        // Provide initial state and init task
                        viewer, init_task,
                    )
                }
                Err(err) => {
                    tracing::error!("{}", err);
                    exit(-1)
                }
            }
        })
}

struct SnowcapViewer {
    filename: String,
    snow: Snowcap<Message>,
}

impl SnowcapViewer {
    pub fn new(filename: String) -> Result<Self, snowcap::Error> {
        let mut viewer = Self {
            filename,
            snow: Snowcap::new().unwrap(),
        };

        viewer.snow.modules().register::<MyModule>("custom-module");

        viewer.load()?;

        Ok(viewer)
    }

    pub fn load(&mut self) -> Result<(), snowcap::Error> {
        self.snow.load_file(self.filename.clone())?;
        Ok(())
    }

    fn init(&mut self) -> Task<snowcap::Message<Message>> {
        self.snow.init()
    }
}

#[derive(Debug, Clone)]
enum Message {}

impl SnowcapViewer {
    #[profiling::function]
    fn update(
        &mut self,
        mut message: snowcap::Message<Message>,
    ) -> Task<snowcap::Message<Message>> {
        self.snow.update(&mut message)
    }

    #[profiling::function]
    fn view(&self) -> Element<snowcap::Message<Message>> {
        profiling::scope!("View");
        let element = self.snow.view();
        profiling::finish_frame!();
        element
    }

    fn theme(&self) -> Theme {
        //Theme::TokyoNight
        Theme::CatppuccinFrappe
    }
}
