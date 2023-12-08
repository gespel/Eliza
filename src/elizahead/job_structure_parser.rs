use std::fs;
use std::path::{Path, PathBuf};
use log::info;

pub struct JobStructureParser {

}

struct JobFolder {
    path: String
}
struct JobFile {
    path: String,
    content: String
}

struct Job {
    name: String,
    folders: Vec<JobFolder>,
    files: Vec<JobFile>
}
impl Job {
    fn new(name: String) -> Job {
        Job {
            name,
            folders: Vec::new(),
            files: Vec::new(),
        }
        
    }

    fn parse_job_folder(&self) {
        let basepath = format!("jobs/{}", self.name);

        if let Ok(entries) = fs::read_dir(&basepath) {
            for entry in entries.flatten() {
                let path = entry.path();

                if path.is_dir() {
                    let jf = JobFolder {
                        path: format!("{}/{}", basepath.clone(), path.file_name().unwrap().to_str().unwrap()),
                    };
                    info!("Jobfolder found! {}", jf.path);
                }
                else if path.is_file() {

                }
                else {

                }
            }
        }
    }
}



impl JobStructureParser {
    pub fn new() -> JobStructureParser {
        info!("Checking if base folder structure exists...");
        if !Path::new("jobs").exists() {
            fs::create_dir("jobs").expect("Error while creating jobs folder");
        }
        info!("Done!");

        JobStructureParser {

        }
    }

    pub fn parse_jobs(&self) {
        if let Ok(entries) = fs::read_dir("jobs") {
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                        info!("Project found! {:?} parsing now...", entry.file_name());
                        let j = Job::new(entry.file_name().into_string().unwrap());
                        j.parse_job();
                    }
                }
            }
        }
    }
}