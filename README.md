# 🐍 RustSnakeTogether

A snake game that lives entirely in your terminal, written in Rust.

> This is my pet project. I'm building it to understand Rust better and to
> scratch my love for game dev. Expect rough edges, half-finished corners and
> the occasional `todo!()` — it's a playground, and it grows as I learn.

---

## What it looks like

Everything is drawn with box-drawing characters and a few symbols, so it runs
in any reasonably modern terminal (at least **80×24**).

```text
   ███████╗███╗   ██╗ █████╗ ██╗  ██╗███████╗  
   ██╔════╝████╗  ██║██╔══██╗██║ ██╔╝██╔════╝     
   ███████╗██╔██╗ ██║███████║█████╔╝ █████╗
   ╚════██║██║╚██╗██║██╔══██║██╔═██╗ ██╔══╝
   ███████║██║ ╚████║██║  ██║██║  ██╗███████╗
   ╚══════╝╚═╝  ╚═══╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝

              ┌──────────────────┐
              │                  │
              │     >Start<      │
              │   Create room    │
              │     Options      │
              │       Exit       │
              │                  │
              └──────────────────┘
   ┌Log───────────────────────────────────────┐
   │                                          │
   └──────────────────────────────────────────┘
```

**In game** – walls join up into proper corners, the snake is green, food is `✦`:

```text
┌──────────┐
│┌────────┐│
││        ││
││ ┌────┐ ││
││ │    │ ││
││        ││
││ ────◉  ││      ◉ = head
││ │    │ ││      ✦ = food
││ │ ── │ ││
││      ✦ ││
│└────────┘│
└──────────┘
```

---

## How to play

```bash
cargo run
```

| Key         | Does                                   |
|-------------|----------------------------------------|
| Arrow keys  | Move in menus / steer the snake        |
| `Enter`     | Select                                 |
| `Backspace` | Go back one screen                     |

> Not done yet: **Create room** and **Options** in the main menu

---

## Configuration

On first launch a default config is created at
`resources/config.json` **next to the executable** 

```json
{
  "input_config": { "up": "Up", "down": "Down", "left": "Left",
                    "right": "Right", "back": "Backspace", "enter": "Enter" },
  "terminal_min_size": { "width": 80, "height": 24 },
  "difficulty": {
    "easy":     { "tick_ms": 1000, "food_count": 1 },
    "advanced": { "tick_ms": 500,  "food_count": 1 },
    "hard":     { "tick_ms": 200,  "food_count": 1 }
  }
}
```

- `tick_ms` – milliseconds between snake moves (lower = faster = harder).
- `food_count` – how many food items are always kept on the map.
- Delete the file to get the defaults back.

---

## How it's put together (the short version)

The whole game is a tiny **state machine**. Every screen is a "state", and
`main` just keeps asking the current one to run a frame (~60 times a second).
A state does its thing and either stays put or hands back the next state.


```text
src/
├── main.rs        loads the config, runs the frame loop
├── states/        one file per screen (main menu, level/difficulty select, game, exit)
├── ui/            drawing: menus, level preview, logo animation, terminal layout
├── model/         game objects (snake, food, walls), level layouts, errors
├── config/        reads / creates / writes the JSON config
├── core/          background task that listens for key presses
└── utils.rs       shared helpers (menu navigation, joining walls into ┌─┐ shapes)
```

Built with [`ratatui`](https://ratatui.rs) + [`crossterm`](https://github.com/crossterm-rs/crossterm)
for the terminal UI and [`tokio`](https://tokio.rs) for the loop and input.

---

## Todo list

- Finish **Options**
- More levels, a visible score 
- Better ui elements. Maybe will require me to get a Designer degree.
- **Create room** when multiplayer support is finished and make the "Together" in the logo mean something 😉

## Ideas/maybe later

- Music support
- Different languages support
- Add support for music bpm act as tick_ms in game (Lets make it a rhythm game, why not)