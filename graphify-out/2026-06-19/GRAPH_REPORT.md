# Graph Report - osrs-engine  (2026-06-19)

## Corpus Check
- 17 files · ~4,937 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 165 nodes · 258 edges · 15 communities (13 shown, 2 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 6 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `f03757f8`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- [[_COMMUNITY_TCP Server & Client|TCP Server & Client]]
- [[_COMMUNITY_Game Engine Core|Game Engine Core]]
- [[_COMMUNITY_Engine Tests|Engine Tests]]
- [[_COMMUNITY_Game State Model|Game State Model]]
- [[_COMMUNITY_Engine Runtime|Engine Runtime]]
- [[_COMMUNITY_Graphify Tooling Meta|Graphify Tooling Meta]]
- [[_COMMUNITY_Server Command Tests|Server Command Tests]]
- [[_COMMUNITY_CLI Entry|CLI Entry]]
- [[_COMMUNITY_Binary Main Entry|Binary Main Entry]]
- [[_COMMUNITY_Community 11|Community 11]]
- [[_COMMUNITY_Community 12|Community 12]]
- [[_COMMUNITY_Community 13|Community 13]]

## God Nodes (most connected - your core abstractions)
1. `GameEngine` - 15 edges
2. `String` - 12 edges
3. `TickResult` - 10 edges
4. `Result` - 9 edges
5. `parse_server_input()` - 9 edges
6. `ServerRequest` - 8 edges
7. `AppState` - 7 edges
8. `combat_state()` - 7 edges
9. `GameState` - 7 edges
10. `handle_client()` - 7 edges

## Surprising Connections (you probably didn't know these)
- None detected - all connections are within the same source files.

## Import Cycles
- 1-file cycle: `src/engine.rs -> src/engine.rs`

## Hyperedges (group relationships)
- **Graphify Query Commands** — claude_graphify_query, claude_graphify_path, claude_graphify_explain [EXTRACTED 1.00]

## Communities (15 total, 2 thin omitted)

### Community 0 - "TCP Server & Client"
Cohesion: 0.18
Nodes (29): EngineResponse, EngineRuntime, ServerInput, dispatch_line(), format_response(), format_state(), format_tick_result(), handle_client() (+21 more)

### Community 1 - "Game Engine Core"
Cohesion: 0.18
Nodes (12): CommandRejectionReason, GameCommand, GameEngine, GameEvent, in_range(), GameState, Position, Self (+4 more)

### Community 2 - "Engine Tests"
Cohesion: 0.15
Nodes (7): aggressive_npc_hits_player_and_auto_retaliate_fights_back(), attacking_an_unknown_npc_is_rejected(), combat_state(), killing_an_npc_removes_it_and_clears_the_target(), passive_npc_does_not_attack_until_provoked(), player_walks_to_target_then_hits_on_attack_speed_cadence(), GameState

### Community 3 - "Game State Model"
Cohesion: 0.20
Nodes (10): Npc, NpcId, Option, Player, GameState, Npc, Player, Position (+2 more)

### Community 4 - "Engine Runtime"
Cohesion: 0.24
Nodes (7): GameEngine, EngineRequest, EngineResponse, EngineRuntime, GameState, Self, TickResult

### Community 5 - "Graphify Tooling Meta"
Cohesion: 0.15
Nodes (14): Agent skills, Community Structure, Domain docs, God Nodes, graph.json, GRAPH_REPORT.md, graphify, graphify explain (+6 more)

### Community 6 - "Server Command Tests"
Cohesion: 0.12
Nodes (18): AppState, client(), main(), seed_world(), send_snapshot(), Snapshot, ws_handler(), GameCommand (+10 more)

### Community 7 - "CLI Entry"
Cohesion: 0.67
Nodes (3): main(), print_help(), Result

### Community 11 - "Community 11"
Cohesion: 0.33
Nodes (5): Before exploring, read these, Domain Docs, File structure, Flag ADR conflicts, Use the glossary's vocabulary

### Community 12 - "Community 12"
Cohesion: 0.40
Nodes (4): Conventions, Issue tracker: Local Markdown, When a skill says "fetch the relevant ticket", When a skill says "publish to the issue tracker"

## Knowledge Gaps
- **42 isolated node(s):** `Result`, `Vec`, `GameEvent`, `Sender`, `Receiver` (+37 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **2 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **What connects `Result`, `Vec`, `GameEvent` to the rest of the system?**
  _42 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Server Command Tests` be split into smaller, more focused modules?**
  _Cohesion score 0.11594202898550725 - nodes in this community are weakly interconnected._