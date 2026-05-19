#### project idea or so
```text
project/
├── .github/
│   ├── workflows/
│   │   ├── ci.yml
│   │   ├── simulation-verify.yml
│   │   └── deploy.yml
│   └── ISSUE_TEMPLATE/
├── packages/
│   ├── simulation-core/
│   │   ├── src/
│   │   │   ├── ecs/
│   │   │   ├── systems/
│   │   │   ├── pathfinding/
│   │   │   ├── procgen/
│   │   │   ├── ai/
│   │   │   ├── events.rs
│   │   │   ├── determinism.rs
│   │   │   └── lib.rs
│   │   ├── benches/
│   │   └── tests/
│   │       ├── determinism.rs
│   │       └── systems/
│   ├── server/
│   │   ├── src/
│   │   │   ├── ws/
│   │   │   ├── session/
│   │   │   ├── tick/
│   │   │   ├── event_log/
│   │   │   ├── broadcast/
│   │   │   └── metrics/
│   │   └── Cargo.toml
│   ├── client/
│   │   ├── src/
│   │   │   ├── engine/
│   │   │   ├── renderer/
│   │   │   ├── ui/
│   │   │   ├── store/
│   │   │   ├── network/
│   │   │   ├── workers/
│   │   │   └── tools/
│   │   └── vite.config.ts
│   ├── replay-service/
│   │   └── src/
│   ├── load-tests/
│   └── tools/
├── infra/
│   ├── terraform/
│   ├── k8s/
│   ├── docker/
│   └── grafana/
├── docs/
│   ├── architecture.md
│   ├── determinism.md
│   ├── ecs-design.md
│   ├── event-sourcing.md
│   ├── replay-system.md
│   ├── networking.md
│   ├── ai-agents.md
│   └── scaling.md
├── benchmarks/
├── docker-compose.yml
├── docker-compose.dev.yml
├── Cargo.toml
├── pnpm-workspace.yaml
└── README.md
````
