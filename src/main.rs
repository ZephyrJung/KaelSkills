use eframe::egui;
use rand::Rng;
use std::collections::HashMap;
use std::path::Path;

#[derive(Clone, Copy, PartialEq)]
struct SkillCombo {
    ice: u8,
    light: u8,
    fire: u8,
}

// B C D F G T V X Y Z
// B: 111 (ice/light/fire)
// C: 030
// D: 012
// F: 102
// G: 201
// T: 003
// V: 210
// X: 120
// Y: 300
// Z: 021
const SKILLS: [(char, SkillCombo); 10] = [
    ('B', SkillCombo { ice: 1, light: 1, fire: 1 }),
    ('C', SkillCombo { ice: 0, light: 3, fire: 0 }),
    ('D', SkillCombo { ice: 0, light: 1, fire: 2 }),
    ('F', SkillCombo { ice: 1, light: 0, fire: 2 }),
    ('G', SkillCombo { ice: 2, light: 0, fire: 1 }),
    ('T', SkillCombo { ice: 0, light: 0, fire: 3 }),
    ('V', SkillCombo { ice: 2, light: 1, fire: 0 }),
    ('X', SkillCombo { ice: 1, light: 2, fire: 0 }),
    ('Y', SkillCombo { ice: 3, light: 0, fire: 0 }),
    ('Z', SkillCombo { ice: 0, light: 2, fire: 1 }),
];

struct KaelSkillsApp {
    state: GameState,
    current_input: String,
    target_skills: Vec<usize>,
    done_skills: Vec<bool>,
    current_count: usize,
    correct_count: usize,
    start_time: Option<std::time::Instant>,
    total_time: f64,
    texture_cache: HashMap<String, egui::TextureHandle>,
}

#[derive(PartialEq)]
enum GameState {
    Idle,
    Playing,
    Finished,
}

impl Default for KaelSkillsApp {
    fn default() -> Self {
        Self {
            state: GameState::Idle,
            current_input: String::new(),
            target_skills: Vec::new(),
            done_skills: Vec::new(),
            current_count: 0,
            correct_count: 0,
            start_time: None,
            total_time: 0.0,
            texture_cache: HashMap::new(),
        }
    }
}

fn check_skill(input: &str, expected: SkillCombo) -> bool {
    let mut ice = 0;
    let mut light = 0;
    let mut fire = 0;

    for c in input.chars() {
        match c.to_ascii_lowercase() {
            'q' => ice += 1,
            'w' => light += 1,
            'e' => fire += 1,
            'r' => {}, // confirm key
            _ => return false,
        }
    }

    ice == expected.ice && light == expected.light && fire == expected.fire
}

