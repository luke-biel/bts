use std::cmp::Ordering;
use std::fs::{DirEntry, File};
use std::hash::Hasher;
use std::io::Read;
use std::path::{Path, PathBuf};

pub struct AcceptanceTestResult {
    pub result: bool,
    pub errors: Vec<ExtraEntry<String>>,
}

#[derive(Debug)]
pub enum ExtraEntry<T> {
    Expected(T),
    Actual(T),
    ContentMismatch(T),
}

pub fn tmp_path() -> PathBuf {
    PathBuf::from("tests")
        .join("tmp")
        .canonicalize()
        .expect("canon tmp path")
}

pub fn test_path(subfolder: &str) -> PathBuf {
    PathBuf::from("tests")
        .join("data")
        .join("acceptance")
        .join(subfolder)
        .canonicalize()
        .expect("canon test path")
}

pub fn tear_down<P: AsRef<Path>>(target_path: P) {
    std::fs::remove_dir_all(target_path).expect("tear_down - rm -rf");
}

pub fn compare_paths<PE: AsRef<Path>, PA: AsRef<Path>>(
    expected: PE,
    actual: PA,
) -> AcceptanceTestResult {
    let mut errors = Vec::new();

    let expected = expected.as_ref().canonicalize().expect("expected");
    let actual = actual.as_ref().canonicalize().expect("expected");

    if expected.is_file() && actual.is_file() {
        compare_file(expected, actual, &mut errors);
    } else if expected.is_dir() && actual.is_dir() {
        compare_dir_recursive(expected, actual, &mut errors);
    } else {
        errors.push(ExtraEntry::ContentMismatch(format!(
            "comparing {:?} to {:?}",
            expected, actual
        )))
    }

    AcceptanceTestResult {
        result: !errors.is_empty(),
        errors,
    }
}

fn compare_dir_recursive<PE: AsRef<Path>, PA: AsRef<Path>>(
    expected: PE,
    actual: PA,
    result: &mut Vec<ExtraEntry<String>>,
) {
    let expected = std::fs::read_dir(expected)
        .expect("read_dir expected")
        .collect::<Result<Vec<_>, _>>()
        .expect("collect expected");
    let actual = std::fs::read_dir(actual)
        .expect("read_dir actual")
        .collect::<Result<Vec<_>, _>>()
        .expect("collect actual");

    let (exp_dirs, exp_files) = expected
        .into_iter()
        .partition(|entry| entry.metadata().expect("exp entry metadata").is_dir());
    let (act_dirs, act_files) = actual
        .into_iter()
        .partition(|entry| entry.metadata().expect("act entry metadata").is_dir());

    compare_files(exp_files, act_files, result);
    compare_directories(exp_dirs, act_dirs, result);
}

fn compare_files(
    mut expected_files: Vec<DirEntry>,
    mut actual_files: Vec<DirEntry>,
    result: &mut Vec<ExtraEntry<String>>,
) {
    expected_files.sort_by(|l, r| l.path().cmp(&r.path()));
    actual_files.sort_by(|l, r| l.path().cmp(&r.path()));

    let mut exp_index = 0usize;
    let mut act_index = 0usize;

    loop {
        match (expected_files.get(exp_index), actual_files.get(act_index)) {
            (Some(exp_entry), Some(act_entry)) => {
                match exp_entry.file_name().cmp(&act_entry.file_name()) {
                    Ordering::Less => {
                        result.push(ExtraEntry::Expected(
                            exp_entry.path().to_string_lossy().to_string(),
                        ));
                        exp_index += 1;
                    }
                    Ordering::Equal => {
                        compare_file(exp_entry.path(), act_entry.path(), result);
                        exp_index += 1;
                        act_index += 1;
                    }
                    Ordering::Greater => {
                        result.push(ExtraEntry::Actual(
                            exp_entry.path().to_string_lossy().to_string(),
                        ));
                        act_index += 1;
                    }
                }
            }
            (Some(exp_entry), _) => {
                result.push(ExtraEntry::Expected(
                    exp_entry.path().to_string_lossy().to_string(),
                ));
                exp_index += 1;
            }
            (_, Some(act_entry)) => {
                result.push(ExtraEntry::Actual(
                    act_entry.path().to_string_lossy().to_string(),
                ));
                act_index += 1;
            }
            (None, None) => break,
        }
    }
}

fn compare_directories(
    mut expected_directory: Vec<DirEntry>,
    mut actual_directory: Vec<DirEntry>,
    result: &mut Vec<ExtraEntry<String>>,
) {
    expected_directory.sort_by(|l, r| l.path().cmp(&r.path()));
    actual_directory.sort_by(|l, r| l.path().cmp(&r.path()));

    let mut exp_index = 0usize;
    let mut act_index = 0usize;

    loop {
        match (
            expected_directory.get(exp_index),
            actual_directory.get(act_index),
        ) {
            (Some(exp_entry), Some(act_entry)) => {
                match exp_entry.file_name().cmp(&act_entry.file_name()) {
                    Ordering::Less => {
                        result.push(ExtraEntry::Expected(
                            exp_entry.path().to_string_lossy().to_string(),
                        ));
                        exp_index += 1;
                    }
                    Ordering::Equal => {
                        compare_dir_recursive(exp_entry.path(), act_entry.path(), result);
                        exp_index += 1;
                        act_index += 1;
                    }
                    Ordering::Greater => {
                        result.push(ExtraEntry::Actual(
                            exp_entry.path().to_string_lossy().to_string(),
                        ));
                        act_index += 1;
                    }
                }
            }
            (Some(exp_entry), _) => {
                result.push(ExtraEntry::Expected(
                    exp_entry.path().to_string_lossy().to_string(),
                ));
                exp_index += 1;
            }
            (_, Some(act_entry)) => {
                result.push(ExtraEntry::Actual(
                    act_entry.path().to_string_lossy().to_string(),
                ));
                act_index += 1;
            }
            (None, None) => break,
        }
    }
}

fn compare_file<PE: AsRef<Path>, PA: AsRef<Path>>(
    expected: PE,
    actual: PA,
    result: &mut Vec<ExtraEntry<String>>,
) {
    let e_hash = file_hash(expected.as_ref());
    let a_hash = file_hash(actual);

    if e_hash != a_hash {
        result.push(ExtraEntry::ContentMismatch(
            expected.as_ref().to_string_lossy().to_string(),
        ))
    }
}

fn file_hash<P: AsRef<Path>>(path: P) -> u64 {
    use twox_hash::XxHash;

    let mut hasher = XxHash::default();

    let mut file = File::open(path).expect("expected open");
    let mut buf = [0u8; 1024];

    loop {
        let len = file.read(&mut buf).expect("read");
        if len == 0 {
            break;
        }
        hasher.write(&buf[..len]);
    }

    hasher.finish()
}
