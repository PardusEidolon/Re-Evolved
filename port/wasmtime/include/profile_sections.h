/*
PROFILE_SECTIONS.H

game.wasm's CPU sections around the game's steps (port/linux/include/
profile_sections.h has the native builds'): each section's entry and exit
is an import the runtime answers (engine/crates/runtime/src/profile.rs),
with the section's name, so a benchmark times the game's own frame and
tick. Found before the Linux port's header (tools/engine_wasm.py).
*/

#ifndef __PROFILE_SECTIONS_H
#define __PROFILE_SECTIONS_H

void halo_profile_enter(const char *name);
void halo_profile_exit(const char *name);

/* call sites have no trailing semicolon, so these macros bring their own; a
call site may be an unbraced if-branch, so profile_scope is one block */
#define PROFILE_SECTION(variable, name) static const char variable[] = name;
#define profile_scope_enter(variable) halo_profile_enter(variable);
#define profile_scope_exit(variable) halo_profile_exit(variable);
#define profile_scope(variable, statement) { halo_profile_enter(variable); statement halo_profile_exit(variable); }

#endif
