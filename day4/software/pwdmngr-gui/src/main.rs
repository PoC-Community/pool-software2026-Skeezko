mod crypto;
mod models;
mod storage;
use crate::crypto::CryptoManager;
use crate::models::{PasswordEntry, PasswordStore};
use crate::storage::Storage;
use iced::Alignment;
use iced::widget::button;
use iced::widget::canvas::fill;
use iced::widget::operation::focus;
use iced::widget::pick_list;
use iced::widget::row;
use iced::widget::text;
use iced::widget::{TextInput, text_input};
use iced::widget::{column, container};
use iced::{Element, Task, Theme, message};
use iced::{Fill, Font};
use std::io::{self, Write};

struct PasswordManagerApp {
    master_password_input: String,
    error_message: Option<String>,
    storage: Option<Storage>,
    store: PasswordStore,
    search_query: String,
    selected_service: Option<usize>,
    is_password_visible: bool,
    theme: Theme,
}

#[derive(Debug, Clone)]
enum Message {
    MasterPasswordChanged(String),
    LoginPressed,
    SearchChanged(String),
    SelectService(usize),
    SaveEntry,
    DeleteService(usize),
    GenerateNewPassword,
    ToggleVisibility,
    CopyPassword(String),
    ThemeChanged(Theme),
}

impl Default for PasswordManagerApp {
    fn default() -> Self {
        Self {
            master_password_input: String::new(),
            error_message: None,
            storage: None,
            store: PasswordStore {
                entries: Vec::new(),
            },
            search_query: String::new(),
            selected_service: None,
            is_password_visible: false,
            theme: Theme::Nightfly,
        }
    }
}

impl PasswordManagerApp {
    fn update(&mut self, message: Message) {
        match message {
            Message::ThemeChanged(theme) => {
                self.theme = theme;
            }
            Message::ToggleVisibility => {
                self.is_password_visible = !self.is_password_visible;
            }
            Message::MasterPasswordChanged(input) => {
                self.master_password_input = input;
            }
            Message::LoginPressed => {
                let crypto = CryptoManager::new(&self.master_password_input);

                match crypto {
                    Ok(c) => {
                        let storage = Storage { crypto: c };
                        match storage.load() {
                            Ok(s) => {
                                self.storage = Some(storage);
                                self.store = s
                            }
                            Err(e) => self.error_message = Some(e),
                        }
                    }
                    Err(e) => {
                        self.error_message = Some(e);
                    }
                }
            }
            _ => {}
        }
    }

    fn view(&self) -> Element<Message> {
        if self.storage.is_none() {
            self.view_login()
        } else {
            self.view_dashboard()
        }
    }

    fn view_login(&self) -> Element<Message> {
        let title = text("SECURE VAULT").size(82).font(Font::MONOSPACE);

        let subtitle = text("Unlock your encrypted credentials").size(22);

        let theme_picker = row![
            text("Theme:").size(20),
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged)
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        let pwd_input = text_input("Enter Master Password", &self.master_password_input)
            .secure(!self.is_password_visible)
            .on_input(Message::MasterPasswordChanged)
            .on_submit(Message::LoginPressed)
            .padding(15)
            .size(18);

        let toggle_button = button(text(if self.is_password_visible {
            "󰈈"
        } else {
            "󰈉"
        }))
        .on_press(Message::ToggleVisibility)
        .padding(12)
        .style(button::secondary);

        let input_row = row![pwd_input, toggle_button]
            .spacing(10)
            .align_y(iced::Alignment::Center);

        let login_button = button(text("UNLOCK SYSTEM").font(Font::MONOSPACE))
            .on_press(Message::LoginPressed)
            .padding([12, 60])
            .style(button::primary);

        let error_text = if let Some(err) = &self.error_message {
            text(err).color([1.0, 0.3, 0.3]).size(14)
        } else {
            text("")
        };

        let login_page = container(
            column![
                title,
                subtitle,
                input_row,
                login_button,
                error_text,
                theme_picker
            ]
            .spacing(25)
            .align_x(iced::Alignment::Center),
        )
        .padding(40);

        container(login_page)
            .width(Fill)
            .height(Fill)
            .center(Fill)
            .into()
    }

    fn view_dashboard(&self) -> Element<Message> {
        text("Hello world!").into()
    }
}

fn main() -> iced::Result {
    iced::application(
        PasswordManagerApp::default,
        PasswordManagerApp::update,
        PasswordManagerApp::view,
    )
    .title("Password Manager")
    .theme(|app: &PasswordManagerApp| app.theme.clone())
    .run()
}

/*
    print!("\nEnter the master password : ");
    io::stdout().flush().expect("Failed to flush");
    let master_password = rpassword::read_password().expect("Failed to read master password");
    let crypto = CryptoManager::new(&master_password);
    let storage = Storage{cyrpto: crypto};

    match cli.command{
        Commands::Add { service, username } => {
            print!("\nEnter the password you want to add : ");
            io::stdout().flush().expect("Failde to flush");
            let password = rpassword::read_password().expect("Faile to read password");
            let mut store = storage.load();
            let new_entry = PasswordEntry{service: service, username: username, password: password};
            store.entries.push(new_entry);
            storage.save(store);
            println!("\nService added successfully !\n")
        },
        Commands::Get { service } => {
            let store = storage.load();
            let target = store.entries.iter().find(|&s| s.service == service);
            if let Some(entry) = target {
                println!("\nService: {:?}", entry.service);
                println!("Username: {:?}", entry.username);
                println!("Password: {:?}\n", entry.password);
            } else {
                println!("\nServices not registered !\n")
            }
        }
        Commands::List => {
            let store = storage.load();
            if store.entries.is_empty() {
                println!("\nYou have 0 services :(\n");
            } else {
                for entry in store.entries {
                    println!("\nService: {:?}", entry.service);
                    println!("Username: {:?}", entry.username);
                    println!("Password: {:?}\n", entry.password);
                }
            }
        }
        Commands::Delete { service } => {
            let mut store = storage.load();
            store.entries.retain(|s| s.service != service);
            storage.save(store);
            println!("\nService deleted successfully !")
        }
        Commands::Generate { length } => {
            let password = storage.generate_password(length);
            println!("Your generated password: {}", password);
        }
    };
*/
