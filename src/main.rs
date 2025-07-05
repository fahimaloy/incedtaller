use iced::{
        Application, Column, Command, Container, Element, Settings, Subscription, Text, Window,
    };
    use std::fs;
    use std::path::Path;

    struct AppImageInstaller {
        file_path: Option<String>,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Message {
        FileDropped(String),
        FileRemoved,
    }

    impl Application for AppImageInstaller {
        type Message = Message;
        type Flags = ();

        fn new(_flags: ()) -> (AppImageInstaller, Command<Self::Message>) {
            (AppImageInstaller { file_path: None }, Command::none())
        }

        fn title(&self) -> String {
            String::from("AppImage Installer")
        }

        fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
            match message {
                Message::FileDropped(file_path) => {
                    self.file_path = Some(file_path);
                    Command::none()
                }
                Message::FileRemoved => {
                    self.file_path = None;
                    Command::none()
                }
            }
        }

        fn subscription(&self) -> Subscription<Self::Message> {
            Subscription::none()
        }

        fn view(&mut self) -> Element<Self::Message> {
            let file_path = self.file_path.clone();

            if let Some(file_path) = file_path {
                let file_name = Path::new(&file_path).file_name().unwrap().to_str().unwrap();
                let file_extension = Path::new(&file_path).extension().unwrap().to_str().unwrap();

                if file_extension == "appimage" {
                    Column::new()
                        .push(Text::new(format!("Installing {}", file_name)))
                        .push(Text::new("Please wait..."))
                        .into()
                } else {
                    Column::new()
                        .push(Text::new(format!("Invalid file: {}", file_name)))
                        .push(Text::new("Please drop a valid .appimage file"))
                        .into()
                }
            } else {
                Column::new()
                    .push(Text::new("Drag and drop a .appimage file here"))
                    .push(Text::new("or"))
                    .push(Text::new("click to select a file"))
                    .into()
            }
        }
    }

    fn main() {
        AppImageInstaller::run(Settings::default());
    }
