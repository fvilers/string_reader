pub struct StringReader {
    chars: Vec<char>,
    position: usize,
}

impl StringReader {
    pub fn new<S: Into<String>>(s: S) -> Self {
        Self {
            chars: s.into().chars().collect(),
            position: 0,
        }
    }

    pub fn peek(&self) -> Option<&char> {
        self.chars.get(self.position)
    }

    pub fn read(&mut self) -> Option<&char> {
        self.position += 1;
        self.chars.get(self.position - 1)
    }

    pub fn read_line(&mut self) -> Option<String> {
        let initial_position = self.position;

        while let Some(c) = self.read() {
            if *c == '\n' {
                break;
            }
        }

        self.chars
            .get(initial_position..self.position - 1)
            .map(|c| c.iter().collect())
    }

    pub fn read_to_end(&mut self) -> Option<String> {
        let index = self.position..;

        self.position = self.chars.len() + 1;
        self.chars.get(index).map(|c| c.iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peek_should_return_none_for_empty_string() {
        let reader = StringReader::new(String::from(""));

        assert_eq!(reader.peek(), None);
    }

    #[test]
    fn peek_should_return_the_current_character() {
        let mut reader = StringReader::new(String::from("hello"));
        reader.read();

        assert_eq!(reader.peek(), Some(&'e'));
    }

    #[test]
    fn read_should_return_none_for_empty_string() {
        let mut reader = StringReader::new(String::from(""));

        assert_eq!(reader.read(), None);
    }

    #[test]
    fn read_should_return_the_characters_in_order_then_none() {
        let mut reader = StringReader::new(String::from("hello"));

        assert_eq!(reader.read(), Some(&'h'));
        assert_eq!(reader.read(), Some(&'e'));
        assert_eq!(reader.read(), Some(&'l'));
        assert_eq!(reader.read(), Some(&'l'));
        assert_eq!(reader.read(), Some(&'o'));
        assert_eq!(reader.read(), None);
    }

    #[test]
    fn read_line_should_return_an_empty_string_for_empty_string() {
        let mut reader = StringReader::new(String::from(""));

        assert_eq!(reader.read_line(), Some(String::from("")));
    }

    #[test]
    fn read_line_should_return_the_whole_string_if_no_new_line() {
        let hello = String::from("hello");
        let mut reader = StringReader::new(&hello);

        assert_eq!(reader.read_line(), Some(hello));
        assert_eq!(reader.read_line(), None);
    }

    #[test]
    fn read_line_should_return_the_string_before_the_new_line() {
        let mut reader = StringReader::new(String::from("hello\nworld!"));
        reader.read();

        // cSpell:disable-next-line
        assert_eq!(reader.read_line(), Some(String::from("ello")));
        assert_eq!(reader.read_line(), Some(String::from("world!")));
    }

    #[test]
    fn read_to_end_should_return_an_empty_string_for_empty_string() {
        let mut reader = StringReader::new(String::from(""));

        assert_eq!(reader.read_to_end(), Some(String::from("")));
        assert_eq!(reader.read_to_end(), None);
    }

    #[test]
    fn read_to_end_should_return_the_whole_string() {
        let hello = String::from("hello");
        let mut reader = StringReader::new(&hello);

        assert_eq!(reader.read_to_end(), Some(hello));
    }

    #[test]
    fn read_to_end_should_return_the_remaining_string() {
        let mut reader = StringReader::new(String::from("hello"));
        reader.read();

        // cSpell:disable-next-line
        assert_eq!(reader.read_to_end(), Some(String::from("ello")));
    }
}
