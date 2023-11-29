pub fn substring_at_last(text: String, character: char) -> String {
    let text_chars = text.chars();
    let index_of_character = text.find(character).unwrap_or(0);

    return text_chars.skip(index_of_character + 1).collect();
}