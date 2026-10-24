# Export linked assets with the generated page

Render Page into index.html and write every Bundle::paths entry using Bundle::get. The default page links assets under /_stucco/. Serve the output directory as the site root so those absolute URLs resolve. No Axum feature or dynamic HTTP service is needed to generate these files. A static host is still needed to deliver the game over HTTP.

Basis: Documented.

Features: No additional features stated.

## Evidence

- [e0036 lines 179–195](../evidence/e0036.txt) ([source](https://github.com/mcaveniathor/stucco/blob/0bc5845c04bc4c841ca855edde0d86247985dd63/docs/guide/recipes.md))
- [e0030 lines 48–50](../evidence/e0030.txt) ([source](https://github.com/mcaveniathor/stucco/blob/0bc5845c04bc4c841ca855edde0d86247985dd63/docs/guide/enhancement.md))
