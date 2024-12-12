use lazy_static::lazy_static;
use serde_json::json;

lazy_static! {
    pub static ref SCHEDULE_1: serde_json::Value = json!({
        "Friday": {
            "date": "2024-12-20",
            "day": "Friday",
            "tasks": [
                {"endTime": "19:00:00", "name": "Practice language skills", "startTime": "18:00:00"},
                {"endTime": "21:00:00", "name": "Take a fencing lesson", "startTime": "20:00:00"},
                {"endTime": "22:00:00", "name": "Memorize important dates/events", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Complete a 100% game achievement or trophy list", "startTime": "22:00:00"}
            ]
        },
        "Monday": {
            "date": "2024-12-16",
            "day": "Monday",
            "tasks": [
                {"endTime": "19:00:00", "name": "Analyze the design of a game you love", "startTime": "18:00:00"},
                {"endTime": "21:00:00", "name": "Create a study schedule", "startTime": "20:00:00"},
                {"endTime": "22:00:00", "name": "Read a book with a protagonist of a different ethnicity", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Watch educational videos", "startTime": "22:00:00"}
            ]
        },
        "Saturday": {
            "date": "2024-12-14",
            "day": "Saturday",
            "tasks": [
                {"endTime": "19:00:00", "name": "Create a playlist of your favorite video game soundtracks", "startTime": "18:00:00"},
                {"endTime": "21:00:00", "name": "Review study guides", "startTime": "20:00:00"},
                {"endTime": "22:00:00", "name": "Attend a poetry slam night", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Participate in a dance competition", "startTime": "22:00:00"}
            ]
        },
        "Sunday": {
            "date": "2024-12-15",
            "day": "Sunday",
            "tasks": [
                {"endTime": "19:00:00", "name": "Join a gaming club or society", "startTime": "18:00:00"},
                {"endTime": "21:00:00", "name": "Read a book about a different culture", "startTime": "20:00:00"},
                {"endTime": "22:00:00", "name": "Build a gaming PC or console setup", "startTime": "21:00:00"},
                {"endTime": "00:00:00", "name": "Watch a midnight movie screening", "startTime": "23:00:00"}
            ]
        },
        "Thursday": {
            "date": "2024-12-19",
            "day": "Thursday",
            "tasks": [
                {"endTime": "20:00:00", "name": "Read a poetry collection", "startTime": "19:00:00"},
                {"endTime": "22:00:00", "name": "Attend a gaming convention or expo", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Start a gaming blog or vlog", "startTime": "22:00:00"}
            ]
        },
        "Tuesday": {
            "date": "2024-12-17",
            "day": "Tuesday",
            "tasks": [
                {"endTime": "19:00:00", "name": "Take a gymnastics class", "startTime": "18:00:00"},
                {"endTime": "21:00:00", "name": "Explore a book on philosophy", "startTime": "20:00:00"},
                {"endTime": "22:00:00", "name": "Attend a paint and sip night", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Formulate questions for discussion", "startTime": "22:00:00"}
            ]
        },
        "Wednesday": {
            "date": "2024-12-18",
            "day": "Wednesday",
            "tasks": [
                {"endTime": "19:00:00", "name": "Join a pilates class", "startTime": "18:00:00"},
                {"endTime": "21:00:00", "name": "Create fan art for your favorite game", "startTime": "20:00:00"},
                {"endTime": "23:00:00", "name": "Develop critical thinking skills", "startTime": "22:00:00"},
                {"endTime": "20:00:00", "name": "Take a gymnastics class", "startTime": "19:00:00"}
            ]
        }
    });
    pub static ref SCHEDULE_2: serde_json::Value = json!({
        "Friday": {
            "date": "2024-12-20",
            "day": "Friday",
            "tasks": [
                {"endTime": "20:00:00", "name": "Create a playlist of your favorite video game soundtracks", "startTime": "19:00:00"},
                {"endTime": "22:00:00", "name": "Explore fantasy literature", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Use mnemonic devices for memorization", "startTime": "22:00:00"},
                {"endTime": "21:00:00", "name": "Take a dance class", "startTime": "20:00:00"}
            ]
        },
        "Monday": {
            "date": "2024-12-16",
            "day": "Monday",
            "tasks": [
                {"endTime": "20:00:00", "name": "Read a book that was adapted into a movie", "startTime": "19:00:00"},
                {"endTime": "22:00:00", "name": "Join online study forums", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Practice language skills", "startTime": "22:00:00"},
                {"endTime": "21:00:00", "name": "Create a gaming-themed cosplay", "startTime": "20:00:00"}
            ]
        },
        "Saturday": {
            "date": "2024-12-14",
            "day": "Saturday",
            "tasks": [
                {"endTime": "20:00:00", "name": "Review course syllabus", "startTime": "19:00:00"},
                {"endTime": "22:00:00", "name": "Attend study group sessions", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Read a poetry collection", "startTime": "22:00:00"},
                {"endTime": "21:00:00", "name": "Review a game and share your thoughts online", "startTime": "20:00:00"}
            ]
        },
        "Sunday": {
            "date": "2024-12-15",
            "day": "Sunday",
            "tasks": [
                {"endTime": "20:00:00", "name": "Visit a planetarium star show", "startTime": "19:00:00"},
                {"endTime": "22:00:00", "name": "Explore a game modding community", "startTime": "21:00:00"},
                {"endTime": "21:00:00", "name": "Attend a drag show event", "startTime": "20:00:00"},
                {"endTime": "23:00:00", "name": "Practice and improve at an online competitive game", "startTime": "22:00:00"}
            ]
        },
        "Thursday": {
            "date": "2024-12-19",
            "day": "Thursday",
            "tasks": [
                {"endTime": "20:00:00", "name": "Join a hiking club", "startTime": "19:00:00"},
                {"endTime": "22:00:00", "name": "Explore a book on psychology", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Participate in a game jam or hackathon", "startTime": "22:00:00"}
            ]
        },
        "Tuesday": {
            "date": "2024-12-17",
            "day": "Tuesday",
            "tasks": [
                {"endTime": "20:00:00", "name": "Create mind maps", "startTime": "19:00:00"},
                {"endTime": "22:00:00", "name": "Join a local baseball team", "startTime": "21:00:00"},
                {"endTime": "21:00:00", "name": "Analyze case studies", "startTime": "20:00:00"},
                {"endTime": "23:00:00", "name": "Host a game night with themed snacks and drinks", "startTime": "22:00:00"}
            ]
        },
        "Wednesday": {
            "date": "2024-12-18",
            "day": "Wednesday",
            "tasks": [
                {"endTime": "20:00:00", "name": "Go ice skating", "startTime": "19:00:00"},
                {"endTime": "22:00:00", "name": "Create a study schedule", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Read a children's classic", "startTime": "22:00:00"}
            ]
        }
    });
    pub static ref SCHEDULE_3: serde_json::Value = json!({
        "Friday": {
            "date": "2024-12-20",
            "day": "Friday",
            "tasks": [
                {"endTime": "20:00:00", "name": "Join a local basketball league", "startTime": "19:00:00"},
                {"endTime": "22:00:00", "name": "Join a night-time yoga session", "startTime": "21:00:00"},
                {"endTime": "21:00:00", "name": "Experience a themed party night", "startTime": "20:00:00"},
                {"endTime": "23:00:00", "name": "Watch a midnight movie screening", "startTime": "22:00:00"}
            ]
        },
        "Monday": {
            "date": "2024-12-16",
            "day": "Monday",
            "tasks": [
                {"endTime": "20:00:00", "name": "Read a bestseller", "startTime": "19:00:00"},
                {"endTime": "22:00:00", "name": "Try archery", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Try night-time zip-lining", "startTime": "22:00:00"},
                {"endTime": "21:00:00", "name": "Watch educational videos", "startTime": "20:00:00"}
            ]
        },
        "Saturday": {
            "date": "2024-12-14",
            "day": "Saturday",
            "tasks": [
                {"endTime": "20:00:00", "name": "Read a book from the library", "startTime": "19:00:00"},
                {"endTime": "22:00:00", "name": "Join a game development or design course", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Start a gaming blog or vlog", "startTime": "22:00:00"}
            ]
        },
        "Sunday": {
            "date": "2024-12-15",
            "day": "Sunday",
            "tasks": [
                {"endTime": "20:00:00", "name": "Practice shooting hoops", "startTime": "19:00:00"},
                {"endTime": "22:00:00", "name": "Read a book from the library", "startTime": "21:00:00"},
                {"endTime": "21:00:00", "name": "Participate in a dance competition", "startTime": "20:00:00"},
                {"endTime": "23:00:00", "name": "Try surfing", "startTime": "22:00:00"}
            ]
        },
        "Thursday": {
            "date": "2024-12-19",
            "day": "Thursday",
            "tasks": [
                {"endTime": "20:00:00", "name": "Read a book recommended by a friend", "startTime": "19:00:00"},
                {"endTime": "22:00:00", "name": "Explore environmental literature", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Discuss topics with classmates", "startTime": "22:00:00"}
            ]
        },
        "Tuesday": {
            "date": "2024-12-17",
            "day": "Tuesday",
            "tasks": [
                {"endTime": "20:00:00", "name": "Join a pilates class", "startTime": "19:00:00"},
                {"endTime": "22:00:00", "name": "Read a book about social issues", "startTime": "21:00:00"},
                {"endTime": "21:00:00", "name": "Visit an escape room adventure", "startTime": "20:00:00"},
                {"endTime": "23:00:00", "name": "Practice problem-solving skills", "startTime": "22:00:00"}
            ]
        },
        "Wednesday": {
            "date": "2024-12-18",
            "day": "Wednesday",
            "tasks": [
                {"endTime": "20:00:00", "name": "Join a local swim team", "startTime": "19:00:00"},
                {"endTime": "22:00:00", "name": "Read a dystopian novel", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Create a gaming-themed cosplay", "startTime": "22:00:00"},
                {"endTime": "21:00:00", "name": "Explore a book on economics", "startTime": "20:00:00"}
            ]
        }
    });
    pub static ref SCHEDULE_4: serde_json::Value = json!({
        "Friday": {
            "date": "2024-12-20",
            "day": "Friday",
            "tasks": [
                {"endTime": "19:00:00", "name": "Play a round of golf", "startTime": "18:00:00"},
                {"endTime": "21:00:00", "name": "Explore science fiction literature", "startTime": "20:00:00"},
                {"endTime": "22:00:00", "name": "Experience a virtual reality game night", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Experience a night market", "startTime": "22:00:00"}
            ]
        },
        "Monday": {
            "date": "2024-12-16",
            "day": "Monday",
            "tasks": [
                {"endTime": "18:00:00", "name": "Try surfing", "startTime": "17:00:00"},
                {"endTime": "20:00:00", "name": "Visit a planetarium star show", "startTime": "19:00:00"},
                {"endTime": "22:00:00", "name": "Analyze the design of a game you love", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Create flashcards for vocabulary", "startTime": "22:00:00"}
            ]
        },
        "Saturday": {
            "date": "2024-12-14",
            "day": "Saturday",
            "tasks": [
                {"endTime": "18:00:00", "name": "Cycle for 10 miles", "startTime": "17:00:00"},
                {"endTime": "20:00:00", "name": "Stay updated with course announcements", "startTime": "19:00:00"},
                {"endTime": "22:00:00", "name": "Go to a rooftop cinema", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Read a poetry collection", "startTime": "22:00:00"}
            ]
        },
        "Sunday": {
            "date": "2024-12-15",
            "day": "Sunday",
            "tasks": [
                {"endTime": "18:00:00", "name": "Read a book about history", "startTime": "17:00:00"},
                {"endTime": "20:00:00", "name": "Collaborate with friends to create a gaming podcast", "startTime": "19:00:00"},
                {"endTime": "22:00:00", "name": "Visit an escape room adventure", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Read a thriller or mystery novel", "startTime": "22:00:00"}
            ]
        },
        "Thursday": {
            "date": "2024-12-19",
            "day": "Thursday",
            "tasks": [
                {"endTime": "18:00:00", "name": "Go on a food truck night tour", "startTime": "17:00:00"},
                {"endTime": "20:00:00", "name": "Reflect on learning experiences", "startTime": "19:00:00"},
                {"endTime": "22:00:00", "name": "Read a book recommended by a friend", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Create a custom level or map in a game", "startTime": "22:00:00"}
            ]
        },
        "Tuesday": {
            "date": "2024-12-17",
            "day": "Tuesday",
            "tasks": [
                {"endTime": "18:00:00", "name": "Participate in a mini-triathlon", "startTime": "17:00:00"},
                {"endTime": "20:00:00", "name": "Develop critical thinking skills", "startTime": "19:00:00"},
                {"endTime": "22:00:00", "name": "Play a retro game from your childhood", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Join an online gaming community or forum", "startTime": "22:00:00"}
            ]
        },
        "Wednesday": {
            "date": "2024-12-18",
            "day": "Wednesday",
            "tasks": [
                {"endTime": "19:00:00", "name": "Explore virtual reality games", "startTime": "18:00:00"},
                {"endTime": "21:00:00", "name": "Read a book by a local author", "startTime": "20:00:00"},
                {"endTime": "22:00:00", "name": "Learn about game development history", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Explore historical fiction", "startTime": "22:00:00"}
            ]
        }
    });
    pub static ref SCHEDULE_5: serde_json::Value = json!({
        "Friday": {
            "date": "2024-12-20",
            "day": "Friday",
            "tasks": [
                {"endTime": "18:00:00", "name": "Prepare presentation materials", "startTime": "17:00:00"},
                {"endTime": "20:00:00", "name": "Join a book club", "startTime": "19:00:00"},
                {"endTime": "22:00:00", "name": "Try night kayaking or paddleboarding", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Try a new martial art", "startTime": "22:00:00"}
            ]
        },
        "Monday": {
            "date": "2024-12-16",
            "day": "Monday",
            "tasks": [
                {"endTime": "19:00:00", "name": "Go bowling at a night bowling club", "startTime": "18:00:00"},
                {"endTime": "21:00:00", "name": "Read a bestseller", "startTime": "20:00:00"},
                {"endTime": "22:00:00", "name": "Complete practice problems", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Prepare presentation materials", "startTime": "22:00:00"}
            ]
        },
        "Saturday": {
            "date": "2024-12-14",
            "day": "Saturday",
            "tasks": [
                {"endTime": "17:00", "name": "Play a game of soccer", "startTime": "16:00"},
                {"endTime": "19:00", "name": "Read a book from the library", "startTime": "18:00"},
                {"endTime": "21:00", "name": "Write essays on assigned topics", "startTime": "20:00"},
                {"endTime": "23:00", "name": "Enjoy a comedy show at a local club", "startTime": "22:00"}
            ]
        },
        "Sunday": {
            "date": "2024-12-15",
            "day": "Sunday",
            "tasks": [
                {"endTime": "19:00:00", "name": "Go bowling at a night bowling club", "startTime": "18:00:00"},
                {"endTime": "21:00:00", "name": "Experiment with streaming your gameplay", "startTime": "20:00:00"},
                {"endTime": "23:00:00", "name": "Try out for a local dodgeball league", "startTime": "22:00:00"},
                {"endTime": "20:00:00", "name": "Read peer-reviewed journal articles", "startTime": "19:00:00"}
            ]
        },
        "Thursday": {
            "date": "2024-12-19",
            "day": "Thursday",
            "tasks": [
                {"endTime": "18:30:00", "name": "Play a game of badminton", "startTime": "17:30:00"},
                {"endTime": "20:30:00", "name": "Explore fantasy literature", "startTime": "19:30:00"},
                {"endTime": "22:30:00", "name": "Attend a live music concert", "startTime": "21:30:00"},
                {"endTime": "23:30:00", "name": "Write a game tutorial or guide", "startTime": "22:30:00"}
            ]
        },
        "Tuesday": {
            "date": "2024-12-17",
            "day": "Tuesday",
            "tasks": [
                {"endTime": "18:00:00", "name": "Play a game of soccer", "startTime": "17:00:00"},
                {"endTime": "20:00:00", "name": "Explore science fiction literature", "startTime": "19:00:00"},
                {"endTime": "22:00:00", "name": "Try night kayaking or paddleboarding", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Read a book that was adapted into a movie", "startTime": "22:00:00"}
            ]
        },
        "Wednesday": {
            "date": "2024-12-18",
            "day": "Wednesday",
            "tasks": [
                {"endTime": "18:00:00", "name": "Practice shooting hoops", "startTime": "17:00:00"},
                {"endTime": "20:00:00", "name": "Read a book about self-improvement", "startTime": "19:00:00"},
                {"endTime": "22:00:00", "name": "Experience a silent disco party", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Try night-time zip-lining", "startTime": "22:00:00"}
            ]
        }
    });
    pub static ref ALL_SCHEDULES: Vec<&'static serde_json::Value> = vec![
        &*SCHEDULE_1,
        &*SCHEDULE_2,
        &*SCHEDULE_3,
        &*SCHEDULE_4,
        &*SCHEDULE_5
    ];
}
