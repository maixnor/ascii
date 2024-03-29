pub mod convert;
use crate::app::convert::convert_to_ascii;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[derive(Default)]
pub struct App {
    dropped_files: Vec<egui::DroppedFile>,
    current_text: Option<String>,
}

impl App {
    /// Called once before the first frame.
    #[allow(dead_code)]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        side_panel(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(file) = self.dropped_files.first() {
                convert_image(&mut self, file);
            }
            match self.current_text {
                Some(text) => render_ascii(ui, text),
                None => {
                    ui.label("drag-and-drop image files to convert to ascii art!");
                }
            };
        });

        preview_files_being_dropped(ctx);

        // Collect dropped files:
        if !ctx.input().raw.dropped_files.is_empty() {
            self.dropped_files = ctx.input().raw.dropped_files.clone();
        }
    }
}

fn preview_files_being_dropped(ctx: &egui::Context) {
    use egui::*;
    use std::fmt::Write as _;

    if !ctx.input().raw.hovered_files.is_empty() {
        let mut text = "Working on File:\n".to_owned();
        for file in &ctx.input().raw.hovered_files {
            if let Some(path) = &file.path {
                write!(text, "\n{}", path.display()).ok();
            } else if !file.mime.is_empty() {
                write!(text, "\n{}", file.mime).ok();
            } else {
                text += "\n???";
            }
        }

        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

        let screen_rect = ctx.input().screen_rect();
        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
        painter.text(
            screen_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE,
        );
    }
}

fn side_panel(ctx: &egui::Context) {
    egui::SidePanel::left("side_panel").show(ctx, |ui| {
        ui.heading("Side Panel");

        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("powered by ");
                ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                ui.label(" and ");
                ui.hyperlink_to(
                    "eframe",
                    "https://github.com/emilk/egui/tree/master/crates/eframe",
                );
                ui.label(".");
            });
        });
        ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
            egui::warn_if_debug_build(ui);
        })
    });
}

#[cfg(target_arch = "wasm32")]
fn convert_image(app: &mut App, file: &egui::DroppedFile) {
    let bytes = file
        .bytes
        .as_ref()
        .expect("could not load bytes from dropped file");
    convert_image_bytes(app, bytes);
}

#[cfg(not(target_arch = "wasm32"))]
fn convert_image(app: &mut App, file: &egui::DroppedFile) {
    let bytes = std::io::Cursor::new(
        std::fs::read(
            file.path
                .as_ref()
                .expect("could not load path from dropped file")
                .display()
                .to_string(),
        )
        .expect("could not read from path")
        .as_slice(),
    );
    convert_image_bytes(app, bytes);
}

fn convert_image_bytes(app: &mut App, cursor: std::io::Cursor<&[u8]>) {
    let reader = image::io::Reader::new(cursor)
        .with_guessed_format()
        .unwrap();

    match reader.decode().ok() {
        Some(img) => app.current_text = Some(convert_to_ascii(img)),
        None => app.current_text = None,
    }
}

fn render_ascii(ui: &mut egui::Ui, text: String) {
    if ui
        .add(egui::Label::new(egui::RichText::new(text).monospace()).sense(egui::Sense::click()))
        .clicked()
    {
        ui.output().copied_text = text;
    }

    ui.label("Click the ASCII Art to Copy to Clipboard");
}

#[test]
#[allow(unused_variables)]
fn test_heart() {
    let image = image::open("/home/maixnor/Pictures/heart.png").unwrap();
    let ascii = convert_to_ascii(image);
    // did not crash!
    assert!(true)
}

#[test]
#[allow(unused_variables)]
fn test_pug() {
    let image = image::open("/home/maixnor/Pictures/pug.png").unwrap();
    let ascii = convert_to_ascii(image);
    // did not crash!
    assert!(true)
}
