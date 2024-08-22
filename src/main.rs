use warp::{Filter, Rejection, Reply};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let tasks: Arc<RwLock<Vec<String>>> = Arc::new(RwLock::new(Vec::new()));

    // Serve static files (CSS and JavaScript)
    let static_files = warp::fs::dir("static");

    // Serve the main HTML page
    let index = warp::path::end()
        .map(move || {
            let html_content = include_str!("../templates/index.html");
            warp::reply::html(html_content)
        });

    // Handle adding tasks
    let add_task = warp::path("add")
        .and(warp::post())
        .and(warp::body::form())
        .and(with_tasks(tasks.clone()))
        .map(|form: HashMap<String, String>, tasks: Arc<RwLock<Vec<String>>>| {
            if let Some(task) = form.get("task") {
                let mut tasks = tasks.write().unwrap();
                tasks.push(task.clone());
                warp::reply::html("Task added successfully. <a href=\"/\">Go back</a>")
            } else {
                warp::reply::html("Error: No task provided. <a href=\"/\">Go back</a>")
            }
        });

    // Serve the current task list
    let get_tasks = warp::path("tasks")
        .and(warp::get())
        .and(with_tasks(tasks.clone()))
        .map(|tasks: Arc<RwLock<Vec<String>>>| {
            let tasks = tasks.read().unwrap();
            let html = tasks.iter()
                .map(|task| format!("<li>{}</li>", task))
                .collect::<String>();
            format!("<ul>{}</ul>", html)
        });

    // Combine routes
    let routes = index.or(add_task).or(get_tasks).or(static_files);

    // Start the server
    warp::serve(routes)
        .run(([0, 0, 0, 0], 3030))
        .await;
}

fn with_tasks(
    tasks: Arc<RwLock<Vec<String>>>,
) -> impl Filter<Extract = (Arc<RwLock<Vec<String>>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || tasks.clone())
}

