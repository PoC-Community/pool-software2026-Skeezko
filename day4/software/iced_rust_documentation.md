# Documentation complète Iced Rust - Interface Graphique

## Table des matières

1. [Introduction à Iced](#introduction)
2. [Installation et Configuration](#installation)
3. [Architecture de base](#architecture)
4. [Widgets de base](#widgets-de-base)
5. [Containers et Layouts](#containers-et-layouts)
6. [Styling et Thèmes](#styling-et-themes)
7. [Widgets avancés](#widgets-avances)
8. [Gestion des événements](#gestion-des-evenements)
9. [Animations et Transitions](#animations)
10. [Exemples pratiques](#exemples-pratiques)

---

## Introduction à Iced {#introduction}

Iced est une bibliothèque GUI cross-platform pour Rust, inspirée par Elm. Elle offre une architecture réactive et déclarative pour créer des interfaces graphiques modernes et performantes.

### Caractéristiques principales

- **Déclaratif** : Définissez votre UI comme une fonction de l'état
- **Réactif** : L'UI se met à jour automatiquement quand l'état change
- **Cross-platform** : Windows, macOS, Linux, Web (via WebAssembly)
- **Type-safe** : Exploite le système de types de Rust
- **Performant** : Rendu GPU avec wgpu

---

## Installation et Configuration {#installation}

### Cargo.toml

```toml
[dependencies]
iced = "0.12"

# Pour des fonctionnalités supplémentaires
iced = { version = "0.12", features = ["tokio", "image", "svg", "canvas"] }
```

### Structure de base

```rust
use iced::{Element, Sandbox, Settings};
use iced::widget::{button, column, container, text};

fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

struct MyApp {
    count: i32,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
}

impl Sandbox for MyApp {
    type Message = Message;

    fn new() -> Self {
        Self { count: 0 }
    }

    fn title(&self) -> String {
        String::from("Mon Application")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.count += 1,
            Message::Decrement => self.count -= 1,
        }
    }

    fn view(&self) -> Element<Message> {
        container(
            column![
                text(self.count).size(50),
                button("Incrémenter").on_press(Message::Increment),
                button("Décrémenter").on_press(Message::Decrement),
            ]
            .spacing(20)
        )
        .padding(20)
        .into()
    }
}
```

---

## Architecture de base {#architecture}

### Le pattern Elm Architecture

Iced suit le pattern Elm Architecture avec trois composants principaux :

1. **Model (État)** : Structure qui contient l'état de l'application
2. **Update** : Fonction qui modifie l'état en réponse aux messages
3. **View** : Fonction qui génère l'UI à partir de l'état

### Sandbox vs Application

#### Sandbox
Pour des applications simples sans effets secondaires :

```rust
impl Sandbox for MyApp {
    type Message = Message;
    
    fn new() -> Self { /* ... */ }
    fn title(&self) -> String { /* ... */ }
    fn update(&mut self, message: Message) { /* ... */ }
    fn view(&self) -> Element<Message> { /* ... */ }
}
```

#### Application
Pour des applications complexes avec effets secondaires (async, IO, etc.) :

```rust
impl Application for MyApp {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;
    
    fn new(flags: Self::Flags) -> (Self, Command<Message>) { /* ... */ }
    fn title(&self) -> String { /* ... */ }
    fn update(&mut self, message: Message) -> Command<Message> { /* ... */ }
    fn view(&self) -> Element<Message> { /* ... */ }
}
```

---

## Widgets de base {#widgets-de-base}

### Text

Affichage de texte avec options de styling :

```rust
use iced::widget::text;
use iced::{Color, Font};
use iced::alignment::{Horizontal, Vertical};

// Texte simple
text("Hello, World!")

// Texte avec taille
text("Grand texte").size(48)

// Texte avec couleur
text("Texte coloré").style(Color::from_rgb(1.0, 0.0, 0.0))

// Texte avec alignement
text("Centré")
    .horizontal_alignment(Horizontal::Center)
    .vertical_alignment(Vertical::Center)

// Texte avec police personnalisée
text("Police custom")
    .font(Font::with_name("Arial"))
    .size(20)
```

### Button

Bouton interactif :

```rust
use iced::widget::button;

// Bouton simple
button("Cliquez-moi").on_press(Message::ButtonClicked)

// Bouton sans action (désactivé visuellement)
button("Désactivé")

// Bouton avec contenu personnalisé
button(
    row![
        text("🚀"),
        text("Lancer").size(16),
    ]
    .spacing(10)
)
.on_press(Message::Launch)
.padding(15)

// Bouton avec style
button("Style custom")
    .on_press(Message::Action)
    .style(theme::Button::Primary)
```

### TextInput

Champ de saisie de texte :

```rust
use iced::widget::text_input;

// Input simple
text_input("Placeholder...", &self.input_value)
    .on_input(Message::InputChanged)
    .on_submit(Message::InputSubmitted)

// Input avec paramètres
text_input("Email", &self.email)
    .on_input(Message::EmailChanged)
    .padding(10)
    .size(20)
    .icon(text_input::Icon {
        font: Font::default(),
        code_point: '📧',
        size: Some(20.0),
        spacing: 10.0,
        side: text_input::Side::Left,
    })

// Input mot de passe
text_input("Mot de passe", &self.password)
    .on_input(Message::PasswordChanged)
    .secure(true)
    .padding(10)
```

### Checkbox

Case à cocher :

```rust
use iced::widget::checkbox;

checkbox("J'accepte les conditions", self.accepted)
    .on_toggle(Message::CheckboxToggled)
    .size(20)
    .text_size(16)
    .spacing(10)
```

### Radio

Bouton radio pour sélection unique :

```rust
use iced::widget::radio;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice {
    Option1,
    Option2,
    Option3,
}

// Dans la view
column![
    radio("Option 1", Choice::Option1, Some(self.selected), Message::RadioSelected),
    radio("Option 2", Choice::Option2, Some(self.selected), Message::RadioSelected),
    radio("Option 3", Choice::Option3, Some(self.selected), Message::RadioSelected),
]
.spacing(10)
```

### Slider

Curseur pour sélection de valeur :

```rust
use iced::widget::slider;

slider(0.0..=100.0, self.value, Message::SliderChanged)
    .step(1.0)

// Slider vertical
slider(0.0..=100.0, self.value, Message::SliderChanged)
    .step(0.5)
    .width(20)
```

### ProgressBar

Barre de progression :

```rust
use iced::widget::progress_bar;

progress_bar(0.0..=100.0, self.progress)
    .height(20)
```

### Image

Affichage d'images :

```rust
use iced::widget::image;
use iced::widget::image::Handle;

// Image depuis un fichier
image(Handle::from_path("path/to/image.png"))
    .width(200)
    .height(200)

// Image depuis des bytes
image(Handle::from_memory(image_bytes))
    .content_fit(ContentFit::Cover)
```

### Svg

Affichage de SVG :

```rust
use iced::widget::svg;

svg(svg::Handle::from_path("icon.svg"))
    .width(50)
    .height(50)
    .style(|theme| svg::Appearance {
        color: Some(theme.palette().primary),
    })
```

### Space

Espace vide pour le layout :

```rust
use iced::widget::Space;
use iced::Length;

// Espace horizontal
Space::with_width(Length::Fixed(20.0))

// Espace vertical
Space::with_height(Length::Fixed(10.0))

// Espace qui remplit
Space::with_width(Length::Fill)
```

### Rule

Ligne de séparation :

```rust
use iced::widget::rule;
use iced::widget::rule::FillMode;

// Ligne horizontale
rule::horizontal(2)

// Ligne verticale
rule::vertical(2)

// Ligne avec style
rule::horizontal(1)
    .style(|theme| rule::Appearance {
        color: theme.palette().primary,
        width: 2,
        radius: 1.0.into(),
        fill_mode: FillMode::Full,
    })
```

---

## Containers et Layouts {#containers-et-layouts}

### Container

Enveloppe un widget avec padding, alignement et style :

```rust
use iced::widget::container;
use iced::alignment::{Horizontal, Vertical};
use iced::{Background, Color, Border};
use iced::Length;

// Container simple
container(text("Contenu"))
    .padding(20)

// Container avec dimensions
container(text("Contenu"))
    .width(Length::Fixed(300.0))
    .height(Length::Fixed(200.0))
    .padding(20)

// Container avec alignement
container(text("Centré"))
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .center_y()

// Container avec style personnalisé
container(text("Stylé"))
    .padding(20)
    .style(|theme| container::Appearance {
        background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.3))),
        border: Border {
            color: Color::from_rgb(0.3, 0.3, 0.8),
            width: 2.0,
            radius: 10.0.into(),
        },
        text_color: Some(Color::WHITE),
        ..Default::default()
    })
```

### Column

Layout vertical :

```rust
use iced::widget::column;
use iced::Alignment;

column![
    text("Premier"),
    text("Deuxième"),
    text("Troisième"),
]
.spacing(10)
.padding(20)
.align_items(Alignment::Center)
.width(Length::Fill)

// Avec itération
column(
    vec!["Item 1", "Item 2", "Item 3"]
        .into_iter()
        .map(|item| text(item).into())
)
.spacing(5)
```

### Row

Layout horizontal :

```rust
use iced::widget::row;

row![
    button("Gauche").on_press(Message::Left),
    Space::with_width(Length::Fill),
    button("Droite").on_press(Message::Right),
]
.spacing(10)
.padding(20)
.align_items(Alignment::Center)
```

### Scrollable

Zone avec défilement :

```rust
use iced::widget::scrollable;

scrollable(
    column(
        (0..100)
            .map(|i| text(format!("Item {}", i)).into())
            .collect()
    )
    .spacing(5)
)
.height(Length::Fixed(300.0))
.width(Length::Fill)

// Scrollable horizontal
scrollable(
    row(items).spacing(10)
)
.direction(scrollable::Direction::Horizontal)
```

### Stack

Empilement de widgets les uns sur les autres :

```rust
use iced::widget::stack;

stack![
    // Image de fond
    image(background_image).width(Length::Fill).height(Length::Fill),
    
    // Contenu par-dessus
    container(
        column![
            text("Titre").size(40),
            text("Sous-titre").size(20),
        ]
    )
    .center_x()
    .center_y()
    .width(Length::Fill)
    .height(Length::Fill),
]
```

### Responsive

Adaptation responsive selon la taille de l'écran :

```rust
use iced::widget::responsive;

responsive(|size| {
    let is_mobile = size.width < 600.0;
    
    if is_mobile {
        // Layout mobile
        column![
            header(),
            content(),
            sidebar(),
        ].into()
    } else {
        // Layout desktop
        row![
            sidebar().width(Length::Fixed(250.0)),
            content().width(Length::Fill),
        ].into()
    }
})
```

### PaneGrid

Grille de panneaux redimensionnables :

```rust
use iced::widget::pane_grid;

// Structure pour gérer les panes
struct State {
    panes: pane_grid::State<PaneContent>,
}

#[derive(Debug, Clone)]
enum Message {
    PaneResized(pane_grid::ResizeEvent),
    PaneDragged(pane_grid::DragEvent),
}

// Dans la view
let pane_grid = pane_grid(&self.panes, |id, pane, is_maximized| {
    pane_grid::Content::new(pane.view())
        .title_bar(pane_grid::TitleBar::new(format!("Pane {}", id)))
})
.on_resize(10, Message::PaneResized)
.on_drag(Message::PaneDragged);
```

### Tooltip

Info-bulle au survol :

```rust
use iced::widget::tooltip;

tooltip(
    button("Survolez-moi"),
    "Ceci est une info-bulle",
    tooltip::Position::Top,
)
.gap(10)
.padding(10)
.style(|theme| container::Appearance {
    background: Some(Background::Color(Color::BLACK)),
    text_color: Some(Color::WHITE),
    ..Default::default()
})
```

### Modal / Overlay

Superposition modale :

```rust
use iced::widget::modal;

modal(
    // Contenu principal
    base_content(),
    
    // Contenu de la modal (optionnel)
    self.show_modal.then(|| {
        container(
            column![
                text("Confirmation").size(24),
                text("Êtes-vous sûr ?"),
                row![
                    button("Annuler").on_press(Message::CloseModal),
                    button("Confirmer").on_press(Message::Confirm),
                ]
                .spacing(10),
            ]
            .spacing(20)
        )
        .padding(30)
        .style(modal_style)
    }),
)
.backdrop(Message::CloseModal)
```

---

## Styling et Thèmes {#styling-et-themes}

### Thèmes prédéfinis

Iced fournit plusieurs thèmes prêts à l'emploi :

```rust
use iced::Theme;

// Dans Application
impl Application for MyApp {
    type Theme = Theme;
    
    fn theme(&self) -> Theme {
        match self.theme_choice {
            ThemeChoice::Light => Theme::Light,
            ThemeChoice::Dark => Theme::Dark,
            ThemeChoice::Dracula => Theme::Dracula,
            ThemeChoice::Nord => Theme::Nord,
            ThemeChoice::SolarizedLight => Theme::SolarizedLight,
            ThemeChoice::SolarizedDark => Theme::SolarizedDark,
            ThemeChoice::GruvboxLight => Theme::GruvboxLight,
            ThemeChoice::GruvboxDark => Theme::GruvboxDark,
            ThemeChoice::CatppuccinLatte => Theme::CatppuccinLatte,
            ThemeChoice::CatppuccinFrappe => Theme::CatppuccinFrappe,
            ThemeChoice::CatppuccinMacchiato => Theme::CatppuccinMacchiato,
            ThemeChoice::CatppuccinMocha => Theme::CatppuccinMocha,
            ThemeChoice::TokyoNight => Theme::TokyoNight,
            ThemeChoice::TokyoNightStorm => Theme::TokyoNightStorm,
            ThemeChoice::TokyoNightLight => Theme::TokyoNightLight,
            ThemeChoice::KanagawaWave => Theme::KanagawaWave,
            ThemeChoice::KanagawaDragon => Theme::KanagawaDragon,
            ThemeChoice::KanagawaLotus => Theme::KanagawaLotus,
            ThemeChoice::Moonfly => Theme::Moonfly,
            ThemeChoice::Nightfly => Theme::Nightfly,
            ThemeChoice::Oxocarbon => Theme::Oxocarbon,
            ThemeChoice::Ferra => Theme::Ferra,
        }
    }
}
```

### Palette de couleurs

Accéder aux couleurs du thème :

```rust
use iced::Theme;

fn custom_style(theme: &Theme) -> container::Appearance {
    let palette = theme.palette();
    
    container::Appearance {
        background: Some(Background::Color(palette.background)),
        text_color: Some(palette.text),
        border: Border {
            color: palette.primary,
            width: 2.0,
            radius: 5.0.into(),
        },
        ..Default::default()
    }
}
```

### Styles de widgets

#### Button Styles

```rust
// Styles prédéfinis
button("Primary").style(theme::Button::Primary)
button("Secondary").style(theme::Button::Secondary)
button("Positive").style(theme::Button::Positive)
button("Destructive").style(theme::Button::Destructive)
button("Text").style(theme::Button::Text)

// Style personnalisé
button("Custom")
    .style(|theme, status| {
        let palette = theme.palette();
        
        match status {
            button::Status::Active => button::Appearance {
                background: Some(Background::Color(palette.primary)),
                text_color: Color::WHITE,
                border: Border {
                    color: palette.primary,
                    width: 2.0,
                    radius: 8.0.into(),
                },
                shadow_offset: Vector::new(0.0, 2.0),
            },
            button::Status::Hovered => button::Appearance {
                background: Some(Background::Color(Color {
                    r: palette.primary.r * 0.8,
                    g: palette.primary.g * 0.8,
                    b: palette.primary.b * 0.8,
                    a: 1.0,
                })),
                text_color: Color::WHITE,
                border: Border {
                    color: palette.primary,
                    width: 2.0,
                    radius: 8.0.into(),
                },
                shadow_offset: Vector::new(0.0, 4.0),
            },
            button::Status::Pressed => button::Appearance {
                background: Some(Background::Color(palette.primary)),
                text_color: Color::WHITE,
                border: Border {
                    color: palette.primary,
                    width: 2.0,
                    radius: 8.0.into(),
                },
                shadow_offset: Vector::new(0.0, 0.0),
            },
            button::Status::Disabled => button::Appearance {
                background: Some(Background::Color(Color {
                    a: 0.3,
                    ..palette.primary
                })),
                text_color: Color {
                    a: 0.5,
                    ..Color::WHITE
                },
                ..Default::default()
            },
        }
    })
```

#### Container Styles

```rust
// Container avec bordure arrondie et ombre
container(content)
    .style(|theme| {
        let palette = theme.palette();
        
        container::Appearance {
            background: Some(Background::Color(palette.background)),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 15.0.into(),
            },
            text_color: Some(palette.text),
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                offset: Vector::new(0.0, 4.0),
                blur_radius: 10.0,
            },
        }
    })

// Container avec gradient
container(content)
    .style(|theme| {
        container::Appearance {
            background: Some(Background::Gradient(
                Gradient::Linear(
                    gradient::Linear::new(Degrees(45.0))
                        .add_stop(0.0, Color::from_rgb(0.2, 0.4, 0.8))
                        .add_stop(1.0, Color::from_rgb(0.8, 0.2, 0.6))
                )
            )),
            ..Default::default()
        }
    })
```

#### TextInput Styles

```rust
text_input("Placeholder", &self.value)
    .on_input(Message::InputChanged)
    .style(|theme, status| {
        let palette = theme.palette();
        
        text_input::Appearance {
            background: Background::Color(palette.background),
            border: Border {
                color: match status {
                    text_input::Status::Active => palette.text,
                    text_input::Status::Hovered => palette.primary,
                    text_input::Status::Focused => palette.primary,
                    text_input::Status::Disabled => Color {
                        a: 0.3,
                        ..palette.text
                    },
                },
                width: 1.0,
                radius: 5.0.into(),
            },
            icon_color: palette.text,
        }
    })
```

### Thème personnalisé complet

```rust
use iced::{Color, Theme};
use iced::theme::Palette;

fn create_custom_theme() -> Theme {
    Theme::custom(Palette {
        background: Color::from_rgb(0.95, 0.95, 0.97),
        text: Color::from_rgb(0.2, 0.2, 0.2),
        primary: Color::from_rgb(0.3, 0.5, 0.9),
        success: Color::from_rgb(0.3, 0.8, 0.4),
        danger: Color::from_rgb(0.9, 0.3, 0.3),
    })
}
```

---

## Widgets avancés {#widgets-avances}

### Canvas

Dessin personnalisé avec Canvas :

```rust
use iced::widget::canvas;
use iced::widget::canvas::{Cache, Cursor, Frame, Geometry, Path, Stroke};
use iced::{Point, Rectangle, Size, Theme};

struct MyCanvas {
    cache: Cache,
}

impl canvas::Program<Message> for MyCanvas {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            // Dessiner un cercle
            let circle = Path::circle(
                Point::new(bounds.width / 2.0, bounds.height / 2.0),
                50.0,
            );
            
            frame.fill(&circle, Color::from_rgb(0.3, 0.5, 0.9));
            frame.stroke(
                &circle,
                Stroke::default()
                    .with_width(2.0)
                    .with_color(Color::BLACK),
            );
            
            // Dessiner une ligne
            let line = Path::line(
                Point::new(0.0, 0.0),
                Point::new(bounds.width, bounds.height),
            );
            
            frame.stroke(
                &line,
                Stroke::default()
                    .with_width(3.0)
                    .with_color(Color::from_rgb(0.9, 0.3, 0.3)),
            );
        });

        vec![geometry]
    }
}

// Dans la view
canvas(MyCanvas { cache: Cache::new() })
    .width(Length::Fixed(400.0))
    .height(Length::Fixed(400.0))
```

### PickList (Dropdown)

Menu déroulant :

```rust
use iced::widget::pick_list;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Language {
    Rust,
    Python,
    JavaScript,
    Go,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Language::Rust => "Rust",
            Language::Python => "Python",
            Language::JavaScript => "JavaScript",
            Language::Go => "Go",
        })
    }
}

// Dans la view
pick_list(
    &[Language::Rust, Language::Python, Language::JavaScript, Language::Go][..],
    Some(self.selected_language),
    Message::LanguageSelected,
)
.placeholder("Choisir un langage...")
.padding(10)
```

### ComboBox

Combo box avec recherche :

```rust
use iced::widget::combo_box;

struct State {
    combo_state: combo_box::State<String>,
    selected: Option<String>,
}

// Initialisation
let options = vec![
    "Option 1".to_string(),
    "Option 2".to_string(),
    "Option 3".to_string(),
];

// Dans la view
combo_box(
    &self.combo_state,
    "Rechercher...",
    self.selected.as_ref(),
    Message::ComboBoxSelected,
)
.on_input(Message::ComboBoxInput)
```

### Toggler

Interrupteur on/off :

```rust
use iced::widget::toggler;

toggler(
    Some("Mode sombre".to_string()),
    self.dark_mode,
    Message::DarkModeToggled,
)
.width(Length::Shrink)
.spacing(10)
.text_size(16)
```

### QRCode

Affichage de QR codes :

```rust
use iced::widget::qr_code;

// Créer un QR code
let qr = qr_code::Data::new("https://example.com").unwrap();

// Dans la view
qr_code(&qr)
    .cell_size(4)
```

---

## Gestion des événements {#gestion-des-evenements}

### Messages et Update

```rust
#[derive(Debug, Clone)]
enum Message {
    // Messages simples
    ButtonClicked,
    InputChanged(String),
    CheckboxToggled(bool),
    
    // Messages avec données
    ItemSelected(usize),
    ValueUpdated(f32),
    
    // Messages asynchrones
    DataLoaded(Result<Data, Error>),
    TaskCompleted,
}

impl Application for MyApp {
    type Message = Message;

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ButtonClicked => {
                self.counter += 1;
                Command::none()
            }
            
            Message::InputChanged(value) => {
                self.input = value;
                Command::none()
            }
            
            Message::DataLoaded(result) => {
                match result {
                    Ok(data) => {
                        self.data = Some(data);
                        self.loading = false;
                    }
                    Err(e) => {
                        self.error = Some(e.to_string());
                        self.loading = false;
                    }
                }
                Command::none()
            }
            
            // Autres messages...
            _ => Command::none(),
        }
    }
}
```

### Commands (effets secondaires)

```rust
use iced::Command;

// Command simple
Command::none()

// Command avec batch (plusieurs commandes)
Command::batch(vec![
    Command::perform(async_task1(), Message::Task1Completed),
    Command::perform(async_task2(), Message::Task2Completed),
])

// Command asynchrone
fn load_data() -> Command<Message> {
    Command::perform(
        async {
            // Opération asynchrone
            let data = fetch_data().await?;
            Ok(data)
        },
        Message::DataLoaded,
    )
}

// Command avec délai
use iced::time;
use std::time::Duration;

Command::perform(
    time::sleep(Duration::from_secs(2)),
    |_| Message::DelayFinished,
)
```

### Subscriptions (événements continus)

```rust
use iced::{Subscription, time};
use std::time::Duration;

impl Application for MyApp {
    fn subscription(&self) -> Subscription<Message> {
        // Timer qui tick toutes les secondes
        time::every(Duration::from_secs(1))
            .map(|_| Message::Tick)
    }
}

// Subscription conditionnelle
fn subscription(&self) -> Subscription<Message> {
    if self.running {
        time::every(Duration::from_millis(16))
            .map(|_| Message::Update)
    } else {
        Subscription::none()
    }
}

// Subscription combinée
fn subscription(&self) -> Subscription<Message> {
    Subscription::batch(vec![
        time::every(Duration::from_secs(1)).map(|_| Message::Tick),
        keyboard::on_key_press(handle_key_press),
        window::events().map(Message::WindowEvent),
    ])
}
```

### Gestion du clavier

```rust
use iced::keyboard;
use iced::keyboard::{Key, key};

fn handle_key_press(key: Key, modifiers: keyboard::Modifiers) -> Option<Message> {
    match key.as_ref() {
        Key::Character("s") if modifiers.control() => {
            Some(Message::Save)
        }
        Key::Character("q") if modifiers.control() => {
            Some(Message::Quit)
        }
        Key::Named(key::Named::Escape) => {
            Some(Message::Cancel)
        }
        Key::Named(key::Named::Enter) => {
            Some(Message::Submit)
        }
        _ => None,
    }
}

// Dans subscription
keyboard::on_key_press(handle_key_press)
```

### Gestion de la souris

```rust
use iced::mouse;

// Dans Canvas pour interactions personnalisées
impl canvas::Program<Message> for MyCanvas {
    fn update(
        &self,
        state: &mut Self::State,
        event: canvas::Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        match event {
            canvas::Event::Mouse(mouse_event) => {
                match mouse_event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        if let Some(position) = cursor.position_in(bounds) {
                            return (
                                canvas::event::Status::Captured,
                                Some(Message::CanvasClicked(position)),
                            );
                        }
                    }
                    mouse::Event::CursorMoved { position } => {
                        if cursor.is_over(bounds) {
                            return (
                                canvas::event::Status::Captured,
                                Some(Message::CursorMoved(position)),
                            );
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        
        (canvas::event::Status::Ignored, None)
    }
}
```

---

## Animations et Transitions {#animations}

### Animations avec Canvas

```rust
use iced::widget::canvas;
use std::time::Instant;

struct AnimatedCanvas {
    start: Instant,
}

impl canvas::Program<Message> for AnimatedCanvas {
    type State = ();

    fn draw(&self, _state: &Self::State, renderer: &Renderer, theme: &Theme, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let elapsed = self.start.elapsed().as_secs_f32();
        
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            // Animation de rotation
            let angle = elapsed * std::f32::consts::PI;
            
            // Centre
            let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);
            
            // Dessiner un rectangle en rotation
            frame.with_save(|frame| {
                frame.translate(Vector::new(center.x, center.y));
                frame.rotate(angle);
                frame.translate(Vector::new(-25.0, -25.0));
                
                let rect = Path::rectangle(Point::ORIGIN, Size::new(50.0, 50.0));
                frame.fill(&rect, Color::from_rgb(0.3, 0.5, 0.9));
            });
        });

        vec![geometry]
    }
}

// Demander un redraw continu
fn subscription(&self) -> Subscription<Message> {
    time::every(Duration::from_millis(16)).map(|_| Message::Tick)
}
```

### Interpolation et easing

```rust
// Fonction d'easing
fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

// Interpolation linéaire
fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t
}

// Animation d'une valeur
struct Animation {
    from: f32,
    to: f32,
    duration: Duration,
    start_time: Instant,
}

impl Animation {
    fn current_value(&self) -> f32 {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        let duration = self.duration.as_secs_f32();
        let t = (elapsed / duration).min(1.0);
        let eased = ease_in_out_cubic(t);
        lerp(self.from, self.to, eased)
    }
    
    fn is_finished(&self) -> bool {
        self.start_time.elapsed() >= self.duration
    }
}
```

### Transitions de couleurs

```rust
fn interpolate_color(from: Color, to: Color, t: f32) -> Color {
    Color {
        r: lerp(from.r, to.r, t),
        g: lerp(from.g, to.g, t),
        b: lerp(from.b, to.b, t),
        a: lerp(from.a, to.a, t),
    }
}

// Utilisation dans un style animé
button("Hover me")
    .style(move |theme, status| {
        let base_color = theme.palette().primary;
        let hover_color = Color::from_rgb(0.9, 0.3, 0.3);
        
        let color = match status {
            button::Status::Hovered => {
                interpolate_color(base_color, hover_color, self.hover_progress)
            }
            _ => base_color,
        };
        
        button::Appearance {
            background: Some(Background::Color(color)),
            ..Default::default()
        }
    })
```

### Loading Spinner personnalisé

```rust
struct Spinner {
    rotation: f32,
}

impl canvas::Program<Message> for Spinner {
    type State = ();

    fn draw(&self, _state: &Self::State, renderer: &Renderer, theme: &Theme, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        
        let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);
        let radius = 20.0;
        
        // Dessiner l'arc de cercle
        let arc = Path::new(|builder| {
            builder.arc(canvas::path::Arc {
                center,
                radius,
                start_angle: self.rotation,
                end_angle: self.rotation + std::f32::consts::PI * 1.5,
            });
        });
        
        frame.stroke(
            &arc,
            Stroke::default()
                .with_width(3.0)
                .with_color(theme.palette().primary)
                .with_line_cap(canvas::LineCap::Round),
        );
        
        vec![frame.into_geometry()]
    }
}

// Mise à jour de la rotation
fn update(&mut self, message: Message) -> Command<Message> {
    match message {
        Message::Tick => {
            self.spinner_rotation += 0.1;
            if self.spinner_rotation > std::f32::consts::TAU {
                self.spinner_rotation = 0.0;
            }
            Command::none()
        }
        // ...
    }
}
```

---

## Exemples pratiques {#exemples-pratiques}

### Formulaire complet

```rust
use iced::widget::{button, checkbox, column, container, pick_list, row, text, text_input};

struct FormApp {
    name: String,
    email: String,
    age: String,
    country: Option<Country>,
    subscribe: bool,
    errors: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Country {
    France,
    USA,
    UK,
    Germany,
}

impl std::fmt::Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
enum Message {
    NameChanged(String),
    EmailChanged(String),
    AgeChanged(String),
    CountrySelected(Country),
    SubscribeToggled(bool),
    Submit,
}

impl Application for FormApp {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                name: String::new(),
                email: String::new(),
                age: String::new(),
                country: None,
                subscribe: false,
                errors: Vec::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Formulaire d'inscription")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NameChanged(name) => {
                self.name = name;
            }
            Message::EmailChanged(email) => {
                self.email = email;
            }
            Message::AgeChanged(age) => {
                self.age = age;
            }
            Message::CountrySelected(country) => {
                self.country = Some(country);
            }
            Message::SubscribeToggled(subscribe) => {
                self.subscribe = subscribe;
            }
            Message::Submit => {
                self.errors.clear();
                
                if self.name.is_empty() {
                    self.errors.push("Le nom est requis".to_string());
                }
                
                if !self.email.contains('@') {
                    self.errors.push("Email invalide".to_string());
                }
                
                if self.age.parse::<u32>().is_err() {
                    self.errors.push("Âge invalide".to_string());
                }
                
                if self.country.is_none() {
                    self.errors.push("Veuillez sélectionner un pays".to_string());
                }
                
                if self.errors.is_empty() {
                    println!("Formulaire valide!");
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let title = text("Formulaire d'inscription")
            .size(32)
            .style(Color::from_rgb(0.2, 0.4, 0.8));

        let name_input = column![
            text("Nom :").size(16),
            text_input("Votre nom", &self.name)
                .on_input(Message::NameChanged)
                .padding(10),
        ]
        .spacing(5);

        let email_input = column![
            text("Email :").size(16),
            text_input("votre.email@example.com", &self.email)
                .on_input(Message::EmailChanged)
                .padding(10),
        ]
        .spacing(5);

        let age_input = column![
            text("Âge :").size(16),
            text_input("18", &self.age)
                .on_input(Message::AgeChanged)
                .padding(10),
        ]
        .spacing(5);

        let country_picker = column![
            text("Pays :").size(16),
            pick_list(
                &[Country::France, Country::USA, Country::UK, Country::Germany][..],
                self.country,
                Message::CountrySelected,
            )
            .placeholder("Sélectionnez un pays...")
            .padding(10),
        ]
        .spacing(5);

        let subscribe_checkbox = checkbox(
            "S'abonner à la newsletter",
            self.subscribe,
        )
        .on_toggle(Message::SubscribeToggled);

        let submit_button = button("Soumettre")
            .on_press(Message::Submit)
            .padding([10, 40])
            .style(theme::Button::Primary);

        let errors = if !self.errors.is_empty() {
            column(
                self.errors
                    .iter()
                    .map(|error| {
                        text(format!("❌ {}", error))
                            .style(Color::from_rgb(0.9, 0.2, 0.2))
                            .into()
                    })
                    .collect(),
            )
            .spacing(5)
        } else {
            column![]
        };

        let content = column![
            title,
            name_input,
            email_input,
            age_input,
            country_picker,
            subscribe_checkbox,
            submit_button,
            errors,
        ]
        .spacing(20)
        .max_width(400);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(20)
            .into()
    }
}
```

### Dashboard avec graphiques

```rust
use iced::widget::{canvas, column, container, row, text};
use iced::widget::canvas::{Cache, Cursor, Frame, Geometry, Path, Stroke};

struct Dashboard {
    data: Vec<f32>,
    cache: Cache,
}

struct ChartCanvas {
    data: Vec<f32>,
    cache: Cache,
}

impl canvas::Program<Message> for ChartCanvas {
    type State = ();

    fn draw(&self, _state: &Self::State, renderer: &Renderer, theme: &Theme, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            // Dessiner les axes
            let axis_color = Color::from_rgb(0.5, 0.5, 0.5);
            
            // Axe X
            let x_axis = Path::line(
                Point::new(50.0, bounds.height - 50.0),
                Point::new(bounds.width - 20.0, bounds.height - 50.0),
            );
            frame.stroke(&x_axis, Stroke::default().with_width(2.0).with_color(axis_color));
            
            // Axe Y
            let y_axis = Path::line(
                Point::new(50.0, 20.0),
                Point::new(50.0, bounds.height - 50.0),
            );
            frame.stroke(&y_axis, Stroke::default().with_width(2.0).with_color(axis_color));
            
            // Dessiner les données
            if self.data.len() > 1 {
                let max_value = self.data.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
                let chart_width = bounds.width - 70.0;
                let chart_height = bounds.height - 70.0;
                let step = chart_width / (self.data.len() - 1) as f32;
                
                let mut path_builder = canvas::path::Builder::new();
                
                for (i, &value) in self.data.iter().enumerate() {
                    let x = 50.0 + i as f32 * step;
                    let y = bounds.height - 50.0 - (value / max_value) * chart_height;
                    
                    if i == 0 {
                        path_builder.move_to(Point::new(x, y));
                    } else {
                        path_builder.line_to(Point::new(x, y));
                    }
                    
                    // Dessiner un point
                    let point = Path::circle(Point::new(x, y), 4.0);
                    frame.fill(&point, theme.palette().primary);
                }
                
                let line = path_builder.build();
                frame.stroke(
                    &line,
                    Stroke::default()
                        .with_width(3.0)
                        .with_color(theme.palette().primary),
                );
            }
        });

        vec![geometry]
    }
}

impl Application for Dashboard {
    // ... implémentation

    fn view(&self) -> Element<Message> {
        let header = container(
            text("Dashboard Analytics")
                .size(32)
        )
        .width(Length::Fill)
        .padding(20)
        .style(|theme| container::Appearance {
            background: Some(Background::Color(theme.palette().primary)),
            text_color: Some(Color::WHITE),
            ..Default::default()
        });

        let stats = row![
            stat_card("Utilisateurs", "1,234", "↑ 12%", Color::from_rgb(0.3, 0.8, 0.4)),
            stat_card("Revenus", "45,678€", "↑ 23%", Color::from_rgb(0.3, 0.5, 0.9)),
            stat_card("Conversions", "89%", "↓ 3%", Color::from_rgb(0.9, 0.3, 0.3)),
        ]
        .spacing(20)
        .padding(20);

        let chart = container(
            canvas(ChartCanvas {
                data: self.data.clone(),
                cache: Cache::new(),
            })
            .width(Length::Fill)
            .height(Length::Fixed(400.0))
        )
        .padding(20);

        column![header, stats, chart]
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn stat_card<'a>(label: &'a str, value: &'a str, change: &'a str, color: Color) -> Element<'a, Message> {
    container(
        column![
            text(label).size(14),
            text(value).size(32),
            text(change).size(16).style(color),
        ]
        .spacing(10)
    )
    .width(Length::Fill)
    .padding(20)
    .style(move |theme| container::Appearance {
        background: Some(Background::Color(theme.palette().background)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 10.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 10.0,
        },
        ..Default::default()
    })
    .into()
}
```

### Application de chat

```rust
struct ChatApp {
    messages: Vec<ChatMessage>,
    input: String,
    scroll_state: scrollable::State,
}

struct ChatMessage {
    author: String,
    content: String,
    timestamp: String,
    is_own: bool,
}

impl Application for ChatApp {
    fn view(&self) -> Element<Message> {
        let messages_view = scrollable(
            column(
                self.messages
                    .iter()
                    .map(|msg| message_bubble(msg).into())
                    .collect()
            )
            .spacing(10)
            .padding(20)
        )
        .height(Length::Fill)
        .id(scrollable::Id::new("chat_scroll"));

        let input_row = row![
            text_input("Tapez votre message...", &self.input)
                .on_input(Message::InputChanged)
                .on_submit(Message::SendMessage)
                .padding(15)
                .width(Length::Fill),
            button("Envoyer")
                .on_press(Message::SendMessage)
                .padding([15, 30])
                .style(theme::Button::Primary),
        ]
        .spacing(10)
        .padding(20);

        column![
            messages_view,
            input_row,
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

fn message_bubble(msg: &ChatMessage) -> Element<Message> {
    let bubble = container(
        column![
            text(&msg.author).size(12).style(Color::from_rgba(1.0, 1.0, 1.0, 0.7)),
            text(&msg.content).size(16),
            text(&msg.timestamp).size(10).style(Color::from_rgba(1.0, 1.0, 1.0, 0.5)),
        ]
        .spacing(5)
    )
    .padding(15)
    .max_width(400)
    .style(move |theme| {
        let base_color = if msg.is_own {
            theme.palette().primary
        } else {
            Color::from_rgb(0.3, 0.3, 0.3)
        };
        
        container::Appearance {
            background: Some(Background::Color(base_color)),
            border: Border {
                radius: 15.0.into(),
                ..Default::default()
            },
            text_color: Some(Color::WHITE),
            ..Default::default()
        }
    });

    let aligned = if msg.is_own {
        row![Space::with_width(Length::Fill), bubble]
    } else {
        row![bubble, Space::with_width(Length::Fill)]
    };

    aligned.into()
}
```

### Application TODO liste

```rust
struct TodoApp {
    todos: Vec<Todo>,
    input: String,
    filter: Filter,
}

struct Todo {
    id: usize,
    description: String,
    completed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Filter {
    All,
    Active,
    Completed,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    AddTodo,
    ToggleTodo(usize),
    DeleteTodo(usize),
    FilterChanged(Filter),
    ClearCompleted,
}

impl Application for TodoApp {
    fn view(&self) -> Element<Message> {
        let title = text("TODO LIST")
            .size(40)
            .style(Color::from_rgb(0.3, 0.5, 0.9));

        let input_row = row![
            text_input("Nouvelle tâche...", &self.input)
                .on_input(Message::InputChanged)
                .on_submit(Message::AddTodo)
                .padding(10)
                .width(Length::Fill),
            button("Ajouter")
                .on_press(Message::AddTodo)
                .padding([10, 20])
                .style(theme::Button::Primary),
        ]
        .spacing(10);

        let filtered_todos: Vec<&Todo> = self.todos
            .iter()
            .filter(|todo| match self.filter {
                Filter::All => true,
                Filter::Active => !todo.completed,
                Filter::Completed => todo.completed,
            })
            .collect();

        let todos_list = column(
            filtered_todos
                .iter()
                .map(|todo| todo_item(todo).into())
                .collect()
        )
        .spacing(10);

        let filters = row![
            filter_button("Toutes", Filter::All, self.filter),
            filter_button("Actives", Filter::Active, self.filter),
            filter_button("Complétées", Filter::Completed, self.filter),
        ]
        .spacing(10);

        let footer = row![
            text(format!("{} tâches restantes", 
                self.todos.iter().filter(|t| !t.completed).count()
            )),
            Space::with_width(Length::Fill),
            button("Effacer complétées")
                .on_press(Message::ClearCompleted)
                .style(theme::Button::Destructive),
        ]
        .spacing(20);

        let content = column![
            title,
            input_row,
            Space::with_height(20),
            filters,
            Space::with_height(10),
            scrollable(todos_list).height(Length::Fill),
            Space::with_height(10),
            footer,
        ]
        .spacing(10)
        .max_width(600);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .padding(20)
            .into()
    }
}

fn todo_item(todo: &Todo) -> Element<Message> {
    container(
        row![
            checkbox("", todo.completed)
                .on_toggle(move |_| Message::ToggleTodo(todo.id))
                .size(20),
            text(&todo.description)
                .style(if todo.completed {
                    Color::from_rgba(0.5, 0.5, 0.5, 0.8)
                } else {
                    Color::BLACK
                })
                .width(Length::Fill),
            button("🗑")
                .on_press(Message::DeleteTodo(todo.id))
                .style(theme::Button::Destructive),
        ]
        .spacing(15)
        .align_items(Alignment::Center)
    )
    .padding(15)
    .width(Length::Fill)
    .style(|theme| container::Appearance {
        background: Some(Background::Color(theme.palette().background)),
        border: Border {
            color: Color::from_rgb(0.9, 0.9, 0.9),
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    })
    .into()
}

fn filter_button(label: &str, filter: Filter, current: Filter) -> Element<Message> {
    let style = if filter == current {
        theme::Button::Primary
    } else {
        theme::Button::Secondary
    };

    button(label)
        .on_press(Message::FilterChanged(filter))
        .padding([8, 15])
        .style(style)
        .into()
}
```

---

## Bonnes pratiques

### Organisation du code

```rust
// main.rs
mod app;
mod views;
mod styles;
mod messages;

use app::MyApp;

fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

// app.rs
pub struct MyApp {
    // État de l'application
}

// views.rs
pub fn main_view(state: &AppState) -> Element<Message> {
    // Construction de la vue
}

// styles.rs
pub fn primary_button() -> impl Fn(&Theme, button::Status) -> button::Appearance {
    // Styles réutilisables
}

// messages.rs
#[derive(Debug, Clone)]
pub enum Message {
    // Messages de l'application
}
```

### Performance

1. **Utilisez Cache pour Canvas** : Évitez de redessiner du contenu statique
2. **Lazy dans Column/Row** : Pour de grandes listes
3. **Conditional rendering** : N'affichez que ce qui est nécessaire
4. **Avoid cloning** : Utilisez des références quand possible

### Accessibilité

- Utilisez des tailles de texte lisibles (minimum 14-16px)
- Assurez un bon contraste de couleurs
- Fournissez des labels pour les widgets interactifs
- Testez la navigation au clavier

---

## Ressources additionnelles

- **Documentation officielle** : https://docs.iced.rs
- **Exemples** : https://github.com/iced-rs/iced/tree/master/examples
- **Discord** : Communauté active pour poser des questions
- **Awesome Iced** : Liste de projets et ressources

---

Cette documentation couvre les aspects essentiels d'Iced pour créer des interfaces graphiques variées et attrayantes. N'hésitez pas à expérimenter et combiner les différents widgets et techniques pour créer votre application unique!
