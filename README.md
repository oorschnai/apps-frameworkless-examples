# Apps frameworkless examples

Minimal backend projects for testing the `another` Apps templates.

| Branch | Runtime | Start command |
| --- | --- | --- |
| `python` | Python | `python3 main.py` |
| `node` | Node.js | `node index.js` |
| `go` | Go | `./main` |
| `php` | PHP with Apache | `apache2-foreground` |
| `php-cli` | PHP worker | `php worker.php` |
| `bun` | Bun | `bun index.ts` |
| `deno` | Deno | `deno run --allow-net --allow-env --allow-read main.ts` |
| `rust` | Rust | `/app/server` |

Each branch has one small project without a named framework. They are intended
for Apps end-to-end and dev checks.
