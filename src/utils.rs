use ratatui::widgets::ListState;

pub struct Selectable<T> {
    items: Vec<T>,
    pub state: ListState,
}

impl<T> Selectable<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {items, state: ListState::default().with_selected(Some(0))}
    }

    pub fn selected_item(&self) -> Option<&T> {
        self.items.get(self.state.selected()?)
    }
}