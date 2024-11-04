use crate::models::ai::{ClusteredUsers, TaskRecommendation};
use crate::models::userdata::{
    DailySchedule, PersonalityScores, Task, UserProfile, UserProfileRequest, UserTask,
};

use std::sync::OnceLock;

// Static storage for our mock database
static MOCK_USERS: OnceLock<Vec<UserProfile>> = OnceLock::new();
static MOCK_CLUSTERS: OnceLock<Vec<ClusteredUsers>> = OnceLock::new();
static MOCK_TASKS: OnceLock<Vec<Task>> = OnceLock::new();
static MOCK_USER_TASKS: OnceLock<Vec<UserTask>> = OnceLock::new();

// Initialize mock data
fn get_mock_users() -> &'static Vec<UserProfile> {
    MOCK_USERS.get_or_init(|| {
        vec![
            UserProfile {
                id: 1,
                scores: PersonalityScores {
                    introverted: 70.0,
                    extraverted: 30.0,
                    observant: 65.0,
                    intuitive: 35.0,
                    thinking: 75.0,
                    feeling: 25.0,
                    judging: 60.0,
                    prospecting: 40.0,
                    assertive: 85.0,
                    turbulent: 15.0,
                },
                preferences: vec!["Finance".to_string(), "Technology".to_string()],
            },
            UserProfile {
                id: 2,
                scores: PersonalityScores {
                    introverted: 40.0,
                    extraverted: 60.0,
                    observant: 45.0,
                    intuitive: 55.0,
                    thinking: 50.0,
                    feeling: 50.0,
                    judging: 45.0,
                    prospecting: 55.0,
                    assertive: 60.0,
                    turbulent: 40.0,
                },
                preferences: vec!["Health".to_string(), "Social".to_string()],
            },
        ]
    })
}

fn get_mock_clusters() -> &'static Vec<ClusteredUsers> {
    MOCK_CLUSTERS.get_or_init(|| {
        vec![
            ClusteredUsers { id: 1, cluster: 0 },
            ClusteredUsers { id: 2, cluster: 1 },
            ClusteredUsers { id: 3, cluster: 0 },
        ]
    })
}

fn get_mock_tasks() -> &'static Vec<Task> {
    MOCK_TASKS.get_or_init(|| {
        vec![
            Task {
                name: "Morning Exercise".to_string(),
                category: "Health".to_string(),
            },
            Task {
                name: "Study Programming".to_string(),
                category: "Learning".to_string(),
            },
            Task {
                name: "Financial Planning".to_string(),
                category: "Finance".to_string(),
            },
        ]
    })
}

fn get_mock_user_tasks() -> &'static Vec<UserTask> {
    MOCK_USER_TASKS.get_or_init(|| {
        vec![
            UserTask {
                id: 1,
                tasks: vec![
                    "Morning Exercise".to_string(),
                    "Study Programming".to_string(),
                ],
            },
            UserTask {
                id: 2,
                tasks: vec!["Financial Planning".to_string()],
            },
        ]
    })
}

pub struct MockDb;

impl MockDb {
    pub async fn get_userdata(req: UserProfileRequest) -> Vec<UserProfile> {
        get_mock_users()
            .iter()
            .filter(|user| req.ids.contains(&user.id))
            .cloned()
            .collect()
    }

    pub async fn get_all_userdata() -> Vec<UserProfile> {
        get_mock_users()
            .iter()
            .map(|user| UserProfile {
                id: user.id,
                scores: PersonalityScores {
                    introverted: user.scores.introverted,
                    extraverted: user.scores.extraverted,
                    observant: user.scores.observant,
                    intuitive: user.scores.intuitive,
                    thinking: user.scores.thinking,
                    feeling: user.scores.feeling,
                    judging: user.scores.judging,
                    prospecting: user.scores.prospecting,
                    assertive: user.scores.assertive,
                    turbulent: user.scores.turbulent,
                },
                preferences: user.preferences.clone(),
            })
            .collect()
    }

    pub async fn get_clustered_users() -> Vec<ClusteredUsers> {
        get_mock_clusters().to_vec()
    }

    pub async fn get_tasks() -> Vec<Task> {
        get_mock_tasks().to_vec()
    }

    pub async fn get_users_with_tasks() -> Vec<UserTask> {
        get_mock_user_tasks().to_vec()
    }

    pub async fn get_weekly_schedules(weeks: i32) -> Vec<serde_json::Value> {
        (0..weeks)
            .map(|week| {
                serde_json::json!({
                    "week": format!("Week {}", week + 1),
                    "schedule": vec![
                        DailySchedule {
                            task_name: "Morning Exercise".to_string(),
                            start_time: "08:00".to_string(),
                            duration: 30,
                        },
                        DailySchedule {
                            task_name: "Study Programming".to_string(),
                            start_time: "10:00".to_string(),
                            duration: 60,
                        },
                    ]
                })
            })
            .collect()
    }

    #[allow(unused_variables)]
    pub async fn get_recommendations(
        scores: Vec<f32>,
        preferences: Vec<String>,
        work_end_time: i32,
        sleep_time: i32,
    ) -> Vec<TaskRecommendation> {
        let mut recommendations = vec![
            TaskRecommendation {
                task_name: "Morning Exercise".to_string(),
                score: 0.85,
                category: "Health".to_string(),
                suggested_time: 800, // 8:00 AM
            },
            TaskRecommendation {
                task_name: "Study Programming".to_string(),
                score: 0.75,
                category: "Learning".to_string(),
                suggested_time: 1000, // 10:00 AM
            },
            TaskRecommendation {
                task_name: "Financial Planning".to_string(),
                score: 0.70,
                category: "Finance".to_string(),
                suggested_time: 1400, // 2:00 PM
            },
            TaskRecommendation {
                task_name: "Evening Workout".to_string(),
                score: 0.65,
                category: "Health".to_string(),
                suggested_time: 1730, // 5:30 PM
            },
            TaskRecommendation {
                task_name: "Reading Time".to_string(),
                score: 0.60,
                category: "Learning".to_string(),
                suggested_time: 2045, // 8:45 PM
            },
        ];

        // Filter recommendations based on preferences
        recommendations.retain(|rec| preferences.contains(&rec.category));

        // Sort by score
        recommendations.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        recommendations
    }
}
