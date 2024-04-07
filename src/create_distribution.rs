use std::{fs, io};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::args::CreateDistribution;

#[derive(Debug)]
struct JobConfig {
    distribution_name: String,
    template_dir: String,
    templates_repo: String,
}

impl JobConfig {
    fn new(name: &str) -> Self {
        let temp_id = Uuid::new_v4().to_string();
        let target_path = String::from("/Temp/") + &temp_id;

        Self {
            distribution_name: String::from(name),
            template_dir: target_path,
            templates_repo: String::from("https://source-dev.webprovisions.io/gitea_admin/widget-service-templates.git"),
        }
    }

    fn get_distribution_dir(&self) -> String {
        let path_str = String::from("./") + &self.distribution_name;
        path_str
    }
}

pub fn create_distribution_action(data: &CreateDistribution) {
    println!("Creating distribution with name: {}", data.name);

    let config = JobConfig::new(&data.name);

    if (Path::new(&config.get_distribution_dir()).exists()) {
        println!("A folder named {} already exists.", &config.get_distribution_dir());
        return;
    }

    // clone template repo
    clone_template_repo(&config);

    // copy cloned template to the new distribution directory
    copy_template_to_distribution_dir(&config);

    // update the package.json file in the created directory
    update_package_json(&config);

    // commit the initial changes
    // commit_initial_changes(&config);

    // remove temporary files
    if let Err(e) = remove_temp_dir(&config) {
        eprintln!("error: Unable to clean up temporary files.");
        return;
    }

    println!("Created boilerplate in directory: {}", config.distribution_name)
}

fn clone_template_repo(config: &JobConfig) {
    let mut git_clone = Command::new("git")
        .args(["clone", &config.templates_repo, &config.template_dir])
        .output()
        .expect("failed to execute process");

    io::stdout().write_all(&git_clone.stdout).unwrap();
    io::stderr().write_all(&git_clone.stderr).unwrap();
}

fn copy_template_to_distribution_dir(config: &JobConfig) {
    let distribution_dir = config.get_distribution_dir();
    let template_path = String::from(&config.template_dir) + "/templates/cli";

    let result = copy_dir::copy_dir(Path::new(&template_path), Path::new(&distribution_dir));

    if let Err(e) = result {
        panic!("Unable to copy template files: {}", e);
    }
}

fn commit_initial_changes(config: &JobConfig) {
    let cwd = config.get_distribution_dir();
    Command::new("git")
        .current_dir(cwd)
        .args(["init"])
        .output()
        .expect("failed to initialize git repository");

    let cwd = config.get_distribution_dir();
    Command::new("git")
        .current_dir(cwd)
        .args(["add", "-A"])
        .output()
        .expect("failed to stage files");

    let cwd = config.get_distribution_dir();
    Command::new("git")
        .current_dir(cwd)
        .args(["commit", "-m", "Initial commit"])
        .output()
        .expect("failed to stage files");
}

fn remove_temp_dir(config: &JobConfig) -> io::Result<()> {
    fs::remove_dir_all(&config.template_dir)?;

    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
struct PackageJson {
    name: String,
    version: String,
    description: String,
    main: String,
    #[serde(rename = "type")]
    package_type: String,
    author: String,
    dependencies: std::collections::HashMap<String, String>,
    scripts: std::collections::HashMap<String, String>,
    #[serde(rename = "devDependencies")]
    dev_dependencies: std::collections::HashMap<String, String>,
}

fn update_package_json(config: &JobConfig) {
    let path_to_json_string = config.get_distribution_dir() + "/package.json";

    let json_path = Path::new(&path_to_json_string);
    let file = File::open(json_path).expect("Unable to read package.json");

    let mut json: PackageJson = serde_json::from_reader(file).expect("Error parsing json");
    json.name = String::from(&config.distribution_name);

    // open file again for writing
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(json_path)
        .expect("Unable to open package.json for writing");

    let serialized = serde_json::to_string_pretty(&json).expect("Unable to serialize json");
    file.write_all(serialized.as_bytes()).expect("Unable to write json");
}