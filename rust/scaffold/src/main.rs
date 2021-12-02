use std::env;
use std::fs;
use std::fs::File;
use std::io::ErrorKind::NotFound;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let day = env::args().nth(1).ok_or("expected day argument").unwrap().parse().unwrap();
    let path = find_root().join("rust").join(format!("d{:02}", day));
    with_path(&path, scaffold_cargo_project, path_is_new);
    with_path(&path.join("src").join("main.rs"), scaffold_main, file_contains(include_str!("templates/main.rs.default.tpl")));
    with_path(&path.join("src").join("testdata"), scaffold_testdata, path_is_new);
    with_path(&path.join("input.txt"), scaffold_input(2021, day), path_is_new);
}

fn find_root() -> PathBuf {
    let mut try_path = env::current_dir().unwrap();
    loop {
        match fs::metadata(try_path.join(".git")) {
            Err(e) if e.kind() == NotFound => {}
            Err(e) => { panic!("{}", e) }
            Ok(_) => { break; }
        }
        try_path = try_path.parent().unwrap().to_path_buf()
    }
    try_path
}

fn with_path(path: &PathBuf, f: impl Fn(&PathBuf), reason: impl Fn(&PathBuf) -> bool) {
    if reason(path) {
        f(path)
    }
}

fn path_is_new(path: &PathBuf) -> bool {
    match fs::metadata(path) {
        Err(e) if e.kind() == NotFound => { return true; }
        Err(e) => { panic!("{}", e) }
        _ => { return false; }
    };
}

fn file_contains<'a>(substr: &'a str) -> Box<dyn Fn(&PathBuf) -> bool + 'a> {
    Box::new(move |path| {
        let contents = match fs::read_to_string(path) {
            Err(e) if e.kind() == NotFound => { String::new() }
            Err(e) => { panic!("{}", e) }
            Ok(c) => { c }
        };
        return contents.contains(substr);
    })
}

fn scaffold_cargo_project(path: &PathBuf) {
    Command::new("cargo")
        .args(["new", path.file_name().unwrap().to_str().unwrap()])
        .current_dir(path.parent().unwrap())
        .output().unwrap();
}

fn scaffold_input(year: u32, day: u32) -> Box<dyn Fn(&PathBuf)> {
    Box::new(move |path| {
        let input = fetch_input(year, day);
        let mut file = File::create(path).unwrap();
        file.write_all(input.as_bytes()).unwrap();
    })
}

fn scaffold_main(path: &PathBuf) {
    fs::write(path, include_bytes!("templates/main.rs.tpl")).unwrap();
}

fn scaffold_testdata(path: &PathBuf) {
    fs::create_dir(path).unwrap();
    fs::write(path.join("basic.txt"), "").unwrap();
}

fn fetch_input(year: u32, day: u32) -> String {
    let session = fs::read_to_string(find_root().join("session.txt")).unwrap();
    let session = session.trim();
    let res = minreq::get(format!("https://adventofcode.com/{}/day/{}/input", year, day))
        .with_header("cookie", format!("session={}", session))
        .send().unwrap();
    if res.status_code != 200 {
        panic!("Could not fetch puzzle input: {}", res.reason_phrase)
    }
    String::from(res.as_str().unwrap())
}
