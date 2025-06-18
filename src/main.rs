mod lexer;
mod parser;

use eframe::egui;
use lexer::Lexer;
use parser::{Context, Parser};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Intérprete en Rust", options, Box::new(|_cc| Box::new(App::default())))
}

/// Estructura principal de la interfaz gráfica.
#[derive(Default)]
struct App {
    code_input: String,
    output: String,
    has_error: bool,
    history: Vec<String>, // Historial de entradas
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Intérprete de Lenguaje Simple");

            ui.label("Escribe tu código:");
            ui.add(egui::TextEdit::multiline(&mut self.code_input).desired_rows(10));

            ui.horizontal(|ui| {
                if ui.button("Ejecutar").clicked() {
                    let trimmed = self.code_input.trim();
                    if !trimmed.is_empty() && self.history.last() != Some(&trimmed.to_string()) {
                        self.history.push(trimmed.to_string());
                    }

                    let (result, error) = interpretar(&self.code_input);
                    self.output = result;
                    self.has_error = error;
                }

                if ui.button("Limpiar").clicked() {
                    self.code_input.clear();
                    self.output.clear();
                    self.has_error = false;
                }
            });

            ui.separator();

            // Historial de entradas
            if !self.history.is_empty() {
                ui.label("Historial de entradas:");
                egui::ScrollArea::vertical()
                    .id_source("history_scroll")
                    .max_height(100.0)
                    .show(ui, |ui| {
                        for entry in self.history.iter().rev() {
                            if ui.button(entry).clicked() {
                                self.code_input = entry.clone();
                            }
                        }
                    });
            }

            ui.separator();
            ui.label("Salida:");

            egui::ScrollArea::vertical()
                .id_source("output_scroll")
                .max_height(200.0)
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    let output_style = if self.has_error {
                        egui::TextStyle::Monospace
                    } else {
                        egui::TextStyle::Body
                    };

                    let color = if self.has_error {
                        egui::Color32::RED
                    } else {
                        egui::Color32::WHITE
                    };

                    ui.colored_label(
                        color,
                        egui::RichText::new(&self.output)
                            .monospace()
                            .text_style(output_style),
                    );
                });
        });
    }
}

/// Ejecuta el código fuente ingresado y devuelve el resultado o errores.
fn interpretar(source: &str) -> (String, bool) {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);

    let statements = match parser.parse_statements() {
        Ok(s) => s,
        Err(e) => return (format!("Error de sintaxis: {}", e), true),
    };

    let mut ctx = Context::new();
    let mut output = String::new();
    let mut has_error = false;

    for stmt in statements {
        match stmt.execute(&mut ctx, &mut output) {
            Ok(_) => {}
            Err(e) => {
                output.push_str(&format!("Error de ejecución: {}\n", e));
                has_error = true;
                break;
            }
        }
    }

    if output.trim().is_empty() && !has_error {
        output.push_str("// Ejecución completada sin errores.");
    }

    (output, has_error)
}
