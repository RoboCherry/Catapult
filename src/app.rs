use open::{self, that};
use egui::{Key, RichText, ThemePreference, Vec2};
use rfd::FileDialog;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ChauncerApp {
    apps : Vec<String>
}

impl Default for ChauncerApp {
    fn default() -> Self {
        Self {
            apps : Vec::new()
        }
    }
}

impl ChauncerApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for ChauncerApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        set_stylings(ctx);
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    if ui.button("Fullscreen").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(true));
                    }
                    if ui.button("Hover Window").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(false));
                    }
                });
                    ui.add_space(16.0);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Applications");
            
            if ui.input(|i| i.key_pressed(Key::Escape)){
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            if ui.button("Delete Apps [-]").clicked(){
                self.apps = Vec::new()
            }
            if ui.button("Add App [+]").clicked() {
                let files = FileDialog::new()
                .add_filter("Executable", &["exe"])
                .set_directory("C:/")
                .pick_file();
                
                let picked_path = files.expect("Holy moly this was supposed to be a file!");
                let path = picked_path.as_path();
                let exe_path = path.to_str().unwrap();
                if ! self.apps.contains(&exe_path.to_string()){
                    let _ = &mut self.apps.push(exe_path.to_string());
                }
            };
            ui.add_space(16.0);
            
            egui::ScrollArea::vertical().show(ui, |ui| {
                for i in &mut self.apps.iter(){
                    
                    let icon = egui::include_image!("../assets/icon.png");

                    ui.add(
                        egui::Image::new(icon)
                            .max_size(Vec2 { x: 32.0, y: 32.0 })
                    );
                    ui.add_space(8.0);
                    let text = RichText::new(get_executable_name(i)).size(16.0);
                    if ui.button(text).clicked(){
                        ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                        open_app(i);
                    };
                    ui.add_space(8.0);
                }
            });
        });
    }
}


#[warn(unused)]
pub struct OpenPopup {
    name : String,
    path : String
}

impl Default for OpenPopup {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            path: "".to_string()
        }
    }
}

impl OpenPopup {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()        
    }
}

impl eframe::App for OpenPopup{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let &mut mut name_text = &mut "yo";
            egui::TextEdit::singleline(&mut name_text).show(ui)
        });
    }
}


fn get_executable_name(path : &String) -> String{
    let mut new_path = path.clone();
    let split_path : Vec<&str> = new_path.split("\\").collect();
    new_path = (*split_path.get(split_path.iter().count() - 1).unwrap()).to_string().replace(".exe", "");
    new_path
}

fn set_stylings(ctx: &egui::Context){
    ctx.set_theme(ThemePreference::Dark);
    //ctx.set_style();
}

fn open_app(name : &String){
    let _ = that(name);
}