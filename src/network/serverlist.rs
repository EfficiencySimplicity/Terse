use crossterm::event::KeyCode;

use crate::tui::Selectable;
use crate::network::Server;
use crate::tui::Window;

pub type ServerList = Selectable<Server>;

// Literally window and widget can both be implemented already...