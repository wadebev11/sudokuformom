# Sudoku for Mom

A Sudoku game for one player, made for Mom and played on her phone.

## Language

### The grid

**Box**:
One of the nine 3×3 squares the grid is divided into.
_Avoid_: Block, region, nonet

**Unit**:
A row, column or Box. It holds each number from 1 to 9 exactly once.
_Avoid_: House, group

**Peer**:
A cell that shares a Unit with a given cell, so it cannot hold the same number.
_Avoid_: Neighbour, buddy

**Given**:
A number the puzzle starts with. The player cannot change it.
_Avoid_: Starting number

### Solving

**Candidate**:
A number that could still go in a cell, given the numbers already placed in its Peers.
_Avoid_: Possibility, option

**Pencil Mark**:
A small number the player writes in a cell to track the Candidates she thinks are possible. It can be wrong or incomplete. "Notes" is fine in text the player reads.
_Avoid_: Annotation

**Technique**:
A named logical deduction a person uses to place a number or rule one out, such as a hidden single or an X-Wing.
_Avoid_: Strategy, rule

**Naked Single**:
An empty cell with only one Candidate left. It is decided by the numbers on the board, not by the player's Pencil Marks.
_Avoid_: Lone single, sole candidate

**Elimination**:
Ruling a Candidate out of a cell by applying a Technique.
_Avoid_: Crossing out

**Mistake**:
A number the player entered that does not match the puzzle's one solution. Mistakes show in red right away, unless the player turns that off.
_Avoid_: Error, wrong answer

### Playing

**Difficulty**:
One of four levels a puzzle belongs to: Easy, Medium, Hard, Expert. It is decided by the hardest Technique needed to solve the puzzle, not by how many Givens it has. Easy and Medium puzzles also start with a minimum number of Givens, so they don't feel sparse.
_Avoid_: Level, mode

**Auto-solve**:
An optional setting, off by default, that enters the number in every Naked Single and keeps going until none are left. It never applies to Easy puzzles, which would almost always solve themselves before the first move. It is named for Naked Singles only, so it isn't mistaken for also solving hidden singles.
_Avoid_: Auto-fill, auto-place

**Hint**:
The simplest Technique that makes progress on the current board, shown with the cells involved and an explanation. Applying it places a number or makes Eliminations from the Pencil Marks. If the board has a Mistake, the Hint points that out instead.
_Avoid_: Tip

**Problem Report**:
A copy of the game in progress, the settings and any Hint showing, which the player sends to Wade when something looks wrong. She describes what happened in her own words alongside it.
_Avoid_: Bug report, feedback
