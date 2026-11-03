//! The gaps ranked: each Missing, Partial or evidence-less function the
//! main loop reaches, by the Missing bytes behind it and its callers.

mod common;

use common::Tree;
use opence_census::classes::Class;
use opence_census::ranking::{rank, write_ranking};
use opence_census::Census;

type Failure = Box<dyn std::error::Error>;

const GAME: &str = "/*
GAME.C

symbols in this file:
00010000 0010:
\t_main_loop (0000)
00010010 0020:
\t_game_a (0000)
00010030 0030:
\t_game_b (0000)
00010060 0040:
\t_game_c (0000)
000100a0 0080:
\t_game_d (0000)
00010120 0010:
\t_game_orphan (0000)
*/

void main_loop(void)
{
\tgame_a();
\tgame_b();
}

void game_a(void)
{
\tgame_c();
}

void game_b(void)
{
\tgame_c();
}

void game_c(void)
{
\tgame_d();
}

void game_d(void)
{
}

void game_orphan(void)
{
}
";

const MANIFEST: &str = r#"
[[rule]]
class = "missing"

[function.game_b]
class = "replaced"
because = "renderer"
stands_in = "app/Sources/Render/Frame.swift"
"#;

#[test]
fn the_gaps_rank_by_the_missing_bytes_behind_them_then_their_callers() -> Result<(), Failure> {
    let tree = Tree::new("ranking")
        .with("source/game/game.c", GAME)
        .with("engine/Cargo.toml", "[workspace.dependencies]\n")
        .with("engine/app/Sources/Render/Frame.swift", "")
        .with("engine/census/census.toml", MANIFEST);
    let census = Census::read(tree.root())?;
    let ranked = rank(&census);
    let rows: Vec<(&str, u32, usize, u64)> = ranked
        .iter()
        .map(|r| (r.name.as_str(), r.size, r.callers, r.gap_bytes))
        .collect();
    // (game_b is Replaced, so it is left out and the walk stops at it;
    // game_orphan is not reached)
    assert_eq!(
        rows,
        [
            ("main_loop", 0x10, 0, 0x10 + 0x20 + 0x40 + 0x80),
            ("game_a", 0x20, 1, 0x20 + 0x40 + 0x80),
            ("game_c", 0x40, 2, 0x40 + 0x80),
            ("game_d", 0x80, 1, 0x80),
        ]
    );
    assert!(ranked.iter().all(|r| r.class == Some(Class::Missing)));
    let text = write_ranking(&ranked);
    assert_eq!(
        text.lines().nth(1),
        Some("main_loop\tgame\tMissing\tno\t16\t0\t240")
    );
    Ok(())
}
