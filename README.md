# Snake in Rust 🐍

A classic Snake game that runs right in your terminal, written in Rust.

You steer a snake around a box, eat food (`*`) to grow longer and score points, and try not to hit the walls or yourself. The game gets a little faster every time you eat.

```
SNAKE  |  Score: 30  |  Length: 6
Arrows / WASD to move  |  P pause  |  R restart  |  Q quit
+------------------------------+
|                              |
|     oooO                     |
|              *               |
|                              |
+------------------------------+
```

No graphics card, no extra setup — just your terminal.

---

## How to Play

| Key | What it does |
|-----|--------------|
| **Arrow keys** ↑ ↓ ← → | Move the snake |
| **W A S D** (or **H J K L**) | Move the snake — same as arrows |
| **P** | Pause / resume |
| **R** | Restart after game over (or press Space / Enter) |
| **Q** or **Esc** | Quit the game |

- Eat `*` → +10 points, snake grows by 1, game speeds up a little
- Hit a wall or bite yourself → game over
- Your snake is `O` (head) + `o` (body), food is `*`

---

## Run It — Complete Beginner Guide

You do **not** need to know Rust to play this. You just need to install Rust once, download this code, and run one command. This guide walks through every step.

### What is Rust / Cargo?

- **Rust** is the programming language this game is written in.
- **Cargo** is Rust's helper tool. It downloads the libraries the game needs and builds/runs it. Cargo comes automatically when you install Rust.
- **Terminal** is the text-based app where you type commands. See below if you're not sure how to open it.

### Step 1 — Open a Terminal

- **Mac:** Press `Cmd + Space`, type `Terminal`, press Enter. Or open `Applications → Utilities → Terminal`.
- **Windows:** Press `Windows` key, type `PowerShell` or `Terminal`, press Enter. Or right-click Start → Terminal.
- **Linux:** Press `Ctrl + Alt + T`, or search for `Terminal` in your apps.

You should see a window with a prompt like `$` or `>` where you can type.

### Step 2 — Install Rust (if you don't have it yet)

Check first — type this in your terminal and press Enter:

```bash
cargo --version
```

- If you see something like `cargo 1.99.0`, you already have Rust. Skip to Step 3.
- If you see `command not found` / `not recognized`, install Rust:

**On Mac or Linux:**

Copy-paste this whole command into your terminal and press Enter:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

When it asks, press Enter to choose the default installation (option `1`).

After it finishes, **close your terminal and open a new one** — this is important, otherwise your terminal won't find `cargo` yet. Then check again:

```bash
cargo --version
```

If it still says not found, run this once in the new terminal:

```bash
source "$HOME/.cargo/env"
cargo --version
```

**On Windows:**

1. Go to https://rustup.rs/ in your browser
2. Download and run `rustup-init.exe`
3. Choose the default install (press Enter)
4. **Restart your terminal** (close it and open a new one)
5. Check with `cargo --version`

> **What is rustup?** It's the official Rust installer. It installs both Rust and Cargo for you. You only need to do this once per computer.

### Step 3 — Get the Code

If you have Git installed, clone this repository:

```bash
git clone https://github.com/y238wang/snake-rust.git
cd snake-rust
```

**No Git?** On GitHub, click the green **<> Code** button → **Download ZIP** → unzip it → open a terminal inside the unzipped `snake-rust` folder.

> Not sure if you have Git? Type `git --version`. If it says not found, you can use the Download ZIP method above, or install Git from https://git-scm.com/downloads

### Step 4 — Run the Game

From inside the `snake-rust` folder, run:

```bash
cargo run
```

That's it.

The first time will take 30–60 seconds because Cargo downloads the libraries the game uses (`crossterm` for terminal drawing, `rand` for placing food). After that, it starts instantly.

The game opens in your terminal. Use the arrow keys or WASD to play. Press `Q` to quit.

### Other Useful Commands

**Run a faster, optimized build** (recommended if the game feels laggy):

```bash
cargo run --release
```

**Just build it without running**, so you get a standalone executable:

```bash
cargo build --release
```

After that, the game is a single file you can run directly:

- Mac / Linux: `./target/release/snake-rust`
- Windows: `.\target\release\snake-rust.exe`

You can copy that file anywhere and double-click / run it — no Rust needed on that computer after building.

**Check that the code is healthy** (developers):

```bash
cargo check
```

---

## Troubleshooting

### `cargo: command not found` / `cargo is not recognized`

Rust is installed but your terminal doesn't know where it is yet.

- Close your terminal completely and open a brand-new one, then try `cargo --version` again.
- Mac/Linux fallback: run `source "$HOME/.cargo/env"` then try again.
- Windows fallback: restart your computer, then open a new Terminal and try again.

### `cargo run` is stuck / downloading for a long time

First run downloads dependencies — that's normal. Give it a minute. If your internet is slow, wait a bit longer. If it fails, check your internet and run `cargo run` again.

### The game looks squished / broken / tiny

Make your terminal window bigger, or zoom out (Mac: `Cmd + -`, Windows/Linux: `Ctrl + -`). The game needs at least ~35 columns wide and ~28 lines tall.

### Arrow keys don't work / letters appear instead

Make sure you clicked into the terminal window first so it has focus. If you're using an unusual terminal, try WASD keys instead of arrows.

### `git: command not found`

You don't need Git to play — use the **Download ZIP** method in Step 3 above.

### I pressed Q but the screen looks weird

The game restores your terminal when it quits. If something looks off, just press Enter, or type `reset` and press Enter (Mac/Linux).

### Windows: "Microsoft Visual C++ Build Tools required"

Rust on Windows sometimes asks for C++ build tools. If you see that error:
1. Install [Visual Studio Community](https://visualstudio.microsoft.com/vs/community/) (free)
2. During install, check **"Desktop development with C++"**
3. Restart your terminal and try `cargo run` again

---

## Project Structure

```
snake-rust/
├── Cargo.toml        # Project settings + dependencies (crossterm, rand)
├── Cargo.lock        # Exact versions of dependencies (auto-generated)
├── README.md         # This file
└── src/
    └── main.rs       # The whole game — board, snake, input, drawing
```

- `Cargo.toml` tells Cargo the game is called `snake-rust` and which libraries it needs.
- `src/main.rs` is where all the game logic lives. Open it if you want to see how it works — it's commented for beginners.

## Want to Tweak It?

Open `src/main.rs` and try changing these at the top:

- `WIDTH` / `HEIGHT` — size of the board
- `180` in `tick_duration()` — starting speed (lower = faster)
- `+10` in `tick()` — points per food

Then run `cargo run` again to see your changes.

## License

MIT — do whatever you want with it.
