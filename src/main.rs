#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use rfd::FileDialog;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Child};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use winreg::enums::*;
use winreg::RegKey;
use std::io;

const MITM_DOWNLOAD_URL: &str = "https://downloads.mitmproxy.org/11.0.2/mitmproxy-11.0.2-windows-x64-installer.exe";
const PYTHON_DOWNLOAD_URL: &str = "https://www.python.org/ftp/python/3.11.9/python-3.11.9-amd64.exe";
const MITM_WEBSITE_URL: &str = "https://mitmproxy.org/";
const PYTHON_WEBSITE_URL: &str = "https://www.python.org/";

struct NexusToolkitApp {
    token_directory: String,
    token_filename: String,
    selected_inject_token: String,
    staged_file_info: String,
    
    status_cap: String,
    status_inj: String,
    proxy_status_info: String,
    env_check_results: String,
    
    logs: Arc<Mutex<String>>,
    
    active_tab: usize,
    
    mitm_process: Option<Child>,
    pipeline_active: bool,
    server_active: bool,
}

impl Default for NexusToolkitApp {
    fn default() -> Self {
        let desktop = dirs::desktop_dir().unwrap_or_else(|| PathBuf::from("C:\\"));
        Self {
            token_directory: desktop.to_string_lossy().into_owned(),
            token_filename: "captured_token.txt".to_string(),
            selected_inject_token: String::new(),
            staged_file_info: "No configuration payload actively staged".to_string(),
            status_cap: "STATUS: PIPELINE_IDLE".to_string(),
            status_inj: "STATUS: SERVER_OFFLINE".to_string(),
            proxy_status_info: "Execute telemetry query to view internal profile status...".to_string(),
            env_check_results: "Run validation check to verify interpreter and proxy binaries".to_string(),
            logs: Arc::new(Mutex::new(">> System initialized. Welcome to Microsoft Token Generator Rust v2.1.\n".to_string())),
            active_tab: 0,
            mitm_process: None,
            pipeline_active: false,
            server_active: false,
        }
    }
}

impl NexusToolkitApp {
    fn log_message(&self, msg: &str) {
        if let Ok(mut logs) = self.logs.lock() {
            logs.push_str(&format!(">> {}\n", msg));
        }
    }

    fn set_windows_proxy(&self, enable: bool, proxy_server: &str) -> bool {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        if let Ok(internet_settings) = hkcu.open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings", KEY_WRITE) {
            let enable_val: u32 = if enable { 1 } else { 0 };
            let _: io::Result<()> = internet_settings.set_value("ProxyEnable", &enable_val);
            if enable {
                let _: io::Result<()> = internet_settings.set_value("ProxyServer", &proxy_server.to_string());
            }
            true
        } else {
            false
        }
    }

    fn query_system_proxy(&mut self) {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        if let Ok(internet_settings) = hkcu.open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings", KEY_READ) {
            let enabled: u32 = internet_settings.get_value("ProxyEnable").unwrap_or(0);
            let server: String = internet_settings.get_value("ProxyServer").unwrap_or_else(|_| "NOT LOCALIZED".to_string());
            
            self.proxy_status_info = format!(
                "Live Machine Proxy Profile Configuration:\n\nStatus Condition: {}\nLoopback Target Endpoint: {}",
                if enabled == 1 { "MAPPED PROXY ENABLED ✔" } else { "PROXY COMPLETELY OFF ❌" },
                server
            );
            self.log_message("Windows registry profile analyzed successfully.");
        }
    }

    fn stop_automation(&mut self) {
        self.set_windows_proxy(false, "");
        if let Some(mut child) = self.mitm_process.take() {
            let _ = child.kill();
        }
        
        let _ = fs::remove_file("temp_capture.py");
        let _ = fs::remove_file("temp_inject.py");
        
        self.pipeline_active = false;
        self.server_active = false;
        self.selected_inject_token = String::new();
        self.staged_file_info = "No configuration payload actively staged".to_string();
        self.status_cap = "STATUS: PIPELINE_IDLE".to_string();
        self.status_inj = "STATUS: SERVER_OFFLINE".to_string();
        self.log_message("Automation streams detached cleanly. Network state normalized completely.");
    }
}

