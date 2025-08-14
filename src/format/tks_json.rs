use crate::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TksFile {
    #[allow(dead_code)]
    version: String,
    tracks: Vec<TksTrack>,
}
#[derive(Debug, Deserialize)]
struct TksTrack {
    #[allow(dead_code)]
    id: String,
    sequence: Vec<TksEvent>,
}
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum TksEvent {
    #[serde(rename = "Insert")]
    Insert { text: String, #[allow(dead_code)] t: Option<u64> },

    #[serde(rename = "Delete")]
    Delete { #[serde(default = "one")] n: u32, #[allow(dead_code)] t: Option<u64> },

    #[serde(rename = "Marker")]
    Marker { name: String, #[allow(dead_code)] t: Option<u64> },
}
fn one() -> u32 { 1 }

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Char(char),
    Backspace,
}

#[derive(Debug, Clone)]
pub struct Sequence {
    pub actions: Vec<Action>,
    pub markers: Vec<(usize, String)>,
}

impl Sequence {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let f: TksFile = serde_json::from_slice(bytes)?;
        let mut actions: Vec<Action> = Vec::new();
        let mut markers: Vec<(usize, String)> = Vec::new();

        if let Some(track) = f.tracks.get(0) {
            for ev in &track.sequence {
                match ev {
                    TksEvent::Insert { text, .. } => {
                        for ch in text.chars() {
                            actions.push(Action::Char(ch));
                        }
                    }
                    TksEvent::Delete { n, .. } => {
                        for _ in 0..*n {
                            actions.push(Action::Backspace);
                        }
                    }
                    TksEvent::Marker { name, .. } => {
                        markers.push((actions.len(), name.clone()));
                    }
                }
            }
        }
        Ok(Self { actions, markers })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn flatten_with_delete() {
        let json = br#"{
            "version":"1",
            "tracks":[{"id":"main","sequence":[
                {"t":0,"type":"Insert","text":"ab"},
                {"t":1,"type":"Delete"},
                {"t":2,"type":"Insert","text":"c"},
                {"t":3,"type":"Marker","name":"m"}
            ]}]
        }"#;
        let seq = Sequence::from_bytes(json).unwrap();
        assert_eq!(seq.actions.len(), 4);
        matches!(seq.actions[0], Action::Char('a'));
        matches!(seq.actions[1], Action::Char('b'));
        matches!(seq.actions[2], Action::Backspace);
        matches!(seq.actions[3], Action::Char('c'));
        assert_eq!(seq.markers[0].0, 4);
    }
}
