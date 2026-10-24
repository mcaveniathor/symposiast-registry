# Choose an explicit theme for a shell without Stucco scripts

Page::server_theme(Some(Scheme::Dark)) omits the localStorage theme initializer. With simple elements and components that have no JavaScript behavior, and without enhanced(), the shell can render without script tags. Adding enhanced navigation or scripted components changes that requirement. The game loader must be supplied separately.

Basis: Documented.

Features: No additional features stated.

## Evidence

- [e0036 lines 333–353](../evidence/e0036.txt) ([source](https://github.com/mcaveniathor/stucco/blob/0bc5845c04bc4c841ca855edde0d86247985dd63/docs/guide/recipes.md))
