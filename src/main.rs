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

    let actions_status: ActionsStatus = serde_json::from_str(&body)?;

    println!("Status: {}", actions_status.workflow_runs[0].status);

    // println!("Response: {}", body);
    //println!("Total Workflow Runs: {}", response.total_count);
    // for run in response.workflow_runs {
    //    println!("ID: {}, Status: {}, Conclusion: {:?}", run.id, run.status, run.conclusion);
    // }

    Ok(())
}

