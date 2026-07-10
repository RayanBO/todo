# === todo — CLI demo ===
# Requires VHS: https://github.com/charmbracelet/vhs
# Run: vhs demo.tape

Output demo.gif

# --- Look & feel (modern terminal chrome) ---
Set Shell "cmd"
Set FontFamily "Consolas"
Set FontSize 16
Set Width 1200
Set Height 700
Set Theme "Catppuccin Mocha"
Set Padding 20
Set WindowBar Colorful
Set WindowBarSize 40
Set BorderRadius 12
Set Margin 40
Set MarginFill "#11111b"
Set TypingSpeed 55ms

# --- Hidden setup (not shown in the GIF) ---
Hide
Type `cd /d %TEMP% && rmdir /s /q todo-demo & mkdir todo-demo && cd todo-demo && cls`
Enter
Show

# --- Init ---
Type `todo init`
Sleep 300ms
Enter
Sleep 1.2s

# --- First task: quick, punchy ---
Type `todo add --task "Add simple a task" --tag test --priority medium`
Sleep 250ms
Enter
Sleep 1.5s

# --- Add an actor (slower, it's a new concept for the viewer) ---
Type `todo add --actor "Alice"`
Sleep 300ms
Enter
Sleep 1.8s

# --- Second task, longer pause before to let the GIF breathe ---
Sleep 800ms
Type `todo add --task "Fix login redirect bug" --tag auth --priority medium --due "2026-07-20 18:00"`
Sleep 250ms
Enter
Sleep 1.5s

# --- Show the list, let it sit on screen ---
Sleep 700ms
Type `todo list --tasks`
Sleep 300ms
Enter
Sleep 3s

# --- Filter by tag: shows there's real functionality, not just a list ---
Type `todo list --tag auth`
Sleep 300ms
Enter
Sleep 2.5s

# --- Search ---
Type `todo search "simple"`
Sleep 300ms
Enter
Sleep 2.5s

# --- Close on the dashboard launch ---
Sleep 900ms
Type `todo dashboard`
Sleep 300ms
Enter
Sleep 5s
