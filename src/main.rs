use std::collections::HashMap;

#[derive(Debug)]
struct Document {
    id: usize,
    text: String,
}

#[derive(Debug)]
struct InvertedIndex {
    data: HashMap<String, Vec<usize>>,
}

impl InvertedIndex {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, docs: Vec<Document>) {
        for doc in docs {
            for token in analyze(&doc.text) {
                if let Some(ids) = self.data.get(&token as &str) {
                    if ids[ids.len() - 1] == doc.id {
                        continue;
                    }
                    let mut new_ids = ids.to_owned();
                    new_ids.push(doc.id);
                    self.data.insert(token.to_owned(), new_ids.to_vec());
                } else {
                    self.data.insert(token.to_owned(), vec![doc.id]);
                }
            }
        }
    }

    pub fn search(&self, query: &str) -> Vec<Vec<usize>>{
        let mut result = vec![];
        for token in analyze(query) {
            if let Some(data) = self.data.get(&token as &str) {
                result.push(data.to_owned())
            }
        }

        result
    }
}

fn analyze(text: &str) -> Vec<String> {
    let mut stop_words = HashMap::new();
    stop_words.insert("a", "");
    stop_words.insert("dia", "");
    let tokens = tokenize(text);
    let tokens = stopword_filter(&tokens, stop_words);
    tokens
}

fn tokenize(text: &str) -> Vec<String> {
    if text.is_empty() {
        return vec!["".to_owned()];
    }

    text.chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .map(String::from)
        .collect()
}

fn stopword_filter(tokens: &Vec<String>, stop_words: HashMap<&str, &str>) -> Vec<String> {
    let mut new_tokens = Vec::new();
    for token in tokens {
        if stop_words.get(&token as &str).is_none() {
            new_tokens.push(token.to_owned())
        }
    }
    new_tokens
}

fn main() {
    let mut index = InvertedIndex::new();
    index.insert(vec![Document {
        id: 1,
        text: "Aku sangat cinta dan sayang dia!".to_owned(),
    }]);
    index.insert(vec![Document {
        id: 2,
        text: "Dia, adalah istriku yang paling aku cinta dan sayang!".to_owned(),
    }]);

    let result = index.search("cinta");
    println!("{:?}", result);
}
