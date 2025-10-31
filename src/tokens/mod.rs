use rand::rng;
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Deserialize)]
pub struct Token {
    pub(crate) name: String,
    pub(crate) token: String,
}

#[derive(Debug)]
struct TokenInner {
    name: String,
    token: String,
    counter: u32,
}

impl Ord for TokenInner {
    fn cmp(&self, other: &Self) -> Ordering {
        other.counter.cmp(&self.counter)
    }
}

impl PartialOrd for TokenInner {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for TokenInner {}

impl PartialEq for TokenInner {
    fn eq(&self, other: &Self) -> bool {
        self.counter == other.counter
    }
}
#[derive(Debug)]
pub struct TokensBalencer(BinaryHeap<TokenInner>);

impl Iterator for TokensBalencer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut head = self.0.pop()?;
        head.counter += 1;
        let token: Token = Token {
            name: head.name.clone(),
            token: head.token.clone(),
        };
        self.0.push(head);
        Some(token)
    }
}

impl From<Vec<Token>> for TokensBalencer {
    fn from(mut tokens: Vec<Token>) -> Self {
        tokens.shuffle(&mut rng());
        let tokens = tokens.into_iter().map(|token| TokenInner {
            counter: 0,
            token: token.token,
            name: token.name,
        });
        Self(BinaryHeap::from_iter(tokens))
    }
}
