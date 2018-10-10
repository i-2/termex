use std::fs::File;
use std::io::Result as ioResult;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Child, Command};

pub fn reload() -> ioResult<Child> {
    return Command::new("history").arg("-r").spawn();
}

pub struct HistoryFile {
    /// path where the temporary history of bash history file is located
    /// by default takes it form HISTFILE environmental variable.
    file: PathBuf,
    /// Inorder to facilitate fast io the new history is store in the memory
    /// via buffer.
    buffer: Vec<String>,
    /// a countable number of history which is being stored
    count: u32,
}

impl HistoryFile {
    pub fn new(path: PathBuf) -> Self {
        HistoryFile {
            file: path,
            buffer: Vec::new(),
            count: 0,
        }
    }

    pub fn append(&mut self, history: String) -> ioResult<()> {
        self.buffer.push(history);
        self.count += 1;
        Ok(())
    }

    pub fn count(&self) -> usize {
        self.count as usize
    }
}

impl Drop for HistoryFile {
    fn drop(&mut self) {
        // history
        let file_wrap = File::open(self.file.to_owned());
        let mut file: File = match file_wrap {
            Ok(_file) => _file,
            Err(_) => {
                return;
            }
        };
        let joined = self.buffer.join("\n");
        file.write_all(joined.as_bytes()).expect("Sync failed");
        reload();
    }
}
