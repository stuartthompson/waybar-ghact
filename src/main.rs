use std::time::Duration;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct ActionsStatus {
    workflow_runs: Vec<WorkflowRun>,
}

#[derive(Debug, Deserialize)]
struct WorkflowRun {
    status: String,
}

fn fetch_actions_status(url: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let response = client.get(url).send()?;
    let status: ActionsStatus = response.json()?;

    // Get the status of the latest workflow run
    if let Some(run) = status.workflow_runs.first() {
        Ok(run.status.clone())
    } else {
        Err(reqwest::Error::from("No workflow runs found"))
    }
}

fn generate_output(status: &str) -> String {
    match status {
        "completed" => "<span foreground=\"#00ff00\">✓</span>".to_string(),
        "in_progress" => "<span foreground=\"#d38b0d\">◷</span>".to_string(),
        _ => "<span foreground=\"#ff0000\">✗</span>".to_string()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        panic!("At least one argument required.");
    }
    let url = &args[1];
    let update_interval = Duration::from_secs(60);

    loop {
        match fetch_actions_status(url) {
            Ok(status) => {
                let output = generate_output(&status);
                println!("{}", output);
            },
            Err(err) => eprintln!("Error: {}", err),
        }
        std::thread::sleep(update_interval);
    }
}

