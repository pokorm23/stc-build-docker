use std::{fmt, io::{self, BufRead, Write}, println};

struct PidNamespace {
    id: i32,
    pids: Vec<Pid>,
    mounts: Vec<Mount>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum PidState {
    Running,
    Exited
}

impl fmt::Display for PidState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            PidState::Running => "running",
            PidState::Exited => "exited",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Pid {
    id: i32,
    name: String,
    state: PidState,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Mount {
    src: String,
    dst: String,
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let buf: Vec<String> = Vec::new();
    
    let mut ns: Vec<PidNamespace> = vec![];

    ns.push(PidNamespace { id: 0, pids: vec![], mounts: vec![] });

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.trim_end_matches('\n');
        if line.trim().is_empty() { continue; }
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "NEWNS" => { 
                let ns_id = ns.len() as i32;
                let ns_root = ns.get(0).unwrap().mounts.clone();
                ns.push(PidNamespace { id: ns_id, pids: vec![], mounts: ns_root });
                println!("{}", ns_id);
             }
            "FORK" => { 
                let ns_id = parts[1].parse::<i32>().unwrap();
                let name = parts[2];

                let ns = ns.get_mut(ns_id as usize).unwrap();

                let pid = ns.pids.len() as i32 + 1;

                ns.pids.push(Pid { id: pid, state: PidState::Running, name: name.to_string() });
                println!("{}", pid);
             }
            "EXIT" => { 
                let ns_id = parts[1].parse::<i32>().unwrap();
                let pid = parts[2].parse::<i32>().unwrap();

                let ns = ns.get_mut(ns_id as usize).unwrap();
                let mut p = ns.pids.get_mut(pid as usize - 1).unwrap();
                p.state = PidState::Exited;
                println!("OK");
             }
            "PS" => {
                let ns_id = parts[1].parse::<i32>().unwrap();
                let ns = ns.get(ns_id as usize).unwrap();
                let mut pids = ns.pids.iter().collect::<Vec<&Pid>>();
                pids.sort_by_key(|p| p.id);

                for p in &pids {
                    println!("{} {} {}", p.id, p.name, p.state);
                }
             },
             "MOUNT" => {
                let ns_id = parts[1].parse::<i32>().unwrap();
                let ns = ns.get_mut(ns_id as usize).unwrap();
                let src = parts[2];
                let dst = parts[3];

                ns.mounts.retain(|x| x.dst != dst);
                ns.mounts.push(Mount { src: src.to_string(), dst: dst.to_string() });

                println!("OK");
             },
             "UMOUNT" => {
                let ns_id = parts[1].parse::<i32>().unwrap();
                let ns = ns.get_mut(ns_id as usize).unwrap();
                let dst = parts[2];

                ns.mounts.retain(|x| x.dst != dst);

                println!("OK");
             },
             "LISTMOUNTS" => {
                let ns_id = parts[1].parse::<i32>().unwrap();
                let ns = ns.get(ns_id as usize).unwrap();

                let mut mounts = ns.mounts.iter().map(|x| x.clone()).collect::<Vec<Mount>>();

                mounts.sort_by_key(|p| p.dst.to_string());
                mounts.dedup_by(|a, b| a.dst == b.dst);

                if (mounts.len() == 0) {
                    println!("(empty)");
                }

                for m in &mounts {
                    println!("{} on {}", m.src, m.dst);
                }
             },
            _ => {}
        }
    }
    writeln!(out, "{}", buf.join("\n")).unwrap();
}
