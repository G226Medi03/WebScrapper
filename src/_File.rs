use std::{
    fs::{DirEntry, File, ReadDir},
    io::{BufRead, BufReader},
    path::PathBuf,
};
static mut LINECOUNT: usize = 0;
fn GetCurDirPath() -> PathBuf {
    let path = std::env::current_dir().unwrap();
    return path;
}

pub fn PrintLinesOfCode() {
    println!("{}", GetLinesOfCode());
}
pub fn GetLinesOfCode() -> usize {
    let rustFilesLineCount = getLinesOfFiles("src");
    let staticFilesLineCount = getLinesOfFiles("static");
    let pythonFilesLineCount = 35;
    println!("{:?}:{:?}:{:?}", rustFilesLineCount, staticFilesLineCount, pythonFilesLineCount);
    return rustFilesLineCount + staticFilesLineCount + pythonFilesLineCount;
}
fn getLinesOfFiles(folderName: &str) -> usize {
    let curPath = GetCurDirPath();

    for entry in std::fs::read_dir(curPath).unwrap() {
        let entry: std::fs::DirEntry = entry.unwrap();

        if entry.file_name().to_str().unwrap().contains(folderName) {
            let dir1 = std::fs::read_dir(entry.file_name()).unwrap();
            HandleDirectory(dir1);
            break;
        }
    }
    unsafe {
        let ret = LINECOUNT.clone();
        LINECOUNT = 0;
        return ret;
    }
}

struct MyFile {
    entry: DirEntry,
}

impl MyFile {
    pub fn New(entry: DirEntry) -> Self {
        return MyFile { entry: entry };
    }

    pub fn path(&self) -> PathBuf {
        return self.entry.path();
    }

    // pub fn is_file(&self) -> bool {
    //     return self.entry.metadata().unwrap().is_file();
    // }

    // pub fn is_dir(&self) -> bool {
    //     return self.entry.metadata().unwrap().is_dir();
    // }

    // pub fn filename(&self) -> OsString {
    //     return self.entry.file_name();
    // }

    pub fn GetFileLineCount(&self) -> usize {
        let mut lineCount = 0;
        let file = File::open(self.path()).unwrap();

        let reader = BufReader::new(file);
        for _ in reader.lines() {
            lineCount += 1;
        }

        return lineCount;
    }
}

pub fn HandleDirectory(dir: ReadDir) {
    for entry in dir {
        let entry = entry.unwrap();

        if entry.metadata().unwrap().is_file() {
            let file = MyFile::New(entry);
            unsafe {
                LINECOUNT += file.GetFileLineCount();
            }
        } else {
            let dirPath = std::fs::read_dir(entry.path()).unwrap();
            HandleDirectory(dirPath);
        }
    }
}
