use error_chain::error_chain;
use std::io::Read;
use serde::Deserialize;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        Serde(::serde_json::Error); 
    }
}

#[derive(Debug, Deserialize)]
struct ActionsStatus {
    workflow_runs: Vec<WorkflowRun>,
}

#[derive(Debug, Deserialize)]
struct WorkflowRun {
    status: String,
}

fn main() -> Result<()> {
    let owner = "stuartthompson";
    let repo = "table-format";
    let workflow_id = "ci.yml";

    let url = format!(
        "https://api.github.com/repos/{}/{}/actions/workflows/{}/runs",
        owner, repo, workflow_id
    );

    let client = reqwest::blocking::Client::new();
    let mut response = client.get(url)
        .header("User-Agent", "request")
        .send()?;
    let mut body = String::new();
    response.read_to_string(&mut body)?;

    let status: ActionsStatus = serde_json::from_str(&body)?;

    println!("Runs: {:?}", status);
    println!("Total workflow runs: {}", status.workflow_runs.len());
    println!("Status: {}", status.workflow_runs[0].status);

    Ok(())
}

