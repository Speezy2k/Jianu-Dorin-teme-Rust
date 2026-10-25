#![allow(dead_code)]
struct Note {
    title: String,
    body: String,
}

// ---------- Part 1: borrow to read (&str) ----------

// How many times does `c` appear in `s`?
fn count_char(s: &str, c: char) -> usize {
    let mut cnt = 0;
    for ch in s.chars() {
        if ch == c {
            cnt += 1;
        }
    }
    cnt
}

// The first word of `s` (up to the first space, or all of `s`), as a slice of `s`.
// No copying: return a &str that points into `s`. Use as_bytes and a while loop.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b' ' {
            return &s[0..i];
        }
        i += 1;
    }

    s
}

// A new String with the characters of `s` in reverse order.
fn reverse(s: &str) -> String {
    let mut s2 = String::new();
    for ch in s.chars().rev() {
        s2.push(ch);
    }
    s2
}

// True if `s` reads the same backwards. Reuse reverse().
fn is_palindrome(s: &str) -> bool {
    let s2 = reverse(s);
    s == s2
}

// A new String made of `s` repeated n times.
fn repeat_str(s: &str, n: usize) -> String {
    let mut s2 = String::new();
    for _i in 0..n {
        s2 += s;
    }
    s2
}

// ---------- Part 2: mutable borrow (&mut String) ----------

// Append "[tag]" to the caller's string.
fn add_tag(s: &mut String, tag: &str) {
    s.push('[');
    s.push_str(tag);
    s.push(']');
}

// Replace every occurrence of `c` with '*' in the caller's string.
// Build a new String, then assign it back with *s = ...
fn censor(s: &mut String, c: char) {
    let mut new_s = String::with_capacity(s.len());

    for ch in s.chars() {
        if ch == c {
            new_s.push('*');
        } else {
            new_s.push(ch);
        }
    }

    *s = new_s;
}

// ---------- Part 3: ownership in, ownership out (String) ----------

// Take a string, make it uppercase, and give it back.
fn shout(s: String) -> String {
    s.to_uppercase()
}

// Take two strings and return the longer one (the first on a tie). The other is dropped.
fn longer_of(a: String, b: String) -> String {
    if a.len() >= b.len() { a } else { b }
}

// ---------- Part 4: Notebook (a struct that owns its strings) ----------

// Build a Note from two string slices. The Note must own its text.
fn make_note(title: &str, body: &str) -> Note {
    Note {
        title: title.to_string(),
        body: body.to_string(),
    }
}

// Number of words in the body, counted by hand with chars().
fn word_count(note: &Note) -> usize {
    let mut count = 0;
    let mut in_word = false;

    for ch in note.body.chars() {
        if ch.is_whitespace() {
            in_word = false;
        } else if !in_word {
            in_word = true;
            count += 1;
        }
    }

    count
}

// Add `text` to the end of the body, separated by one space
// (no leading space if the body is empty).
fn append(note: &mut Note, text: &str) {
    if note.body.is_empty() {
        note.body.push_str(text);
    } else {
        note.body.push(' ');
        note.body.push_str(text);
    }
}

// Use the note up and return "[ARCHIVED] <title>: <body>".
fn archive(note: Note) -> String {
    format!("[ARCHIVED] {}: {}", note.title, note.body)
}

// Combine two notes. The first is consumed and reused, the second is only read.
// Title becomes "<a>+<b>", body becomes "<a body> <b body>".
fn merge(mut a: Note, b: &Note) -> Note {
    a.title.push('+');
    a.title.push_str(&b.title);

    append(&mut a, &b.body);

    a
}

fn main() {}
