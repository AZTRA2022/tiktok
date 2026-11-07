pub mod emotion;
use std::{collections::HashMap, ops::Deref};

use emotion::{DEFAULT_COUNT, Emotion, EmotionType, INTENSITY_RANGE};
use strum::IntoEnumIterator;

#[derive(Debug, Clone)]
#[allow(unused)]
struct Video {
    pub title: String,
    pub duration: f64,
    pub emotions: Vec<Emotion>,
}

impl Video {
    pub fn new(title: String, duration: f64) -> Self {
        Video {
            title,
            duration,
            emotions: Vec::new(),
        }
    }

    pub fn add_random_emotions(&mut self, count: usize) -> Vec<Emotion> {
        let mut generated: Vec<Emotion> = Vec::new();
        let emotions = EmotionType::iter().collect::<Vec<EmotionType>>();
        for _ in 0..count {
            let random_emotion = emotions[rand::random_range(0..emotions.len())];
            let intensity = rand::random::<f64>() % INTENSITY_RANGE.end();
            self.emotions.push(Emotion {
                emotion_type: random_emotion,
                intensity,
            });
            generated.push(Emotion {
                emotion_type: random_emotion,
                intensity,
            });
        }

        generated
    }
}

#[derive(Debug)]
struct Feed {
    pub videos: Vec<Video>,
}

impl Feed {
    fn new() -> Self {
        Feed { videos: Vec::new() }
    }

    fn inject(&mut self, perceived_emotions: Vec<Emotion>, db: &Database) {
        let total: f64 = perceived_emotions.iter().map(|e| e.intensity).sum();
        if total == 0.0 {
            return;
        }

        let perceived: HashMap<&EmotionType, f64> = perceived_emotions
            .iter()
            .map(|e| (&e.emotion_type, e.intensity / total))
            .collect();

        let mut scored: Vec<(Video, f64)> = db
            .videos
            .iter()
            .filter_map(|video| {
                let score: f64 = video
                    .emotions
                    .iter()
                    .filter(|e| perceived.contains_key(&e.emotion_type))
                    .map(|e| perceived[&e.emotion_type])
                    .sum();
                (score > 0.0).then(|| (video.clone(), score))
            })
            .collect();

        // Classement par pertinence décroissante
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let to_be_injected: Vec<Video> = scored.into_iter().map(|(v, _)| v).collect();
        dbg!(to_be_injected.len());
    }
}

struct Database {
    pub videos: Vec<Video>,
}

impl Database {
    fn new() -> Self {
        Database { videos: Vec::new() }
    }

    fn populate(&mut self) {
        for i in 0..10000 {
            let mut video = Video::new(format!("Video {}", i + 1), rand::random_range(0.0..60.0));
            video.add_random_emotions(DEFAULT_COUNT);
            self.videos.push(video);
        }
    }
}

#[derive(Debug)]
struct User {
    pub username: String,
    pub today_emotions: Vec<Emotion>,
    pub feed: Feed,
}

impl User {
    fn new(username: String) -> Self {
        User {
            username,
            today_emotions: Vec::new(),
            feed: Feed::new(),
        }
    }

    fn add_random_emotions(&mut self, count: usize) {
        let emotions = EmotionType::iter().collect::<Vec<EmotionType>>();
        for _ in 0..count {
            let random_emotion = emotions[rand::random_range(0..emotions.len())];
            let intensity = rand::random::<f64>() % INTENSITY_RANGE.end();
            self.today_emotions.push(Emotion {
                emotion_type: random_emotion,
                intensity,
            });
        }
    }
}
