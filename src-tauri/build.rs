use std::{fs::File, io::Write, path::Path};

fn main() {
  let env_path = Path::new(".env");
  if !env_path.exists() {
      let mut file = File::create(env_path).expect("Failed to create `.env` file");
      writeln!(file, "OAI_KEY=").expect("Failed to write to .env file");
  }
  
  tauri_build::build()
}
