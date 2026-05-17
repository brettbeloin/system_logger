# System Metrics Logger - Rust Training Project

A CLI tool that collects and logs system metrics (CPU, memory) at regular intervals. This project teaches core Rust concepts needed for systems programming.

## Project Goals

Build skills in:
- Ownership and borrowing patterns
- Struct design and serialization
- File I/O and error handling
- CLI argument parsing
- Async programming with Tokio
- Graceful shutdown handling

## Steps to Complete

### Step 1: Basic Collection
**Goal:** Print CPU and memory usage to stdout every 2 seconds.

**What to learn:**
- Using the `sysinfo` crate
- Calling `System::new_all()` and `refresh_all()`
- Accessing CPU and memory data
- Using `std::thread::sleep` for intervals

**Tips:**
- `System` requires `&mut self` for `refresh_all()` - practice mutable borrows
- `global_cpu_usage()` returns current CPU percentage
- `used_memory()` and `total_memory()` for memory stats

### Step 2: Structure the Data
**Goal:** Create a `Metrics` struct to hold collected data.

**What to learn:**
- Defining structs with proper types
- Adding `#[derive(Serialize)]` for JSON support
- Working with `chrono` or `std::time` for timestamps

**Requirements:**
- Fields: `timestamp`, `cpu_percent`, `memory_used_mb`, `memory_total_mb`
- Derive `Serialize` from serde
- Create a function that returns `Metrics`

### Step 3: Write to File
**Goal:** Append metrics as JSON lines to `metrics.log`.

**What to learn:**
- Opening files with `OpenOptions` (append mode)
- Using `serde_json::to_string()` to serialize
- Writing to files with proper error handling
- Understanding `Result<T, E>` and `?` operator

**Format:** One JSON object per line (newline-delimited JSON)
```json
{"timestamp":"2026-05-16T10:30:00","cpu_percent":45.2,"memory_used_mb":8192,"memory_total_mb":16384}
{"timestamp":"2026-05-16T10:30:02","cpu_percent":42.1,"memory_used_mb":8200,"memory_total_mb":16384}
```

### Step 4: CLI Arguments
**Goal:** Accept `--interval` and `--output` flags.

**What to learn:**
- Using `clap` with derive macros
- Defining CLI struct with `#[derive(Parser)]`
- Accessing parsed arguments
- Default values for optional args

**Example usage:**
```bash
cargo run -- --interval 5 --output /tmp/metrics.log
cargo run -- -i 1 -o data.json
```

### Step 5: Graceful Shutdown
**Goal:** Handle Ctrl+C, flush file, print summary before exit.

**What to learn:**
- Using `ctrlc` crate or `tokio::signal`
- Channel-based shutdown signaling
- Ensuring resources are cleaned up
- Printing final statistics (total samples collected)

**Behavior:**
- User presses Ctrl+C
- Current write completes
- File is flushed/closed
- Summary printed: "Collected 47 samples over 94 seconds"

### Step 6: Async Version
**Goal:** Convert to async with Tokio for non-blocking collection.

**What to learn:**
- `#[tokio::main]` attribute
- `tokio::time::interval()` vs `thread::sleep()`
- Async file I/O with `tokio::fs` or keep sync writes
- Spawning tasks with `tokio::spawn()`

**Why async:**
- Prepares you for concurrent tasks in capstone
- Non-blocking intervals
- Better resource utilization
- Foundation for multi-threaded metrics collection

## Dependencies

Add to `Cargo.toml`:
```toml
[dependencies]
sysinfo = "0.33"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
clap = { version = "4.5", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
tokio = { version = "1.43", features = ["full"] }
ctrlc = "3.4"
```

## Testing Your Work

After each step:
1. Run with `cargo run`
2. Verify output matches expectations
3. Check for compiler warnings (`cargo clippy`)
4. Format code (`cargo fmt`)

**For file output:**
```bash
# Run for 10 seconds then Ctrl+C
cargo run -- --interval 1 --output test.log

# Verify JSON format
cat test.log | jq .

# Count lines
wc -l test.log
```

## Common Issues & Solutions

**"cannot borrow as mutable"**
- `System` needs `&mut` for refresh - make sure it's declared `mut`

**"moved value used after move"**
- Metrics struct was moved - either `.clone()` or restructure ownership

**File not flushing**
- Wrap `File` in `BufWriter` and call `.flush()` on shutdown

**Async confusion**
- Start with sync version (Steps 1-5) first
- Convert to async in Step 6 after sync works

## Time Estimates

- Step 1: 30-45 min
- Step 2: 20-30 min
- Step 3: 45-60 min (most error handling here)
- Step 4: 30 min
- Step 5: 45-60 min
- Step 6: 60-90 min (learning async)

**Total:** 4-6 hours for beginners, 2-3 if comfortable with concepts

## Success Criteria

You're done when:
- ✓ Compiles with no warnings
- ✓ Runs async with configurable interval
- ✓ Writes valid JSON to specified file
- ✓ Handles Ctrl+C gracefully
- ✓ You understand every line of code you wrote

## Next Steps

After completing this:
1. Add more metrics (disk I/O, network traffic, process count)
2. Implement basic anomaly detection (alert if CPU > 80% for 30s)
3. Read the capstone project planning notes
4. Start architecture design for the full monitoring system

## Resources

- [Sysinfo crate docs](https://docs.rs/sysinfo/latest/sysinfo/)
- [Clap derive tutorial](https://docs.rs/clap/latest/clap/_derive/index.html)
- [Tokio tutorial](https://tokio.rs/tokio/tutorial)
- [Rust book - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
