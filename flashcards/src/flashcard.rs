use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

type CardId = Uuid;
type DeckId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Card {
    card_id: CardId,
    front: String,
    back: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    deck_id: DeckId,
    title: String,
    cards: HashSet<CardId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    cards: HashSet<Card>,
    decks: Vec<Deck>,
}

#[cfg(test)]
mod flashcard_test {}
