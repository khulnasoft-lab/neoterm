use crate::state::{Block, BlockStatus};
use crate::pty::{self, PtyWriter, PtyReader};
use iced::widget::{column, container, scrollable, text, text_input};
use iced::{executor, Application, Command, Element, Length, Subscription, Theme};

pub struct TerminalApp {
    blocks: Vec<Block>,
    current_input: String,
    pty_writer: PtyWriter,
    pty_reader: PtyReader,
}

#[derive(Debug, Clone)]
pub enum Message {
    // User-triggered events
    InputChanged(String),
    Submit,
    // Events from our PTY subscription
    PtyOutputReceived(Vec<u8>),
}

impl Application for TerminalApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let (pty_writer, pty_reader) = pty::spawn_shell();

        let app = Self {
            blocks: vec![],
            current_input: String::new(),
            pty_writer,
            pty_reader,
        };

        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("NeoTerm - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::InputChanged(input) => {
                self.current_input = input;
            }
            Message::Submit => {
                if !self.current_input.is_empty() {
                    // Add a newline, as if we pressed Enter in a real terminal
                    let command_with_newline = format!("{}\n", self.current_input);

                    // Write to the PTY
                    self.pty_writer.write_all(command_with_newline.as_bytes()).unwrap();

                    // Create a new block and add it to our state
                    let new_block = Block::new(self.current_input.clone());
                    self.blocks.push(new_block);

                    // Clear the input field
                    self.current_input.clear();
                }
            }
            Message::PtyOutputReceived(output) => {
                if let Some(last_block) = self.blocks.last_mut() {
                    // For now, just append all output to the last running block
                    if last_block.status == BlockStatus::Running {
                        let output_str = String::from_utf8_lossy(&output);
                        last_block.output.push_str(&output_str);

                        // A very simple heuristic to "finish" a block: if the output
                        // contains the shell prompt (e.g., '$ '). A real implementation
                        // is MUCH more complex (e.g., using shell integration scripts).
                        if output_str.contains("$ ") {
                            last_block.status = BlockStatus::Finished;
                        }
                    }
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let mut content = column![].spacing(10);

        // Render all the blocks
        for block in &self.blocks {
            let command_prompt = text(format!("$ {}", block.command)).size(16);
            // In a real app, you would parse ANSI codes here
            let output = text(&block.output).size(14);
            let block_view = column![command_prompt, output].spacing(5);
            content = content.push(
                container(block_view)
                    .width(Length::Fill)
                    .padding(10)
                    .style(iced::theme::Container::Box),
            );
        }

        // Render the input field
        let input = text_input("Enter command...", &self.current_input)
            .on_input(Message::InputChanged)
            .on_submit(Message::Submit)
            .padding(10);

        content = content.push(input);

        container(scrollable(content))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        // This is how `iced` listens to external events, like our PTY output.
        // We use `unfold` to create a stream of messages from our `PtyReader`.
        iced::subscription::unfold(
            "pty_reader",
            self.pty_reader.try_clone().expect("Failed to clone pty reader"),
            |mut reader| async move {
                match reader.recv() {
                    Ok(output) => (Some(Message::PtyOutputReceived(output)), reader),
                    Err(_) => (None, reader), // End the subscription on error
                }
            },
        )
    }
} 