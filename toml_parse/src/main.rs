#![allow(unused)]
use serde::Deserialize;
use std::fs;
use toml::Table;

#[derive(Debug, Deserialize)]
struct Project {
    name: String,
    description: String,
    github_link: String,
    readme_link: String,
}

#[derive(Debug, Deserialize)]
struct ProjectsFile {
    projects: Vec<Project>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let toml_str = fs::read_to_string("projects.toml")?;
    let data: ProjectsFile = toml::from_str(&toml_str)?;

    for project in data.projects {
        println!("{}", project.name);
        println!("{}", project.description);
        println!("{}", project.github_link);
        println!("{}", project.readme_link);
    }
    Ok(())
}
