
/// Remove emoji from a text string.
///
/// ":mahjong: Anki" -> "Anki"
pub fn remove_emoji(txt: &str) -> &str {
    let re = regex!(r"^(?::[^:]*:)?\s*(.+)");
    let caps = re.captures(txt);
    match caps {
        Some(x) => x.at(1).unwrap(),
        // Should never happen
        None => panic!("Failed to match text: {}", txt),
    }
}

