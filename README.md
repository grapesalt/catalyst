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

### Watch the site

Watches for file changes and automatically rebuilds.

```bash
cargo run -- watch
```

### Create a new post

```bash
cargo run -- add "My Post Title"
```

Create a post in a subfolder:

```bash
cargo run -- add "My Post" -f math/calculus
```

### List all the posts

```bash
cargo run -- list
```

## Configuration

Create a `catalyst.yaml` file:

```yaml
title: "Sample Site"
logo: "logo.png"
entries: "content"
build: "public"
theme: "static"
```

## Theme

Your theme directory should include a `template.html` which is used for the posts, and a `index.html` which is used for, you guessed it, the `index` file.

## Roadmap

- [x] Incremental builds
- [ ] Handle paths properly
- [ ] Add an actual template engine
- [ ] Add tags/categories
- [ ] Better errors
- [ ] RSS feed
- [ ] Sitemap
- [ ] Static assets
- [ ] Image optimization
- [ ] TOC
