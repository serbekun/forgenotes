//! note file DTO (main kind of file)
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::core::model::types::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Test {
    pub id: Uuid,
    pub title: String,
    pub note_uuids: Vec<Uuid>,
    pub dictionary_uuids: Vec<Uuid>,
    pub created_at: String,
    pub problems: Vec<Problem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "problem_type")]
pub enum Problem {
    #[serde(rename = "multiple_choice")]
    MultipleChoice {
        question: String,
        answers_variants: Vec<String>,
        answers: Vec<String>,
        need_to_select_some_answer: bool,
        hint: Option<String>,
    },

    #[serde(rename = "open_ended")]
    OpenEnded {
        question: String,
        answer: String,
        hint: Option<String>,
    },

    #[serde(rename = "exact_answer")]
    ExactAnswer {
        question: String,
        answer: String,
        hint: Option<String>,
    },
}

impl HasId for Test {
    fn id(&self) -> Uuid { self.id }
    fn entity_type() -> Types { Types::Test }
}