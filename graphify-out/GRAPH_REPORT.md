# Graph Report - .  (2026-06-19)

## Corpus Check
- Corpus is ~3,158 words - fits in a single context window. You may not need a graph.

## Summary
- 126 nodes · 214 edges · 11 communities (10 shown, 1 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 6 edges (avg confidence: 0.8)
- Token cost: 0 input · 16,516 output

## Community Hubs (Navigation)
- [[_COMMUNITY_TCP Server & Client|TCP Server & Client]]
- [[_COMMUNITY_Game Engine Core|Game Engine Core]]
- [[_COMMUNITY_Engine Tests|Engine Tests]]
- [[_COMMUNITY_Game State Model|Game State Model]]
- [[_COMMUNITY_Engine Runtime|Engine Runtime]]
- [[_COMMUNITY_Graphify Tooling Meta|Graphify Tooling Meta]]
- [[_COMMUNITY_CLI Entry|CLI Entry]]
- [[_COMMUNITY_Binary Main Entry|Binary Main Entry]]

## God Nodes (most connected - your core abstractions)
1. `GameEngine` - 15 edges
2. `String` - 12 edges
3. `TickResult` - 10 edges
4. `Result` - 9 edges
5. `parse_server_input()` - 9 edges
6. `ServerRequest` - 8 edges
7. `combat_state()` - 7 edges
8. `GameState` - 7 edges
9. `handle_client()` - 7 edges
10. `EngineRuntime` - 6 edges

## Surprising Connections (you probably didn't know these)
- None detected - all connections are within the same source files.

## Import Cycles
- 1-file cycle: `src/engine.rs -> src/engine.rs`

## Hyperedges (group relationships)
- **Graphify Query Commands** — claude_graphify_query, claude_graphify_path, claude_graphify_explain [EXTRACTED 1.00]

## Communities (11 total, 1 thin omitted)

### Community 0 - "TCP Server & Client"
Cohesion: 0.18
Nodes (29): EngineResponse, EngineRuntime, Sender, ServerInput, dispatch_line(), format_response(), format_state(), format_tick_result() (+21 more)

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
Cohesion: 0.24
Nodes (10): Community Structure, God Nodes, graph.json, GRAPH_REPORT.md, Graphify Knowledge Graph, graphify explain, graphify path, graphify query (+2 more)

### Community 7 - "CLI Entry"
Cohesion: 0.67
Nodes (3): main(), print_help(), Result

## Knowledge Gaps
- **23 isolated node(s):** `Result`, `CommandRejectionReason`, `Vec`, `Self`, `GameState` (+18 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **1 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **What connects `Result`, `CommandRejectionReason`, `Vec` to the rest of the system?**
  _23 weakly-connected nodes found - possible documentation gaps or missing edges._