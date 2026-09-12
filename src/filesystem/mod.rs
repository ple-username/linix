// Filesystem Module - Simple FAT32-like filesystem

use spin::Mutex;

const MAX_FILES: usize = 256;
const MAX_FILENAME: usize = 256;

#[derive(Clone)]
pub struct FileMetadata {
    pub name: [u8; MAX_FILENAME],
    pub size: u64,
    pub is_directory: bool,
    pub created_time: u64,
}

pub struct File {
    pub metadata: FileMetadata,
    pub data: [u8; 4096],
    pub data_size: usize,
}

impl File {
    pub fn new(name: &str, is_directory: bool) -> Self {
        let mut filename = [0u8; MAX_FILENAME];
        let name_bytes = name.as_bytes();
        let len = name_bytes.len().min(MAX_FILENAME);
        filename[..len].copy_from_slice(&name_bytes[..len]);

        File {
            metadata: FileMetadata {
                name: filename,
                size: 0,
                is_directory,
                created_time: 0,
            },
            data: [0u8; 4096],
            data_size: 0,
        }
    }

    pub fn write(&mut self, data: &[u8]) -> bool {
        if self.data_size + data.len() > 4096 {
            return false;
        }
        self.data[self.data_size..self.data_size + data.len()].copy_from_slice(data);
        self.data_size += data.len();
        self.metadata.size = self.data_size as u64;
        true
    }

    pub fn read(&self) -> &[u8] {
        &self.data[..self.data_size]
    }
}

pub struct Filesystem {
    files: [Option<File>; MAX_FILES],
    file_count: usize,
}

impl Filesystem {
    pub const fn new() -> Self {
        const NONE: Option<File> = None;
        Filesystem {
            files: [NONE; MAX_FILES],
            file_count: 0,
        }
    }

    pub fn create_file(&mut self, name: &str) -> bool {
        if self.file_count >= MAX_FILES {
            return false;
        }
        self.files[self.file_count] = Some(File::new(name, false));
        self.file_count += 1;
        true
    }

    pub fn find_file(&mut self, name: &str) -> Option<&mut File> {
        for file in &mut self.files {
            if let Some(f) = file {
                let fname = core::str::from_utf8(&f.metadata.name)
                    .unwrap_or("");
                if fname.trim_end_matches('\0') == name {
                    return Some(f);
                }
            }
        }
        None
    }

    pub fn list_files(&self) {
        println!("\nFilesystem Contents:");
        println!("─────────────────────────────────────");
        for file in &self.files {
            if let Some(f) = file {
                let fname = core::str::from_utf8(&f.metadata.name)
                    .unwrap_or("Unknown")
                    .trim_end_matches('\0');
                let ftype = if f.metadata.is_directory { "[DIR]" } else { "[FILE]" };
                println!("{} {} {} bytes", ftype, fname, f.metadata.size);
            }
        }
        println!("─────────────────────────────────────");
    }
}

pub static FILESYSTEM: Mutex<Filesystem> = Mutex::new(Filesystem::new());

pub fn init() {
    let mut fs = FILESYSTEM.lock();
    fs.create_file("boot.log");
    fs.create_file("system.cfg");
    fs.create_file("readme.txt");
    drop(fs);
    println!("Filesystem initialized with 3 files");
}
