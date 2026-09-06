use std::{fmt, io::{self, BufRead, Write}, println};

// PID Namespace Simulator
// Each namespace gets its own PID counter starting at 1.
// Commands:
//   NEWNS  -> allocate next ns id, init empty process table, print id
//   FORK <ns> <name>  -> spawn process; assign in-ns pid; print pid
//   EXIT <ns> <pid>  -> mark exited; print OK
//   PS <ns>  -> print '<pid> <name> <state>' sorted by pid

struct PidNamespace {
    id: i32,
    pids: Vec<Pid>,
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

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let mut buf: Vec<String> = Vec::new();
    
    let mut ns: Vec<PidNamespace> = vec![];

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.trim_end_matches('\n');
        if line.trim().is_empty() { continue; }
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "NEWNS" => { 
                let ns_id = ns.len() as i32 + 1;
                ns.push(PidNamespace { id: ns_id, pids: vec![] });
                println!("{}", ns_id);
             }
            "FORK" => { 
                let ns_id = parts[1].parse::<i32>().unwrap();
                let name = parts[2];

                let ns = ns.get_mut(ns_id as usize - 1).unwrap();

                let pid = ns.pids.len() as i32 + 1;

                ns.pids.push(Pid { id: pid, state: PidState::Running, name: name.to_string() });
                println!("{}", pid);
             }
            "EXIT" => { 
                let ns_id = parts[1].parse::<i32>().unwrap();
                let pid = parts[2].parse::<i32>().unwrap();

                let ns = ns.get_mut(ns_id as usize - 1).unwrap();
                let mut p = ns.pids.get_mut(pid as usize - 1).unwrap();
                p.state = PidState::Exited;
                println!("OK");
             }
            "PS" => {
                let ns_id = parts[1].parse::<i32>().unwrap();
                let ns = ns.get(ns_id as usize - 1).unwrap();
                let mut pids = ns.pids.iter().collect::<Vec<&Pid>>();
                pids.sort_by_key(|p| p.id);

                for p in &pids {
                    println!("{} {} {}", p.id, p.name, p.state);
                }
             }
            _ => {}
        }
    }
    writeln!(out, "{}", buf.join("\n")).unwrap();
}
