use crossterm::{
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
};
use std::io;

/// A node in the tree. It contains the name of the file or directory, the depth of the node in the
/// tree, and a flag indicating whether the node is a directory or not. The `print` method is used
/// to print the node to the terminal.
///
/// * `name`:
/// * `depth`:
/// * `is_dir`:
pub struct Node {
    components: Vec<String>,
    is_dir: bool,
}

impl Node {
    /// Create a new node with the given name, depth, and directory flag.
    ///
    /// * `name`:
    /// * `depth`:
    /// * `is_dir`:
    pub fn new(components: Vec<String>, is_dir: bool) -> Node {
        Node { components, is_dir }
    }

    /// Print the node to the terminal. If `with_style` is `true`, the node will be printed with
    /// colors and styles. Otherwise, the node will be printed without any colors or styles.
    ///
    /// * `with_style`:
    pub fn print(&self, flat: bool, color: bool) -> Result<(), io::Error> {
        // Skip directories if flat is enabled
        if flat && self.is_dir {
            return Ok(());
        }

        // Calculate the depth of the node
        let depth = self.components.len() - 1;

        // Print indentation
        if !flat {
            execute!(io::stdout(), Print(" ".repeat(depth)))?;
        }

        for (index, component) in self.components.iter().enumerate() {
            if !flat {
                if index < depth {
                    continue;
                }

                if self.is_dir {
                    execute!(
                        io::stdout(),
                        Print(match depth {
                            0 => "◆",
                            1 => "◈",
                            2 => "⟐",
                            _ => "⟡",
                        }),
                        Print(" "),
                    )?;
                } else {
                    execute!(io::stdout(), Print(" "))?;
                }
            }

            if color && (self.is_dir || index < depth) {
                execute!(
                    io::stdout(),
                    SetAttribute(Attribute::Bold),
                    SetForegroundColor(match index {
                        0 => Color::Green,
                        1 => Color::Cyan,
                        _ => Color::Blue,
                    }),
                )?;
            }

            execute!(
                io::stdout(),
                Print(component),
                ResetColor,
                SetAttribute(Attribute::Reset),
            )?;

            if flat && index < depth {
                execute!(io::stdout(), Print("/"))?;
            }
        }

        execute!(io::stdout(), Print("\n"))
    }
}
