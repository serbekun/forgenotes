//! test file DTO
use crate::core::model::types::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Test {
    pub id: Uuid,
    pub title: String,
    pub problems: Vec<Problem>,
    pub metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestDraft {
    pub title: String,
    pub problems: Vec<Problem>,
    pub metadata: Metadata,
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
    fn id(&self) -> Uuid {
        self.id
    }
    fn entity_type() -> Types {
        Types::Test
    }
    fn metadata(&self) -> &Metadata {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut Metadata {
        &mut self.metadata
    }
}

impl FromDraft for Test {
    type Draft = TestDraft;

    fn from_draft(draft: Self::Draft, id: Uuid) -> Self {
        Self {
            id,
            title: draft.title,
            problems: draft.problems,
            metadata: draft.metadata,
        }
    }
}
