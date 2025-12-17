# Catalyst

Catalyst is a tool I built to generate my personal site. It’s fast, minimal, and does exactly what I need. If you’re looking for a general-purpose site generator, though, I’d recommend checking out other tools that are much better suited for that.

## Installation

```bash
cargo build --release
```

## Usage

### Build the site

```bash
cargo run -- build
```

### Run the site

Watches for file changes and automatically rebuilds.

```bash
cargo run -- run
```

Serves the pre-built site on a local development server. Does not support hot-reloading.

```bash
cargo run -- run --serve
```

### Create a new post

```bash
cargo run -- add "My Post Title"
```

Create a post in a subdirectory:

```bash
cargo run -- add "My Post" -d math/calculus
```

## Configuration

Create a `catalyst.yaml` file:

```yaml
site_title: "Sample Site"
site_logo: "logo.png"
content_dir: "content"
output_dir: "output"
theme_dir: "static"
```

## Theme

Your theme directory should include a `template.html` which is used for the posts, and a `index.html` which is used for, you guessed it, the `index` file.
