use std::io::{self, BufRead, Write};

// PID Namespace Simulator
// Each namespace gets its own PID counter starting at 1.
// Commands:
//   NEWNS  -> allocate next ns id, init empty process table, print id
//   FORK <ns> <name>  -> spawn process; assign in-ns pid; print pid
//   EXIT <ns> <pid>  -> mark exited; print OK
//   PS <ns>  -> print '<pid> <name> <state>' sorted by pid

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let mut buf: Vec<String> = Vec::new();
    // TODO: declare your state structures here
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.trim_end_matches('\n');
        if line.trim().is_empty() { continue; }
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "NEWNS" => { /* TODO: allocate next ns id, init empty process table, print id */ }
            "FORK" => { /* TODO: spawn process; assign in-ns pid; print pid */ }
            "EXIT" => { /* TODO: mark exited; print OK */ }
            "PS" => { /* TODO: print '<pid> <name> <state>' sorted by pid */ }
            _ => {}
        }
    }
    writeln!(out, "{}", buf.join("\n")).unwrap();
}
