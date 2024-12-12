use lazy_static::lazy_static;
use serde_json::json;

lazy_static! {
    pub static ref SCHEDULE_1: serde_json::Value = json!({
        "Friday": {
            "date": "2024-12-20",
            "day": "Friday",
            "tasks": [
                {"endTime": "19:00:00", "name": "Play a round of golf", "startTime": "18:00:00"},
                {"endTime": "21:00:00", "name": "Read a book about art or design", "startTime": "20:00:00"},
                {"endTime": "22:00:00", "name": "Participate in a dance competition", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Host a LAN party", "startTime": "22:00:00"}
            ]
        },
        "Monday": {
            "date": "2024-12-16",
            "day": "Monday",
            "tasks": [
                {"endTime": "19:00:00", "name": "Reflect on learning experiences", "startTime": "18:00:00"},
                {"endTime": "21:00:00", "name": "Attend a jazz night at a local bar", "startTime": "20:00:00"},
                {"endTime": "22:00:00", "name": "Test a beta version of an upcoming game", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Collaborate with friends to create a gaming podcast", "startTime": "22:00:00"}
            ]
        },
        "Saturday": {
            "date": "2024-12-14",
            "day": "Saturday",
            "tasks": [
                {"endTime": "20:00:00", "name": "Explore a non-fiction book on a new topic", "startTime": "19:00:00"},
                {"endTime": "22:00:00", "name": "Play a retro game from your childhood", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Participate in a pub crawl", "startTime": "22:00:00"},
                {"endTime": "21:00:00", "name": "Join a rugby team", "startTime": "20:00:00"}
            ]
        },
        "Sunday": {
            "date": "2024-12-15",
            "day": "Sunday",
            "tasks": [
                {"endTime": "19:00:00", "name": "Test a beta version of an upcoming game", "startTime": "18:00:00"},
                {"endTime": "21:00:00", "name": "Read a book from a different century", "startTime": "20:00:00"},
                {"endTime": "22:00:00", "name": "Explore religious or spiritual texts", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Read a short story collection", "startTime": "22:00:00"}
            ]
        },
        "Thursday": {
            "date": "2024-12-19",
            "day": "Thursday",
            "tasks": [
                {"endTime": "19:00:00", "name": "Complete a 100% game achievement or trophy list", "startTime": "18:00:00"},
                {"endTime": "21:00:00", "name": "Review past exams", "startTime": "20:00:00"},
                {"endTime": "22:00:00", "name": "Go on a food truck night tour", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Take a tennis lesson", "startTime": "22:00:00"}
            ]
        },
        "Tuesday": {
            "date": "2024-12-17",
            "day": "Tuesday",
            "tasks": [
                {"endTime": "19:00:00", "name": "Go for a 5K run", "startTime": "18:00:00"},
                {"endTime": "21:00:00", "name": "Start a gaming blog or vlog", "startTime": "20:00:00"},
                {"endTime": "22:00:00", "name": "Practice your golf swing at the driving range", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Read a book from the library", "startTime": "22:00:00"}
            ]
        },
        "Wednesday": {
            "date": "2024-12-18",
            "day": "Wednesday",
            "tasks": [
                {"endTime": "19:00:00", "name": "Create a playlist of your favorite video game soundtracks", "startTime": "18:00:00"},
                {"endTime": "21:00:00", "name": "Take a fencing lesson", "startTime": "20:00:00"},
                {"endTime": "22:00:00", "name": "Visit a hookah lounge", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Dance at a salsa club", "startTime": "22:00:00"}
            ]
        }
    });
    pub static ref SCHEDULE_2: serde_json::Value = json!({
        "Friday": {
            "date": "2024-12-20",
            "day": "Friday",
            "tasks": [
                {"endTime": "19:00:00", "name": "Review feedback from assignments", "startTime": "18:00:00"},
                {"endTime": "21:00:00", "name": "Take a boxing class", "startTime": "20:00:00"},
                {"endTime": "23:00:00", "name": "Attend a paint and sip night", "startTime": "22:00:00"},
                {"endTime": "22:00:00", "name": "Test yourself with flashcards", "startTime": "21:00:00"}
            ]
        },
        "Monday": {
            "date": "2024-12-16",
            "day": "Monday",
            "tasks": [
                {"endTime": "19:00:00", "name": "Review past exams", "startTime": "18:00:00"},
                {"endTime": "21:00:00", "name": "Try out for a volleyball team", "startTime": "20:00:00"},
                {"endTime": "22:00:00", "name": "Join a game development or design course", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Try your luck at a casino", "startTime": "22:00:00"}
            ]
        },
        "Saturday": {
            "date": "2024-12-14",
            "day": "Saturday",
            "tasks": [
                {"endTime": "19:00:00", "name": "Attend a cultural dance night", "startTime": "18:00:00"},
                {"endTime": "21:00:00", "name": "Explore a famous nightlife district", "startTime": "20:00:00"},
                {"endTime": "22:00:00", "name": "Visit a rooftop bar", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Go to a rooftop cinema", "startTime": "22:00:00"}
            ]
        },
        "Sunday": {
            "date": "2024-12-15",
            "day": "Sunday",
            "tasks": [
                {"endTime": "19:00:00", "name": "Visit an escape room adventure", "startTime": "18:00:00"},
                {"endTime": "21:00:00", "name": "Read a children's classic", "startTime": "20:00:00"},
                {"endTime": "22:00:00", "name": "Experiment with streaming your gameplay", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Reflect on learning experiences", "startTime": "22:00:00"}
            ]
        },
        "Thursday": {
            "date": "2024-12-19",
            "day": "Thursday",
            "tasks": [
                {"endTime": "19:00:00", "name": "Join a pilates class", "startTime": "18:00:00"},
                {"endTime": "21:00:00", "name": "Create mind maps", "startTime": "20:00:00"},
                {"endTime": "22:00:00", "name": "Explore historical fiction", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Visit a hookah lounge", "startTime": "22:00:00"}
            ]
        },
        "Tuesday": {
            "date": "2024-12-17",
            "day": "Tuesday",
            "tasks": [
                {"endTime": "19:00:00", "name": "Join a night-time yoga session", "startTime": "18:00:00"},
                {"endTime": "21:00:00", "name": "Review course syllabus", "startTime": "20:00:00"},
                {"endTime": "22:00:00", "name": "Go to a karaoke night", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Read a book that was adapted into a movie", "startTime": "22:00:00"}
            ]
        },
        "Wednesday": {
            "date": "2024-12-18",
            "day": "Wednesday",
            "tasks": [
                {"endTime": "20:00:00", "name": "Explore contemporary fiction", "startTime": "19:00:00"},
                {"endTime": "21:00:00", "name": "Write a game tutorial or guide", "startTime": "20:00:00"},
                {"endTime": "23:00:00", "name": "Explore a book on philosophy", "startTime": "22:00:00"},
                {"endTime": "22:00:00", "name": "Collaborate with friends to create a gaming podcast", "startTime": "21:00:00"}
            ]
        }
    });
    pub static ref SCHEDULE_3: serde_json::Value = json!({
        "Friday": {
            "date": "2024-12-20",
            "day": "Friday",
            "tasks": [
                {"endTime": "19:00:00", "name": "Join a local basketball league", "startTime": "18:00:00"},
                {"endTime": "21:00:00", "name": "Read a book with a protagonist of a different ethnicity", "startTime": "20:00:00"},
                {"endTime": "22:00:00", "name": "Attend a live theatre performance", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Experience a silent disco party", "startTime": "22:00:00"}
            ]
        },
        "Monday": {
            "date": "2024-12-16",
            "day": "Monday",
            "tasks": [
                {"endTime": "19:00:00", "name": "Go for a 5K run", "startTime": "18:00:00"},
                {"endTime": "21:00:00", "name": "Analyze the design of a game you love", "startTime": "20:00:00"},
                {"endTime": "23:00:00", "name": "Experience a silent disco party", "startTime": "22:00:00"},
                {"endTime": "20:00:00", "name": "Subscribe to a literary magazine", "startTime": "19:00:00"}
            ]
        },
        "Saturday": {
            "date": "2024-12-14",
            "day": "Saturday",
            "tasks": [
                {"endTime": "18:00", "name": "Take a tennis lesson", "startTime": "17:00"},
                {"endTime": "20:00", "name": "Explore contemporary fiction", "startTime": "19:00"},
                {"endTime": "22:00", "name": "Review a game and share your thoughts online", "startTime": "21:00"}
            ]
        },
        "Sunday": {
            "date": "2024-12-15",
            "day": "Sunday",
            "tasks": [
                {"endTime": "19:00:00", "name": "Read a book from the library", "startTime": "18:00:00"},
                {"endTime": "21:00:00", "name": "Create a study schedule", "startTime": "20:00:00"},
                {"endTime": "22:00:00", "name": "Experience a night market", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Take a tennis lesson", "startTime": "22:00:00"}
            ]
        },
        "Thursday": {
            "date": "2024-12-19",
            "day": "Thursday",
            "tasks": [
                {"endTime": "19:00:00", "name": "Play a retro game from your childhood", "startTime": "18:00:00"},
                {"endTime": "21:00:00", "name": "Create a study schedule", "startTime": "20:00:00"},
                {"endTime": "22:00:00", "name": "Participate in a dance competition", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Build a gaming PC or console setup", "startTime": "22:00:00"}
            ]
        },
        "Tuesday": {
            "date": "2024-12-17",
            "day": "Tuesday",
            "tasks": [
                {"endTime": "19:00:00", "name": "Join a local football team", "startTime": "18:00:00"},
                {"endTime": "21:00:00", "name": "Analyze the design of a game you love", "startTime": "20:00:00"},
                {"endTime": "22:00:00", "name": "Go to a karaoke night", "startTime": "21:00:00"},
                {"endTime": "23:00:00", "name": "Go skiing or snowboarding", "startTime": "22:00:00"}
            ]
        },
        "Wednesday": {
            "date": "2024-12-18",
            "day": "Wednesday",
            "tasks": [
                {"endTime": "20:00:00", "name": "Experience a virtual reality game night", "startTime": "19:00:00"},
                {"endTime": "21:00:00", "name": "Attend a yoga class", "startTime": "20:00:00"},
                {"endTime": "23:00:00", "name": "Explore a book on economics", "startTime": "22:00:00"}
            ]
        }
    });
    pub static ref ALL_SCHEDULES: Vec<&'static serde_json::Value> =
        vec![&*SCHEDULE_1, &*SCHEDULE_2, &*SCHEDULE_3];
}