impl eframe::App for KaelSkillsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set a nicer style
        ctx.style_mut(|style| {
            style.spacing.item_spacing = egui::vec2(10.0, 10.0);
            style.spacing.window_margin = egui::Margin::same(20.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Title with larger font and centered alignment
            ui.vertical_centered(|ui| {
                ui.add_space(5.0);
                let heading = egui::RichText::new("KaelSkills 卡尔技能练习")
                    .size(24.0)
                    .color(egui::Color32::from_rgb(50, 50, 150));
                ui.heading(heading);
                ui.add_space(8.0);
            });

            // Start button - larger and more prominent
            ui.vertical_centered(|ui| {
                if self.state == GameState::Idle || self.state == GameState::Finished {
                    let button = egui::Button::new(egui::RichText::new("开始练习").size(18.0))
                        .min_size(egui::vec2(150.0, 40.0));
                    if ui.add(button).clicked() {
                        self.start_game();
                    }
                }
            });

            ui.add_space(15.0);

            if self.state != GameState::Idle {
                // Precompute all needed data to avoid borrowing issues
                let target_skills: Vec<(usize, String, char)> = self.target_skills
                    .iter()
                    .map(|&skill_idx| {
                        let (skill_char, _) = SKILLS[skill_idx];
                        let image_path = format!("src/{}.png", skill_char);
                        (skill_idx, image_path, skill_char)
                    })
                    .collect();

                // Target skills section with a framed container
                egui::Frame::group(ui.style())
                    .inner_margin(egui::Margin::symmetric(8.0, 8.0))
                    .show(ui, |ui| {
                        ui.heading("目标技能顺序");
                        ui.add_space(6.0);
                        ui.horizontal_wrapped(|ui| {
                            ui.spacing_mut().item_spacing = egui::vec2(5.0, 5.0);
                            for (skill_idx, image_path, skill_char) in target_skills {
                                let _ = skill_idx;
                                if let Some(texture) = self.load_image_cached(ctx, &image_path) {
                                    egui::Frame::none()
                                        .rounding(4.0)
                                        .inner_margin(egui::Margin::same(2.0))
                                        .fill(egui::Color32::from_rgb(240, 240, 240))
                                        .show(ui, |ui| {
                                            ui.add(egui::Image::new(texture).fit_to_exact_size(egui::vec2(32.0, 32.0)));
                                        });
                                } else {
                                    egui::Frame::none()
                                        .rounding(4.0)
                                        .inner_margin(egui::Margin::same(2.0))
                                        .fill(egui::Color32::from_rgb(240, 240, 240))
                                        .show(ui, |ui| {
                                            ui.centered_and_justified(|ui| {
                                                ui.label(egui::RichText::new(skill_char.to_string()).size(18.0));
                                            });
                                        });
                                }
                            }
                        });
                    });

                ui.add_space(8.0);

                // Done skills section - precompute data to avoid borrowing issues
                let done_skills: Vec<(char, bool, String)> = self.target_skills
                    .iter()
                    .enumerate()
                    .take(self.current_count)
                    .map(|(i, &skill_idx)| {
                        let (skill_char, _) = SKILLS[skill_idx];
                        let correct = self.done_skills[i];
                        let image_path = if correct {
                            format!("src/{}.png", skill_char)
                        } else {
                            "src/error.png".to_string()
                        };
                        (skill_char, correct, image_path)
                    })
                    .collect();

                egui::Frame::group(ui.style())
                    .inner_margin(egui::Margin::symmetric(8.0, 8.0))
                    .show(ui, |ui| {
                        ui.heading("已完成技能");
                        ui.add_space(6.0);
                        ui.horizontal_wrapped(|ui| {
                            ui.spacing_mut().item_spacing = egui::vec2(5.0, 5.0);
                            for (skill_char, correct, image_path) in done_skills {
                                let bg_color = if correct {
                                    egui::Color32::from_rgb(200, 240, 200)
                                } else {
                                    egui::Color32::from_rgb(240, 200, 200)
                                };
                                let border_color = if correct { egui::Color32::GREEN } else { egui::Color32::RED };

                                if let Some(texture) = self.load_image_cached(ctx, &image_path) {
                                    egui::Frame::none()
                                        .rounding(4.0)
                                        .inner_margin(egui::Margin::same(2.0))
                                        .fill(bg_color)
                                        .stroke(egui::Stroke::new(2.0, border_color))
                                        .show(ui, |ui| {
                                            ui.add(egui::Image::new(texture).fit_to_exact_size(egui::vec2(32.0, 32.0)));
                                        });
                                } else {
                                    let text_color = border_color;
                                    egui::Frame::none()
                                        .rounding(4.0)
                                        .inner_margin(egui::Margin::same(2.0))
                                        .fill(bg_color)
                                        .stroke(egui::Stroke::new(2.0, text_color))
                                        .show(ui, |ui| {
                                            ui.centered_and_justified(|ui| {
                                                ui.label(egui::RichText::new(skill_char.to_string()).size(18.0).color(text_color));
                                            });
                                        });
                                }
                            }
                        });
                    });

                ui.add_space(8.0);

                // Current input with highlighted container
                if self.state == GameState::Playing {
                    egui::Frame::none()
                        .rounding(8.0)
                        .fill(egui::Color32::from_rgb(245, 245, 250))
                        .inner_margin(egui::Margin::same(15.0))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.strong("当前输入:");
                                if !self.current_input.is_empty() {
                                    let text = egui::RichText::new(&self.current_input)
                                        .size(20.0)
                                        .color(egui::Color32::from_rgb(80, 80, 180));
                                    ui.label(text);
                                } else {
                                    ui.label(egui::RichText::new("(等待输入...)").color(egui::Color32::from_rgb(150, 150, 150)));
                                }
                            });

                            // Progress indicator
                            ui.add_space(5.0);
                            let progress = self.current_count as f32 / 10.0;
                            ui.label(format!("进度: {}/10", self.current_count));
                            ui.add(egui::ProgressBar::new(progress).show_percentage());
                        });
                }

                if self.state == GameState::Finished {
                    ui.add_space(10.0);
                    let accuracy = self.correct_count * 10;
                    let result_color = if accuracy >= 90 {
                        egui::Color32::from_rgb(0, 150, 0)
                    } else if accuracy >= 70 {
                        egui::Color32::from_rgb(180, 140, 0)
                    } else {
                        egui::Color32::from_rgb(180, 60, 0)
                    };

                    egui::Frame::none()
                        .rounding(10.0)
                        .fill(egui::Color32::from_rgb(245, 248, 255))
                        .inner_margin(egui::Margin::symmetric(20.0, 15.0))
                        .show(ui, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.label(egui::RichText::new("练习完成!").size(20.0).color(result_color));
                                ui.add_space(10.0);
                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        ui.label("用时");
                                        ui.label(egui::RichText::new(format!("{:.0}秒", self.total_time)).size(24.0));
                                    });
                                    ui.add_space(40.0);
                                    ui.vertical(|ui| {
                                        ui.label("准确率");
                                        ui.label(egui::RichText::new(format!("{}%", accuracy)).size(24.0).color(result_color));
                                    });
                                });
                            });
                        });
                }
            }

            ui.add_space(8.0);

            // Help section at the bottom
            egui::Frame::none()
                .rounding(8.0)
                .fill(egui::Color32::from_rgb(248, 248, 248))
                .inner_margin(egui::Margin::symmetric(10.0, 8.0))
                .show(ui, |ui| {
                    ui.strong("操作说明");
                    ui.add_space(3.0);
                    ui.label(egui::RichText::new("Q = 冰   |   W = 雷   |   E = 火   |   R = 确认技能   |   N = 重新开始").color(egui::Color32::from_rgb(80, 80, 80)));
                });
        });

        // Handle keyboard input
        if self.state == GameState::Playing || self.state == GameState::Finished {
            ctx.input(|i| {
                for event in &i.events {
                    if let egui::Event::Text(text) = event {
                        for c in text.chars() {
                            match c.to_ascii_lowercase() {
                                'q' | 'w' | 'e' => {
                                    if self.state == GameState::Playing {
                                        self.current_input.push(c.to_ascii_uppercase());
                                    }
                                }
                                'r' => {
                                    if self.state == GameState::Playing && self.current_count < 10 {
                                        if self.start_time.is_none() {
                                            self.start_time = Some(std::time::Instant::now());
                                        }
                                        let expected_idx = self.target_skills[self.current_count];
                                        let (_, expected_combo) = SKILLS[expected_idx];
                                        let correct = check_skill(&self.current_input, expected_combo);
                                        if correct {
                                            self.correct_count += 1;
                                        }
                                        self.done_skills.push(correct);
                                        self.current_count += 1;
                                        self.current_input.clear();

                                        if self.current_count == 10 {
                                            if let Some(start) = self.start_time {
                                                self.total_time = start.elapsed().as_secs_f64();
                                            }
                                            self.state = GameState::Finished;
                                        }
                                    }
                                }
                                'n' => { // N for new game when finished
                                    if self.state == GameState::Finished {
                                        self.start_game();
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            });
        }
    }
}

fn load_image(ctx: &egui::Context, path: &str) -> Option<egui::TextureHandle> {
    let path = Path::new(path);
    if !path.exists() {
        return None;
    }

    match image::open(path) {
        Ok(img) => {
            let rgba = img.to_rgba8();
            let size = egui::vec2(rgba.width() as f32, rgba.height() as f32);
            let pixels = rgba.into_raw();
            let texture = ctx.load_texture(
                path.file_name()?.to_string_lossy(),
                egui::ColorImage::from_rgba_unmultiplied(
                    [size.x as usize, size.y as usize],
                    &pixels
                ),
                Default::default()
            );
            Some(texture)
        }
        Err(_) => None
    }
}

impl KaelSkillsApp {
    fn load_image_cached(&mut self, ctx: &egui::Context, path: &str) -> Option<&egui::TextureHandle> {
        if !self.texture_cache.contains_key(path) {
            if let Some(texture) = load_image(ctx, path) {
                self.texture_cache.insert(path.to_string(), texture);
            }
        }
        self.texture_cache.get(path)
    }

    fn start_game(&mut self) {
        let mut rng = rand::thread_rng();
        self.target_skills.clear();
        self.done_skills.clear();
        self.current_input.clear();
        self.current_count = 0;
        self.correct_count = 0;
        self.start_time = None;
        self.total_time = 0.0;

        for _ in 0..10 {
            let idx = rng.gen_range(0..10);
            self.target_skills.push(idx);
        }

        self.state = GameState::Playing;
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(640.0, 520.0))
            .with_min_inner_size(egui::vec2(500.0, 400.0)),
        ..Default::default()
    };

    eframe::run_native(
        "KaelSkills - 卡尔技能练习",
        options,
        Box::new(|cc| {
            // 添加中文字体支持
            let mut fonts = egui::FontDefinitions::default();

            #[cfg(target_os = "macos")]
            {
                // macOS 系统中文字体
                let font_path = "/System/Library/Fonts/PingFang.ttc";
                if std::path::Path::new(font_path).exists() {
                    if let Ok(font_bytes) = std::fs::read(font_path) {
                        let font_data = egui::FontData::from_owned(font_bytes);
                        fonts.font_data.insert("pingfang".to_string(), font_data);
                        fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "pingfang".to_string());
                    }
                }
            }

            #[cfg(target_os = "windows")]
            {
                // Windows 系统中文字体
                let font_path = "C:/Windows/Fonts/msyh.ttc";
                if std::path::Path::new(font_path).exists() {
                    if let Ok(font_bytes) = std::fs::read(font_path) {
                        let font_data = egui::FontData::from_owned(font_bytes);
                        fonts.font_data.insert("microsoft-yahei".to_string(), font_data);
                        fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "microsoft-yahei".to_string());
                    }
                }
            }

            #[cfg(target_os = "linux")]
            {
                // Linux 系统中文字体
                let font_paths = [
                    "/usr/share/fonts/truetype/wqy/wqy-microhei.ttc",
                    "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
                ];
                for font_path in font_paths {
                    if std::path::Path::new(font_path).exists() {
                        if let Ok(font_bytes) = std::fs::read(font_path) {
                            let font_data = egui::FontData::from_owned(font_bytes);
                            fonts.font_data.insert("chinese".to_string(), font_data);
                            fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "chinese".to_string());
                            break;
                        }
                    }
                }
            }

            cc.egui_ctx.set_fonts(fonts);

            Box::new(KaelSkillsApp::default())
        })
    )
}
