use iced::{executor, Application, Command, Element, Settings, Theme};
use iced::widget::{column, container, scrollable, text_input, button, row};
use std::path::PathBuf;
use tokio::sync::mpsc;
use uuid::Uuid;

mod block;
mod shell;
mod input;
mod renderer;

use block::{Block, BlockContent};
use shell::ShellManager;
use input::EnhancedTextInput;

#[derive(Debug, Clone)]
pub struct NeoTerm {
    blocks: Vec<Block>,
    current_input: String,
    input_history: Vec<String>,
    history_index: Option<usize>,
    shell_manager: ShellManager,
    input_state: text_input::State,
    suggestions: Vec<String>,
    active_suggestion: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    ExecuteCommand,
    CommandOutput(String, i32), // output, exit_code
    KeyPressed(iced::keyboard::Key),
    HistoryUp,
    HistoryDown,
    SuggestionSelected(usize),
    BlockAction(Uuid, BlockMessage),
    Tick,
}

#[derive(Debug, Clone)]
pub enum BlockMessage {
    Copy,
    Rerun,
    Delete,
    Export,
}

impl Application for NeoTerm {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let shell_manager = ShellManager::new();
        
        (
            Self {
                blocks: Vec::new(),
                current_input: String::new(),
                input_history: Vec::new(),
                history_index: None,
                shell_manager,
                input_state: text_input::State::new(),
                suggestions: Vec::new(),
                active_suggestion: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "NeoTerm".to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::InputChanged(input) => {
                self.current_input = input.clone();
                self.suggestions = self.generate_suggestions(&input);
                Command::none()
            }
            Message::ExecuteCommand => {
                if !self.current_input.trim().is_empty() {
                    let command = self.current_input.clone();
                    self.input_history.push(command.clone());
                    self.history_index = None;
                    
                    // Create new command block
                    let block = Block::new_command(command.clone());
                    self.blocks.push(block);
                    self.current_input.clear();
                    
                    // Execute command asynchronously
                    Command::perform(
                        self.shell_manager.execute_command(command),
                        |(output, exit_code)| Message::CommandOutput(output, exit_code)
                    )
                } else {
                    Command::none()
                }
            }
            Message::CommandOutput(output, exit_code) => {
                if let Some(last_block) = self.blocks.last_mut() {
                    last_block.set_output(output, exit_code);
                }
                Command::none()
            }
            Message::HistoryUp => {
                if !self.input_history.is_empty() {
                    let new_index = match self.history_index {
                        None => Some(self.input_history.len() - 1),
                        Some(i) if i > 0 => Some(i - 1),
                        Some(i) => Some(i),
                    };
                    
                    if let Some(index) = new_index {
                        self.current_input = self.input_history[index].clone();
                        self.history_index = new_index;
                    }
                }
                Command::none()
            }
            Message::HistoryDown => {
                match self.history_index {
                    Some(i) if i < self.input_history.len() - 1 => {
                        self.history_index = Some(i + 1);
                        self.current_input = self.input_history[i + 1].clone();
                    }
                    Some(_) => {
                        self.history_index = None;
                        self.current_input.clear();
                    }
                    None => {}
                }
                Command::none()
            }
            Message::BlockAction(block_id, action) => {
                self.handle_block_action(block_id, action)
            }
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        let blocks_view = scrollable(
            column(
                self.blocks
                    .iter()
                    .map(|block| block.view())
                    .collect::<Vec<_>>()
            )
            .spacing(8)
        )
        .height(iced::Length::Fill);

        let input_view = self.create_input_view();

        column![blocks_view, input_view]
            .spacing(8)
            .padding(16)
            .into()
    }
}

impl NeoTerm {
    fn generate_suggestions(&self, input: &str) -> Vec<String> {
        // Implement fuzzy matching for commands, files, etc.
        let mut suggestions = Vec::new();
        
        // Add command history matches
        for cmd in &self.input_history {
            if cmd.contains(input) && cmd != input {
                suggestions.push(cmd.clone());
            }
        }
        
        // Add common commands
        let common_commands = ["ls", "cd", "git", "npm", "cargo", "docker"];
        for cmd in &common_commands {
            if cmd.starts_with(input) && !input.is_empty() {
                suggestions.push(cmd.to_string());
            }
        }
        
        suggestions.truncate(5);
        suggestions
    }

    fn create_input_view(&self) -> Element<Message> {
        let input = text_input("Enter command...", &self.current_input)
            .on_input(Message::InputChanged)
            .on_submit(Message::ExecuteCommand)
            .padding(12)
            .size(16);

        let suggestions_view = if !self.suggestions.is_empty() {
            column(
                self.suggestions
                    .iter()
                    .enumerate()
                    .map(|(i, suggestion)| {
                        button(text(suggestion))
                            .on_press(Message::SuggestionSelected(i))
                            .width(iced::Length::Fill)
                            .into()
                    })
                    .collect::<Vec<_>>()
            )
            .spacing(2)
            .into()
        } else {
            column![].into()
        };

        column![input, suggestions_view].spacing(4).into()
    }

    fn handle_block_action(&mut self, block_id: Uuid, action: BlockMessage) -> Command<Message> {
        match action {
            BlockMessage::Rerun => {
                if let Some(block) = self.blocks.iter().find(|b| b.id == block_id) {
                    if let BlockContent::Command { input, .. } = &block.content {
                        let command = input.clone();
                        Command::perform(
                            self.shell_manager.execute_command(command),
                            |(output, exit_code)| Message::CommandOutput(output, exit_code)
                        )
                    } else {
                        Command::none()
                    }
                } else {
                    Command::none()
                }
            }
            BlockMessage::Delete => {
                self.blocks.retain(|b| b.id != block_id);
                Command::none()
            }
            _ => Command::none(),
        }
    }
}

fn main() -> iced::Result {
    NeoTerm::run(Settings::default())
}