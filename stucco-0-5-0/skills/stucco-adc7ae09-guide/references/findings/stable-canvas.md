# Keep a fixed canvas mount in trusted application markup

Use Raw::trusted only for developer-authored static canvas markup with a fixed ID; use normal escaped components for player-provided text. The game client can select that ID and own the canvas contents. Keeping it mounted during UI updates is an application responsibility, not a Stucco lifecycle guarantee.

Basis: Inferred from the cited sources.

Features: No additional features stated.

## Evidence

- [e0028 lines 47–58](../evidence/e0028.txt) ([source](https://github.com/mcaveniathor/stucco/blob/0bc5845c04bc4c841ca855edde0d86247985dd63/docs/guide/components.md))
