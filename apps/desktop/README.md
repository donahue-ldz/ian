# Ian Desktop

P0 desktop shell for Ian, a local-first desktop creature.

## Commands

```bash
npm install
npm run desktop:test
npm run desktop:typecheck
npm run desktop:build
npm run desktop:tauri -- dev
```

Rust is the source of truth for `IanEvent`, `IanAction`, and `IanState` under
`src-tauri/src/protocol`. The checked-in TypeScript file at
`src/protocol/generated.ts` is a P0 generated placeholder until the local Rust
toolchain can run `ts-rs` export in this workspace.

## Architecture Boundary

- Rust Core receives `IanEvent` and returns `IanAction`.
- React renders actions and does not own Ian's behavior policy.
- Resource Pack manifests live under `public/resources/pets/ian-alpaca`.
- Future adapters, storage, dialogue providers, and security gates exist as
  skeletons only.
