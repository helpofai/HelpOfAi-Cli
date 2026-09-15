i try build own codebase-memory-mcp, i want use (https://github.com/DeusData/codebase-memory-mcp/) code and rebrandind my (helpofai) carefully chech DeusData/codebase-memory-mcp: High-performance code intelligence MCP server. Indexes codebases into a persistent knowledge graph — average repo in milliseconds. 158 languages, sub-ms queries, 99% fewer tokens. Single static binary, zero dependencies. this main feature (like codebase-memory-mcp) use or build in my project own global codebase-memory-mcp

The important detail is that the graph UI is embedded in the UI-enabled codebase-memory-mcp binary and is served on localhost:9749; the same process can provide the MCP server and the graph UI.

For HelpOfAi-Cli, I recommend making helpofai graph a supervisor command that starts the upstream binary as a separate child process, rather than putting the graph server inside your Rust TUI.

Target architecture
HelpOfAi-Cli
│
│
helpofai graph
│
▼
┌──────────────────────┐
│ Rust Graph Supervisor│
└──────────┬───────────┘
│
spawn child process
│
▼
┌─────────────────────────────┐
│ codebase-memory-mcp │
│ --ui=true │
│ --port=9749 │
└──────────────┬──────────────┘
│
localhost:9749
│
▼
┌─────────────────┐
│ Browser / 3D UI │
│ │
│ ●──●──● │
│ / \ │
│● ● │
└─────────────────┘

The upstream documentation currently shows:

codebase-memory-mcp --ui=true --port=9749

and then opening:

http://localhost:9749

The UI is served by the binary itself.

1. Don't copy the graph UI into your TUI

This is important.

Don't do this:

HelpOfAi TUI
└── embedded browser
└── 3D graph

You would introduce a browser/WebView dependency and make your terminal application heavier.

Instead:

HelpOfAi TUI
│
│ process manager
▼
codebase-memory-mcp
│
▼
Browser

This is cleaner and matches your low-resource goal.

2. Put the upstream binary in a dedicated directory

For HelpOfAi, I would use something like:

Windows
%LOCALAPPDATA%\HelpOfAi\
└── engines\
 └── codebase-memory\
 └── codebase-memory-mcp.exe
Linux
~/.local/share/helpofai/
└── engines/
└── codebase-memory/
└── codebase-memory-mcp
macOS
~/Library/Application Support/HelpOfAi/
└── engines/
└── codebase-memory/
└── codebase-memory-mcp

Don't put the binary inside the user's project.

3. Add a Rust module

Inside your HelpOfAi-Cli source, create:

src/
├── commands/
│ ├── mod.rs
│ └── graph.rs
│
├── engines/
│ ├── mod.rs
│ └── codebase_memory.rs
│
└── main.rs

I'd separate command handling from engine lifecycle management.

4. Create CodebaseMemoryManager

Conceptually:

pub struct CodebaseMemoryManager {
binary_path: PathBuf,
port: u16,
child: Option<Child>,
}

Its responsibility should be:

find binary
↓
verify binary
↓
check port
↓
start process
↓
wait for HTTP server
↓
open browser
↓
monitor process
↓
stop/restart 5. Locate the binary

Something like:

fn codebase_memory_binary() -> Result<PathBuf> {
let root = helpofai_data_dir()?;

    let binary = if cfg!(target_os = "windows") {
        root.join("engines")
            .join("codebase-memory")
            .join("codebase-memory-mcp.exe")
    } else {
        root.join("engines")
            .join("codebase-memory")
            .join("codebase-memory-mcp")
    };

    if !binary.exists() {
        anyhow::bail!(
            "Codebase Memory engine is not installed: {}",
            binary.display()
        );
    }

    Ok(binary)

} 6. Start the upstream process

This is the core part.

use std::process::{Child, Command, Stdio};

fn start_graph_server(
binary: &Path,
port: u16,
) -> anyhow::Result<Child> {
let child = Command::new(binary)
.arg("--ui=true")
.arg("--port")
.arg(port.to_string())
.stdin(Stdio::null())
.stdout(Stdio::inherit())
.stderr(Stdio::inherit())
.spawn()?;

    Ok(child)

}

That means HelpOfAi executes:

codebase-memory-mcp --ui=true --port 9749

as a separate OS process.

Exactly what you want.

7. Wait until the UI is actually ready

Don't immediately launch the browser after .spawn().

The process might still be starting.

Use a readiness check:

async fn wait_for_graph_server(port: u16) -> anyhow::Result<()> {
let url = format!("http://127.0.0.1:{port}/");

    for _ in 0..50 {
        if reqwest::get(&url).await.is_ok() {
            return Ok(());
        }

        tokio::time::sleep(
            std::time::Duration::from_millis(200)
        ).await;
    }

    anyhow::bail!(
        "Codebase Memory graph server did not become ready"
    );

}

So the sequence becomes:

spawn
↓
wait
↓
HTTP available?
├── no → retry
└── yes
↓
open browser 8. Open the browser

After the server is ready:

open::that(format!("http://127.0.0.1:{port}"))?;

On Windows this will open the user's default browser.

So:

helpofai graph

produces:

Starting HelpOfAi Codebase Memory...

Codebase Memory Graph
Server: http://127.0.0.1:9749
Status: ready

Opening browser...

Then the user sees the 3D graph.

9. Add the CLI command

If your CLI uses clap, add something conceptually like:

#[derive(Subcommand)]
enum Commands {
Graph { #[arg(long, default_value_t = 9749)]
port: u16,

        #[arg(long)]
        no_open: bool,
    },

}

Then:

helpofai graph

or:

helpofai graph --port 9750

or:

helpofai graph --no-open 10. Don't terminate it immediately

This is a common mistake.

If you do:

let mut child = start_graph_server(...)?;
open_browser()?;
Ok(())

your command may finish while you still want the graph server running.

Instead, decide on the lifecycle you want.

I recommend:

helpofai graph

starts the graph server as a detached/background service.

Then:

helpofai graph stop

stops it.

And:

helpofai graph status

shows:

Codebase Memory Graph

Status: running
PID: 18244
Host: 127.0.0.1
Port: 9749
URL: http://127.0.0.1:9749 11. Better command structure

I'd make:

helpofai graph
helpofai graph start
helpofai graph stop
helpofai graph restart
helpofai graph status
helpofai graph open

For example:

helpofai graph start

→ start process

helpofai graph open

→ open existing graph

helpofai graph status

→ check process

helpofai graph stop

→ terminate it

12. Store PID information

Create:

~/.helpofai/
└── runtime/
└── codebase-memory-graph.json

For example:

{
"pid": 18244,
"port": 9749,
"host": "127.0.0.1",
"started_at": "2026-09-14T21:45:00+05:30"
}

Don't blindly trust the PID file.

When running:

helpofai graph status

verify:

PID exists
process is actually the Codebase Memory binary
port responds 13. Handle port conflicts

Suppose:

9749

is already occupied.

Don't simply crash.

You can do:

9749 occupied
↓
try 9750
↓
try 9751
↓
...

or allow:

helpofai graph --port 9800

I'd make the default:

127.0.0.1:9749

because that's the upstream default documented for the graph UI.

14. The important part: project selection

Your command should eventually support:

helpofai graph

meaning:

current project

and:

helpofai graph --project C:\Projects\MyApp

meaning:

specific project

You should pass the project context through whatever integration mechanism the upstream engine expects rather than inventing a new indexing mechanism around the UI.

The upstream system has automatic indexing/watch behavior and persistent graph storage, so HelpOfAi should primarily orchestrate it, not duplicate its indexing implementation.

15. Make it a HelpOfAi engine

This is where I would go beyond simply wrapping the binary.

Create:

HelpOfAi
│
├── commands/
│ └── graph
│
├── engines/
│ └── codebase-memory/
│ ├── manager.rs
│ ├── installer.rs
│ ├── lifecycle.rs
│ ├── health.rs
│ ├── config.rs
│ └── platform.rs
│
└── integrations/
└── codebase-memory/
├── mcp.rs
└── graph.rs

Then you have:

CodebaseMemoryManager
│
├── install()
├── update()
├── start()
├── stop()
├── restart()
├── status()
├── open_ui()
└── health_check() 16. Make installation automatic

Eventually:

helpofai setup

could install:

HelpOfAi
│
├── Core
├── Skills
├── MCP
├── Codebase Memory
└── other engines

But do not download the upstream binary every time.

Use:

~/.helpofai/engines/

and version it:

engines/
└── codebase-memory/
├── 0.10.x/
│ └── codebase-memory-mcp(.exe)
│
└── current -> 0.10.x

This also makes rollback possible.

17. Your final user experience

This is what I would aim for.

Terminal 1
helpofai
╭──────────────────────────────────────────────╮
│ HelpOfAi AIOS │
│ │
│ Project: HelpOfAi-Cli │
│ │
│ Agent: Analyzing codebase... │
│ │
│ Codebase Memory: ● Connected │
│ Graph: http://127.0.0.1:9749 │
╰──────────────────────────────────────────────╯
Terminal 2
helpofai graph
HelpOfAi Codebase Graph

✓ Codebase Memory engine found
✓ Graph server started
✓ Port 9749 available
✓ Graph UI ready

Opening:
http://127.0.0.1:9749
Browser
3D CODEBASE GRAPH

             ● UserService
            /       \
           /         \
          ●           ●
    UserModel      AuthService
       |               |
       ●               ●

Database Middleware

And the agent continues running independently.

18. One thing I would change from the upstream command

The upstream documentation currently says the UI can run alongside the MCP server.

For HelpOfAi, I'd introduce a lifecycle abstraction:

HelpOfAi
│
├── MCP session
│
└── Graph UI
│
└── same Codebase Memory engine

Eventually you could have:

helpofai graph start

without starting a normal agent session at all.

That's useful because the user might simply want to explore their codebase visually.

One caveat before you implement it

The upstream project has had some recent platform/release issues around UI binaries and installers. For example, there are recent reports of UI-enabled Windows/macOS installations ending up with a binary that says it was built without the embedded UI, and a recent Windows release issue around archive layout.

So don't make your HelpOfAi integration assume that every downloaded codebase-memory-mcp binary has the UI embedded.

Your installer should verify:

binary installed
↓
run --ui=true --port=<test-port>
↓
HTTP responds
↓
✓ UI-capable

If not:

✗ UI unavailable in this binary

Please install the UI-enabled Codebase Memory engine.

That makes HelpOfAi robust instead of inheriting an upstream packaging problem.

The architecture I recommend for your repository
HelpOfAi-Cli
│
├── Core CLI/TUI
│
├── AIOS Runtime
│
├── Engines
│ ├── Analysis Engine
│ ├── Planning Engine
│ ├── Implementation Engine
│ ├── Verification Engine
│ │
│ └── Codebase Memory Engine ◄── upstream binary
│ │
│ ├── Process Manager
│ ├── Health Manager
│ ├── Version Manager
│ ├── MCP Integration
│ └── Graph UI Launcher
│
└── Commands
└── graph
├── start
├── stop
├── restart
├── status
└── open

This is the approach I'd use rather than embedding the 3D frontend into HelpOfAi itself. It preserves the upstream graph implementation, keeps your Rust TUI lightweight, lets the browser handle the 3D rendering, and gives HelpOfAi complete lifecycle control.
