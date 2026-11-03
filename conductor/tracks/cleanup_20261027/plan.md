# Plan: the cleanup after the change of direction (Track B)

## Phase 1: Clear the ground

- [x] B1: Write the retrospective of the fork in the road. 4931389
- [x] B2: Archive every track from before the change of direction. 795b0d1
- [x] B5: Write the four tracks of the new direction. 1153d7a
- [x] B3: Remove the launcher; the app starts the game directly. 18b3e5d
  (with d373261, which gave the menu tests' fake the pause menu's call)
- [x] B4: Rewrite CLAUDE.md, AGENTS.md, product.md and tech-stack.md for
  the new direction.

B5 was written before B3 and B4 so that the rules in B4 could name the
tracks. The Swift tests ran in a clean worktree of HEAD, because another
session's untracked `ZZLauncherShotTests.swift` refers to the launcher.
