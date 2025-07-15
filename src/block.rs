use iced::{Element, widget::{column, row, text, button, container}};
use std::path::PathBuf;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::{Message, BlockMessage};

#[derive(Debug, Clone)]
pub struct Block {
    pub id: Uuid,
    pub content: BlockContent,
    pub timestamp: DateTime<Utc>,
    pub exit_code: Option<i32>,
}

#[derive(Debug, Clone)]
pub enum BlockContent {
    Command { 
        input: String, 
        output: String,
        working_dir: PathBuf,
    },
    Markdown(String),
    FilePreview {
        path: PathBuf,
        content: String,
        file_type: FileType,
    },
    Error {
        message: String,
        details: Option<String>,
    },
}

#[derive(Debug, Clone)]
pub enum FileType {
    Text,
    Json,
    Yaml,
    Code(String), // language
    Image,
    Binary,
}

impl Block {
    pub fn new_command(input: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            content: BlockContent::Command {
                input,
                output: String::new(),
                working_dir: std::env::current_dir().unwrap_or_default(),
            },
            timestamp: Utc::now(),
            exit_code: None,
        }
    }

    pub fn new_markdown(content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            content: BlockContent::Markdown(content),
            timestamp: Utc::now(),
            exit_code: None,
        }
    }

    pub fn new_file_preview(path: PathBuf, content: String) -> Self {
        let file_type = Self::detect_file_type(&path);
        
        Self {
            id: Uuid::new_v4(),
            content: BlockContent::FilePreview {
                path,
                content,
                file_type,
            },
            timestamp: Utc::now(),
            exit_code: None,
        }
    }

    pub fn set_output(&mut self, output: String, exit_code: i32) {
        if let BlockContent::Command { output: ref mut out, .. } = &mut self.content {
            *out = output;
            self.exit_code = Some(exit_code);
        }
    }

    pub fn view(&self) -> Element<Message> {
        let header = self.create_header();
        let content = self.create_content();
        let actions = self.create_actions();

        container(
            column![header, content, actions]
                .spacing(8)
        )
        .padding(12)
        .style(|theme| {
            container::Appearance {
                background: Some(theme.palette().background.into()),
                border: iced::Border {
                    color: theme.palette().text,
                    width: 1.0,
                    radius: 8.0.into(),
                },
                ..Default::default()
            }
        })
        .into()
    }

    fn create_header(&self) -> Element<Message> {
        let timestamp_str = self.timestamp.format("%H:%M:%S").to_string();
        let status_indicator = match self.exit_code {
            Some(0) => "✓",
            Some(_) => "✗",
            None => "⏳",
        };

        row![
            text(status_indicator).size(16),
            text(timestamp_str).size(12),
        ]
        .spacing(8)
        .into()
    }

    fn create_content(&self) -> Element<Message> {
        match &self.content {
            BlockContent::Command { input, output, working_dir } => {
                column![
                    text(format!("$ {}", input))
                        .size(14)
                        .style(|theme| text::Appearance {
                            color: Some(theme.palette().primary),
                        }),
                    if !output.is_empty() {
                        text(output)
                            .size(12)
                            .style(|theme| text::Appearance {
                                color: Some(theme.palette().text),
                            })
                    } else {
                        text("Running...")
                            .size(12)
                            .style(|theme| text::Appearance {
                                color: Some(theme.palette().text.scale_alpha(0.7)),
                            })
                    }
                ]
                .spacing(4)
                .into()
            }
            BlockContent::Markdown(content) => {
                // Implement markdown rendering
                text(content).size(14).into()
            }
            BlockContent::FilePreview { path, content, file_type } => {
                column![
                    text(format!("📁 {}", path.display()))
                        .size(12)
                        .style(|theme| text::Appearance {
                            color: Some(theme.palette().text.scale_alpha(0.8)),
                        }),
                    text(content).size(12)
                ]
                .spacing(4)
                .into()
            }
            BlockContent::Error { message, details } => {
                let mut elements = vec![
                    text(format!("❌ {}", message))
                        .size(14)
                        .style(|theme| text::Appearance {
                            color: Some(iced::Color::from_rgb(0.8, 0.2, 0.2)),
                        })
                        .into()
                ];

                if let Some(details) = details {
                    elements.push(
                        text(details)
                            .size(12)
                            .style(|theme| text::Appearance {
                                color: Some(theme.palette().text.scale_alpha(0.7)),
                            })
                            .into()
                    );
                }

                column(elements).spacing(4).into()
            }
        }
    }

    fn create_actions(&self) -> Element<Message> {
        let mut actions = Vec::new();

        match &self.content {
            BlockContent::Command { .. } => {
                actions.push(
                    button("Rerun")
                        .on_press(Message::BlockAction(self.id, BlockMessage::Rerun))
                        .into()
                );
            }
            _ => {}
        }

        actions.push(
            button("Copy")
                .on_press(Message::BlockAction(self.id, BlockMessage::Copy))
                .into()
        );

        actions.push(
            button("Delete")
                .on_press(Message::BlockAction(self.id, BlockMessage::Delete))
                .into()
        );

        row(actions).spacing(8).into()
    }

    fn detect_file_type(path: &PathBuf) -> FileType {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("json") => FileType::Json,
            Some("yaml") | Some("yml") => FileType::Yaml,
            Some("rs") => FileType::Code("rust".to_string()),
            Some("js") | Some("ts") => FileType::Code("javascript".to_string()),
            Some("py") => FileType::Code("python".to_string()),
            Some("png") | Some("jpg") | Some("jpeg") | Some("gif") => FileType::Image,
            Some("txt") | Some("md") => FileType::Text,
            _ => FileType::Binary,
        }
    }
}