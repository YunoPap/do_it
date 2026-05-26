# Do_It! ⏳

`Do_It!` is a sleek, terminal-based, time-traveling task manager built in Rust using the **Ratatui** library. Unlike standard to-do apps, `Do_It!` is completely aware of time, allowing you to cycle through different dates, plan ahead, and automatically roll uncompleted tasks forward into the next day.

---

## ✨ Features

* **Time-Travel Architecture:** Use intuitive vim-like keys to cycle through past and future days to see your schedule layout dynamically.
* **Intelligent Rolling Over:** Incomplete tasks from yesterday don't disappear; they automatically roll over to meet you today. Future days remain a clean slate until you get there.
* **Modular Architecture:** Cleanly organized into isolated state and UI submodules for easy feature updates.
* **Persistent Storage:** Automatically saves your tasks to a local `todos.json` file on a clean exit, and reloads them instantly when booted up.

---

## ⌨️ Controls & Key Bindings

### Main Menu
| Key | Action |
| :--- | :--- |
| `1` | Start Program / Enter Task View |
| `4` | Exit Application |
| `Esc` | Close Application |

### Task Dashboard
| Key | Action |
| :--- | :--- |
| `j` / `k` | Navigate Up / Down through tasks |
| `h` / `l` | Time-Travel to Previous / Next Day |
| `A` | Add a new task (opens text input) |
| `D` | Delete the currently selected task |
| `Enter` | Toggle task completion (cross out) |
| `Esc` | Return to the Main Menu |

### Task Input Form
| Key | Action |
| :--- | :--- |
| `Char` | Type description characters |
| `Backspace`| Delete last character |
| `Enter` | Submit and create task |
| `Esc` | Cancel item creation |

---

## 🛠️ Project Structure

The project has been fully modularized to scale efficiently:

```text
src/
├── app.rs           # Core AppState, TodoItem definitions, and serialization modules
├── main.rs          # Main application loop, key event processors, and IO operations
└── ui/
    ├── mod.rs       # Central screen rendering router
    ├── main_menu.rs # Design and formatting for the Title Screen
    └── task_view.rs # Layout management, time headers, and custom date filtering