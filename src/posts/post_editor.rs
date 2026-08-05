use ratatui::prelude::Text;

pub struct DraftHeader {
	pub title: String,
}

impl From<DraftHeader> for Text<'_> {
	fn from(value: DraftHeader) -> Self {
		Text::from(value.title).centered()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use ratatui::prelude::{Buffer, Widget, Rect};

	#[test]
	fn post_header_renders_right() {
		let header = DraftHeader {title: String::from("ABC DEF GHI")};
		let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 1));

		Text::from(header).render(buffer.area, &mut buffer);
		// Taking inspiration from how Ratatui tests its List widget
		assert_eq!(
			buffer,
			Buffer::with_lines(vec![
				"  ABC DEF GHI  "
			])
		);
	}
}