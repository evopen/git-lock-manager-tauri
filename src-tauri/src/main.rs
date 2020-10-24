#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use lazy_static::lazy_static;
use nfd::DialogType;
use serde::Serialize;
use std::fs::read_dir;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;
use std::time::SystemTime;
use tauri::api::dialog::Response;
use walkdir::WalkDir;

mod cmd;

lazy_static! {
  static ref LAST_SEARCH: SystemTime = SystemTime::now();
  static ref CACHED_LIST: Mutex<Vec<String>> = Mutex::new(Vec::new());
  static ref REPO: Mutex<String> = Mutex::new(String::new());
}

fn cache_list(path: &str) {
  let mut cached_file = CACHED_LIST.lock().unwrap();
  cached_file.clear();
  let list: Vec<String> = WalkDir::new(path)
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.path().is_file())
    .map(|e| String::from(e.path().to_str().unwrap()))
    .collect();
  cached_file.clone_from(&list);
}

#[derive(Serialize, Debug, Clone)]
struct LockEntry {
  file: String,
  user: String,
}

fn main() {
  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;
      match serde_json::from_str(arg) {
        Err(e) => Err(e.to_string()),
        Ok(command) => {
          match command {
            // definitions for your custom commands from Cmd here
            MyCustomCommand { argument } => {
              //  your command code
              println!("{}", argument);
            }
            SelectRepo { callback, error } => tauri::execute_promise(
              _webview,
              move || {
                let mut path: String = String::new();
                match nfd::open_dialog(None, None, DialogType::PickFolder).unwrap() {
                  Response::Okay(s) => {
                    path = s;
                  }
                  _ => {}
                }

                let mut is_git_repo = false;
                for entry in read_dir(&path).unwrap() {
                  let entry = entry.unwrap();
                  if entry.path().is_dir()
                    && entry
                      .path()
                      .file_name()
                      .unwrap()
                      .to_str()
                      .unwrap()
                      .eq(".git")
                  {
                    is_git_repo = true;
                    break;
                  }
                }
                if is_git_repo {
                  if CACHED_LIST.lock().unwrap().is_empty() || REPO.lock().unwrap().ne(&path) {
                    cache_list(&path);
                    REPO.lock().unwrap().clone_from(&path);
                  }
                  Ok(path)
                } else {
                  Ok("wrong folder".to_string())
                }
              },
              callback,
              error,
            ),
            QueryLocks { callback, error } => tauri::execute_promise(
              _webview,
              move || {
                let repo = REPO.lock().unwrap();
                std::env::set_current_dir(repo.as_str());
                let lock_list: Vec<String> = Vec::new();
                let output = Command::new("git")
                  .arg("lfs")
                  .arg("locks")
                  .output()
                  .expect("failed to execute git lfs locks")
                  .stdout;
                let buf = String::from_utf8(output).unwrap();
                let lock_entries: Vec<LockEntry> = buf
                  .lines()
                  .map(|l| {
                    let mut entry = l.split_whitespace().collect::<Vec<&str>>();
                    entry.pop();
                    LockEntry {
                      file: entry[0].to_string(),
                      user: entry[1].to_string(),
                    }
                  })
                  .collect();
                dbg!(&lock_entries);
                Ok(lock_entries)
              },
              callback,
              error,
            ),
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}
