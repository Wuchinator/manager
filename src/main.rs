use eframe::egui;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Task {
    #[allow(dead_code)]
    id: u32,
    description: String,
    completed: bool,
}

struct TaskerApp {
    tasks: HashMap<u32, Task>,
    next_id: u32,
    new_task_description: String,
}

impl TaskerApp {
    fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            next_id: 1,
            new_task_description: String::new(),
        }
    }

    fn add_task(&mut self) {
        if !self.new_task_description.trim().is_empty() {
            let task = Task {
                id: self.next_id,
                description: self.new_task_description.trim().to_string(),
                completed: false,
            };
            self.tasks.insert(self.next_id, task);
            self.next_id += 1;
            self.new_task_description.clear();
        }
    }

    fn remove_task(&mut self, id: u32) {
        self.tasks.remove(&id);
    }
}

impl eframe::App for TaskerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tasker");


            ui.horizontal(|ui| {
                let text_edit = egui::TextEdit::singleline(&mut self.new_task_description)
                    .hint_text("Введите новую задачу");
                ui.add(text_edit);
                
                if ui.button("Добавить").clicked() {
                    self.add_task();
                }
            });

            ui.separator();

            let mut tasks_to_toggle = Vec::new();
            let mut tasks_to_remove = Vec::new();

            for (id, task) in &self.tasks {
                ui.horizontal(|ui| {
                    let mut completed = task.completed;
                    if ui.checkbox(&mut completed, "").changed() {
                        tasks_to_toggle.push((*id, completed));
                    }
                    
                    if task.completed {
                        ui.add(egui::Label::new(
                            egui::RichText::new(&task.description).strikethrough(),
                        ));
                    } else {
                        ui.label(&task.description);
                    }

                    if ui.button("❌").clicked() {
                        tasks_to_remove.push(*id);
                    }
                });
            }
            for (id, completed) in tasks_to_toggle {
                if let Some(task) = self.tasks.get_mut(&id) {
                    task.completed = completed;
                }
            }
            for id in tasks_to_remove {
                self.remove_task(id);
            }
            ui.separator();
            let completed_count = self.tasks.values().filter(|t| t.completed).count();
            ui.label(format!(
                "Всего задач: {} | Выполнено: {}",
                self.tasks.len(),
                completed_count
            ));
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 500.0]),
        ..Default::default()
    };
    
    let _ = eframe::run_native(
        "Tasker",
        options,
        Box::new(|_cc| Box::new(TaskerApp::new())),
    );
}