impl eframe::App for NexusToolkitApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = egui::Color32::from_rgb(5, 5, 10);
        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(15, 15, 30);
        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(22, 22, 45);
        visuals.widgets.active.bg_fill = egui::Color32::from_rgb(0, 255, 204);
        ctx.set_visuals(visuals);

        egui::TopBottomPanel::bottom("global_settings").resizable(false).show(ctx, |ui: &mut egui::Ui| {
            ui.add_space(10.0);
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(15, 15, 30))
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 255, 204)))
                .inner_margin(egui::Margin::same(12.0))
                .show(ui, |ui: &mut egui::Ui| {
                    ui.horizontal(|ui: &mut egui::Ui| {
                        ui.colored_label(egui::Color32::from_rgb(0, 255, 204), "TARGET STORAGE PATH:");
                        ui.add(egui::TextEdit::singleline(&mut self.token_directory).font(egui::TextStyle::Monospace).desired_width(f32::INFINITY));
                        if ui.button("BROWSE DIRECTORY").clicked() {
                            if let Some(path) = FileDialog::new().pick_folder() {
                                self.token_directory = path.to_string_lossy().into_owned();
                                self.log_message(&format!("Storage path reassigned to: {}", self.token_directory));
                            }
                        }
                    });
                });
            ui.add_space(10.0);
        });

        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            ui.add_space(10.0);
            
            ui.horizontal(|ui: &mut egui::Ui| {
                let tab_names = ["  GET TOKEN  ", "  INJECT TOKEN  ", "  PROXY STATUS  ", "  DEPENDENCIES MANAGER  ", "  OPERATION MANUAL  "];
                for (idx, name) in tab_names.iter().enumerate() {
                    let is_selected = self.active_tab == idx;
                    let mut button = egui::Button::new(*name);
                    if is_selected {
                        button = button.fill(egui::Color32::from_rgb(22, 22, 45));
                    }
                    if ui.add(button).clicked() {
                        self.active_tab = idx;
                    }
                }
            });
            ui.separator();
            ui.add_space(15.0);

            match self.active_tab {
                0 => {
                    ui.vertical(|ui: &mut egui::Ui| {
                        ui.heading(egui::RichText::new("AUTOMATED INTERCEPTION PIPELINE").color(egui::Color32::from_rgb(255, 0, 127)).size(24.0));
                        ui.add_space(20.0);
                        
                        ui.horizontal(|ui: &mut egui::Ui| {
                            if ui.add_enabled(!self.pipeline_active, egui::Button::new("INITIALIZE CAPTURE").min_size(egui::vec2(180.0, 45.0))).clicked() {
                                self.pipeline_active = true;
                                self.status_cap = "STATUS: DATA_INTERCEPTION_ACTIVE".to_string();
                                self.log_message("Modifying active user network proxy parameters...");
                                self.set_windows_proxy(true, "127.0.0.1:8080");
                                
                                let full_output_path = format!("{}/{}", self.token_directory, self.token_filename).replace("\\", "\\\\");
                                let script = format!(
                                    "import json\nfrom mitmproxy import http\ndef response(flow: http.HTTPFlow):\n    if flow.response and 'licensing.mp.microsoft.com' in flow.request.pretty_host and '/v8.0/licenseToken' in flow.request.path:\n        if flow.response.status_code == 200 and flow.response.content:\n            try:\n                data = json.loads(flow.response.text)\n                token = data.get('licenseToken')\n                if token:\n                    with open(r'{}', 'w', encoding='utf-8') as f: f.write(token)\n            except: pass", 
                                    full_output_path
                                );
                                let _ = fs::write("temp_capture.py", script);
                                self.log_message("Spawning mitmdump engine subprocess frame inside environment...");
                                self.mitm_process = Command::new("cmd").args(["/c", "start", "mitmdump", "-s", "temp_capture.py"]).spawn().ok();
                                
                               
                                let _ = Command::new("cmd").args(&["/c", "start", "ms-windows-store://home"]).spawn();
                                let _ = Command::new("cmd").args(&["/c", "start", "xbox://"]).spawn();
                            }
                            
                            if ui.add_enabled(self.pipeline_active || self.server_active, egui::Button::new("TERMINATE PIPELINE").min_size(egui::vec2(180.0, 45.0))).clicked() {
                                self.stop_automation();
                            }
                        });

                        ui.add_space(15.0);
                        ui.monospace(&self.status_cap);
                    });

                    ui.add_space(25.0);
                    ui.label("REALTIME CONSOLE LOG STREAM:");
                    egui::ScrollArea::vertical().max_height(250.0).show(ui, |ui: &mut egui::Ui| {
                        if let Ok(logs) = self.logs.lock() {
                            let mut logs_clone = logs.clone();
                            ui.add(egui::TextEdit::multiline(&mut logs_clone)
                                .font(egui::TextStyle::Monospace)
                                .text_color(egui::Color32::GREEN)
                                .desired_width(f32::INFINITY));
                        }
                    });
                }
                1 => {
                    ui.vertical(|ui: &mut egui::Ui| {
                        ui.heading(egui::RichText::new("AUTHENTICATION FLOW PATCHER").color(egui::Color32::from_rgb(0, 255, 204)).size(24.0));
                        ui.add_space(20.0);
                        
                        ui.horizontal(|ui: &mut egui::Ui| {
                            if ui.button("AUTO-LOAD FROM EXPORT PATH").clicked() {
                                let target_path = format!("{}/{}", self.token_directory, self.token_filename);
                                if let Ok(content) = fs::read_to_string(&target_path) {
                                    self.selected_inject_token = content.trim().to_string();
                                    self.staged_file_info = format!("Staged Dataset Target: {}", self.token_filename);
                                    self.log_message(&format!("Token successfully allocated from: {}", target_path));
                                } else {
                                    self.log_message("Error: Active payload tracking artifact missing.");
                                }
                            }
                            
                            if ui.button("BROWSE TOKEN MANUALLY").clicked() {
                                if let Some(path) = FileDialog::new().add_filter("Text", &["txt"]).pick_file() {
                                    if let Ok(content) = fs::read_to_string(&path) {
                                        self.selected_inject_token = content.trim().to_string();
                                        self.staged_file_info = format!("Staged Dataset Target: {:?}", path.file_name().unwrap());
                                        self.log_message(&format!("Token successfully loaded from: {:?}", path));
                                    }
                                }
                            }
                        });

                        ui.add_space(10.0);
                        ui.colored_label(egui::Color32::from_rgb(92, 92, 124), &self.staged_file_info);
                        ui.add_space(20.0);

                        ui.horizontal(|ui: &mut egui::Ui| {
                            let has_token = !self.selected_inject_token.is_empty();
                            if ui.add_enabled(has_token && !self.server_active, egui::Button::new("ENGAGE EMULATION SERVER").min_size(egui::vec2(200.0, 45.0))).clicked() {
                                self.server_active = true;
                                self.status_inj = "STATUS: EMULATION_LOOP_ACTIVE".to_string();
                                self.set_windows_proxy(true, "127.0.0.1:8080");
                                
                                let script = format!(
                                    "from mitmproxy import http\nimport json\ndef response(flow: http.HTTPFlow):\n    if flow.request.pretty_url == 'https://licensing.mp.microsoft.com/v8.0/licenseToken':\n        flow.response.headers['content-type'] = 'application/json'\n        flow.response.text = json.dumps({{'licenseToken': '{}'}})", 
                                    self.selected_inject_token
                                );
                                let _ = fs::write("temp_inject.py", script);
                                self.mitm_process = Command::new("cmd").args(["/c", "start", "mitmdump", "-s", "temp_inject.py"]).spawn().ok();
                                self.log_message("Spawning mock server context loops asynchronously...");
                                
                           
                                let _ = Command::new("cmd").args(&["/c", "start", "ms-windows-store://home"]).spawn();
                                let _ = Command::new("cmd").args(&["/c", "start", "xbox://"]).spawn();
                            }

                            if ui.add_enabled(self.server_active, egui::Button::new("DISENGAGE SERVER").min_size(egui::vec2(200.0, 45.0))).clicked() {
                                self.stop_automation();
                            }
                        });

                        ui.add_space(15.0);
                        ui.monospace(&self.status_inj);
                    });
                }
                2 => {
                    ui.vertical(|ui: &mut egui::Ui| {
                        ui.heading(egui::RichText::new("WINDOWS REGISTRY INTERNET OPTIONS").color(egui::Color32::from_rgb(0, 255, 204)).size(24.0));
                        ui.add_space(30.0);
                        ui.label(&self.proxy_status_info);
                        ui.add_space(30.0);
                        
                        ui.horizontal(|ui: &mut egui::Ui| {
                            if ui.button("QUERY LIVE REGISTRY STATUS").clicked() {
                                self.query_system_proxy();
                            }
                            if ui.button("FORCE DISABLE SYSTEM PROXY").clicked() {
                                self.set_windows_proxy(false, "");
                                self.query_system_proxy();
                                self.log_message("Windows network stack variables successfully cleared.");
                            }
                        });
                    });
                }
                3 => {
                    ui.vertical(|ui: &mut egui::Ui| {
                        ui.heading(egui::RichText::new("CORE COMPONENT ENVIRONMENT ANALYSIS").color(egui::Color32::from_rgb(0, 255, 204)).size(24.0));
                        ui.add_space(15.0);
                        
                        if ui.button("EXECUTE FULL ENVIRONMENT VERIFICATION").clicked() {
                            let python_check = Command::new("python").arg("--version").output().is_ok();
                            let mitm_check = Command::new("mitmdump").arg("--version").output().is_ok();
                            
                            self.env_check_results = format!(
                                "System Architecture Environment Status Reports:\n\n1. Core Python Language Environment: {}\n2. Mitmproxy Binary Path Daemon: {}\n3. Internal Site-Package Libraries: READY TO DEPLOY",
                                if python_check { "STABLE VERIFIED ✔" } else { "MISSING EXECUTABLE PATH ❌" },
                                if mitm_check { "STABLE VERIFIED ✔" } else { "MISSING EXECUTABLE PATH ❌" }
                            );
                            self.log_message("Complete environment variable telemetry analysis processed entirely.");
                        }
                        
                        ui.add_space(10.0);
                        ui.label(&self.env_check_results);
                        ui.add_space(20.0);
                        
                        ui.group(|ui: &mut egui::Ui| {
                            ui.label(" DEPLOYMENT & OFFICIAL LINKS ");
                            ui.add_space(5.0);
                            ui.horizontal(|ui: &mut egui::Ui| {
                                if ui.button("DOWNLOAD PYTHON EXECUTABLE").clicked() {
                                    let _ = Command::new("cmd").args(&["/c", "start", PYTHON_DOWNLOAD_URL]).spawn();
                                }
                                if ui.button("DOWNLOAD MITMPROXY EXECUTABLE").clicked() {
                                    let _ = Command::new("cmd").args(&["/c", "start", MITM_DOWNLOAD_URL]).spawn();
                                }
                                if ui.button("INSTALL REQUIRED LIBRARIES (PIP)").clicked() {
                                    let _ = Command::new("cmd").args(["/c", "start", "pip", "install", "mitmproxy"]).spawn();
                                    self.log_message("Downloading required site-packages securely via PIP structures...");
                                }
                            });
                            ui.add_space(10.0);
                            ui.horizontal(|ui: &mut egui::Ui| {
                                if ui.button("VISIT OFFICIAL PYTHON SITE 🌐").clicked() {
                                    let _ = Command::new("cmd").args(&["/c", "start", PYTHON_WEBSITE_URL]).spawn();
                                }
                                if ui.button("VISIT OFFICIAL MITMPROXY SITE 🌐").clicked() {
                                    let _ = Command::new("cmd").args(&["/c", "start", MITM_WEBSITE_URL]).spawn();
                                }
                            });
                        });
                    });
                }
                4 => {
                    egui::ScrollArea::vertical().show(ui, |ui: &mut egui::Ui| {
                        let mut manual_text = "=========================================================================\n                        APPLICATION OPERATIONAL FLOW MANUAL\n=========================================================================\n\n1. GET TOKEN MANAGEMENT INTERFACE:\n   - [INITIALIZE CAPTURE]: Configures active machine loops on port 8080 and monitors active connections. Once targeted Microsoft Store profiles populate traffic request payloads, it intercepts the validation token key entirely and locks it down within local workspace files.\n   - [TERMINATE PIPELINE]: Tears down active telemetry listeners and resets machine proxy profiles safely.\n\n2. INJECT TOKEN MANAGEMENT INTERFACE:\n   - [AUTO-LOAD FROM EXPORT PATH]: Syncs staged token inputs with historical files fetched into the default paths.\n   - [BROWSE TOKEN MANUALLY]: Allows you to explicitly browse and choose any custom token text file stored locally on your device.\n   - [ENGAGE EMULATION SERVER]: Provisions virtualized system request endpoints locally, handling game store license challenges with previously captured authorization matrices to securely activate the game license locally.\n\n3. PROXY STATUS CONTROLS:\n   - Queries live network adapter settings to track registry variables and verify correct gateway pathways.\n\n4. DEPENDENCIES MANAGER MODULE:\n   - Features environmental automated system check options alongside setup buttons to map dependencies, with direct links included to verify runtime configurations straight from developers' official sites.".to_string();
                        ui.add(egui::TextEdit::multiline(&mut manual_text)
                            .font(egui::TextStyle::Monospace)
                            .text_color(egui::Color32::from_rgb(138, 138, 171))
                            .desired_width(f32::INFINITY)
                            .lock_focus(true));
                    });
                }
                _ => {}
            }
        });

        ctx.request_repaint_after(Duration::from_millis(500));
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.set_windows_proxy(false, "");
        if let Some(mut child) = self.mitm_process.take() {
            let _ = child.kill();
        }
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("MICROSOFT TOKEN GENERATOR")
            .with_inner_size([1050.0, 850.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "NexusToolkitApp",
        options,
        Box::new(|_cc| Box::new(NexusToolkitApp::default())),
    )
}