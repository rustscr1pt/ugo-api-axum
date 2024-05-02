use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RemoveNoteBody {
    pub note_id : String,
    pub related_id : String
}