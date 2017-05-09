extern crate difference;
extern crate term;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::BTreeSet;
use difference::Changeset;
use difference::Difference;
use std::process::Command;

fn read_file(file_name: &str) -> String {
    if file_name.starts_with("http://") {
        let tail = if file_name.ends_with(".gz") {
            " | zcat"
        } else {
            ""
        };
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("curl -s {}{}", file_name, tail))
            .output()
            .expect("failed to execute process");
        return String::from_utf8(output.stdout).unwrap();
    }
    if file_name.ends_with(".gz") {
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("zcat < {}", file_name))
            .output()
            .expect("failed to execute process");
        return String::from_utf8(output.stdout).unwrap();
    }
    let mut contents = String::new();
    let mut file = File::open(file_name).unwrap();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}

fn parse_packages(file_name: &str) -> HashMap<String, String> {
    let mut packages = HashMap::<String, String>::new();
    let contents = read_file(file_name);
    let mut str = String::new();
    let mut pkg = String::new();
    for line in contents.lines() {
        if line.starts_with("Package: ") {
            pkg = line[9..].to_string();
        } else if line == "" {
            packages.insert(pkg.to_string(), str);
            str = String::new();
        } else {
            str.push_str(line);
            str.push_str("\n");
        }
    }
    return packages;
}

fn has_changes(changeset: &Changeset) -> bool {
    for change in &changeset.diffs {
        match change {
            &Difference::Add(_) => {
                return false;
            }
            &Difference::Rem(_) => {
                return false;
            }
            &Difference::Same(_) => {}
        }
    }
    true
}

fn change_type(old: &String, new: &String) -> &'static str {
    if old.is_empty() {
        return &"Added";
    }
    if new.is_empty() {
        return &"Removed";
    }
    return &"Changed";
}

fn merge(a: &HashMap<String, String>, b: &HashMap<String, String>) -> Vec<String> {
    let va: Vec<String> = a.iter().map(|(user, _)| user.clone()).collect();
    let vb: Vec<String> = b.iter().map(|(user, _)| user.clone()).collect();
    let mut set: BTreeSet<_> = va.into_iter().collect();
    set.extend(vb);
    return set.iter().cloned().collect();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: apt-packages-diff [Packages.old] [Packages.gz]");
        return;
    }
    let old_packages = parse_packages(&args[1]);
    let new_packages = parse_packages(&args[2]);

    let package_names = merge(&old_packages, &new_packages);

    let mut t = term::stdout().unwrap();

    let empty = String::new();
    for kv in package_names {
        let old_text = old_packages.get(kv.as_str()).unwrap_or(&empty);
        let new_text = new_packages.get(kv.as_str()).unwrap_or(&empty);
        let changeset = Changeset::new(&old_text, &new_text, "\n");

        if !has_changes(&changeset) {
            let event_type = change_type(old_text, new_text).to_string();
            write!(t, "\n{} Package: {}", event_type, kv).unwrap();
            for change in changeset.diffs {
                match change {
                    Difference::Same(ref x) => {
                        t.reset().unwrap();
                        writeln!(t, "{}", x).unwrap();
                    }
                    Difference::Add(ref x) => {
                        t.fg(term::color::GREEN).unwrap();
                        writeln!(t, "{}", x).unwrap();
                        t.reset().unwrap();
                    }
                    Difference::Rem(ref x) => {
                        t.fg(term::color::RED).unwrap();
                        writeln!(t, "{}", x).unwrap();
                        t.reset().unwrap();
                    }
                }
            }
        }
    }
}
