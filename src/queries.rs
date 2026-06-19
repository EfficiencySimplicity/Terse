use crate::{posts::{Post, get_post}, tui::get_default_block, network::{Server, SearchResult}};
use reqwest::{Error};
use crate::tui::{App, Window, PostWidget};
use url::Url;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Modifier,
    widgets::{List, ListState, StatefulWidget, Widget},
};

use crossterm::event::KeyCode;


pub struct SearchResults {
    links: Vec<SearchResult>,
    list_state: ListState,
}

impl SearchResults {
    fn new(links: Vec<SearchResult>) -> Self {
        let mut list_state = ListState::default();
        list_state.select_first();

        Self { links, list_state }
    }

    fn get_selected_article(&self) -> Result<Post, Error> {
        // https://stackoverflow.com/questions/37890405/is-there-a-way-to-simplify-converting-an-option-into-a-result-without-a-macro
        get_post(self.links.get(self.list_state.selected().unwrap_or(0)).unwrap().postid)
    }
}

impl Widget for &mut SearchResults {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // inner is inside the block, not the whole magic scrollview buffer...

        let container = get_default_block()
            .title_bottom("( ((j / k) + enter) or (id) to select )");

        StatefulWidget::render(
            List::new(&self.links)
                // This can also be a style.
                .highlight_style(Modifier::REVERSED)
                .scroll_padding(3)
                .block(container),
            area,
            buf,
            &mut self.list_state,
        );
    }
}

impl Window for SearchResults {
    fn handle_key_event(&mut self, key: KeyCode) {
        match key {
            // TODO: clamp after!
            KeyCode::Char('j') => self.list_state.scroll_down_by(1),
            KeyCode::Char('k') => self.list_state.scroll_up_by(1),
            _ => (),
        }
    }
}

pub enum SearchMenuMode {
    Results,
    Answer(PostWidget),
}

pub struct SearchMenu {
    results: SearchResults,
    mode: SearchMenuMode,
}

impl SearchMenu {
    fn new(results: SearchResults) -> Self {
        Self {results, mode: SearchMenuMode::Results}
    }
}

impl Widget for &mut SearchMenu {
    fn render(self, area: Rect, buf: &mut Buffer) {
    
        match &mut self.mode {
            SearchMenuMode::Results => {self.results.render(area, buf)}
            SearchMenuMode::Answer(p) => {
                let container = get_default_block().title_bottom("( b to go back )");
                let inner = container.inner(area);

                container.render(area, buf);
                p.render(inner, buf);
            }
        }
    }
}

impl Window for SearchMenu {
    fn handle_key_event(&mut self, key: KeyCode) {
        match &mut self.mode {
            SearchMenuMode::Results => {
                match key {
                    KeyCode::Enter => {
                        self.mode = SearchMenuMode::Answer(
                            PostWidget::new(self.results.get_selected_article().unwrap())
                        )
                    }
                    _ => self.results.handle_key_event(key)
                }
            }
            SearchMenuMode::Answer(a) => {
                match key {
                    KeyCode::Char('b') => {
                        self.mode = SearchMenuMode::Results;
                    }
                    _ => a.handle_key_event(key),
                }
            }
        }
    }
}

// NOTE: Query?...
pub fn process_query(query: Vec<String>) {
    let server = Server::new(Url::parse("https://localhost:3000").unwrap());
    let results = server.search(query);

    match results {
        Ok(r) => {_ = App::default().run(&mut SearchMenu::new(SearchResults::new(r)))}
        _ => {}
    }
}