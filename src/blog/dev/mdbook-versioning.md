---
title: Versioned Docs with mdBook
description: How to create performant, versioned docs for free with mdBook and GitHub Pages.
layout: post.njk
date: 2021-09-19
tags:
    - _blog
    - _categoryDev
    - rust
    - oss
    - tutorial
---

If you've ever worked on an open-source library that's used by other people's code, you know how important documentation is. But deploying and organizing it can be challenging and expensive, especially with commercial solutions like GitBook.

The Rust project launched [mdBook](https://rust-lang.github.io/mdBook) a few years ago to solve the problem of deployment: it allows you to build an entirely self-contained 'book' for your project in Markdown, which can then be served as a set of static files wherever you like!

In this post, I'll outline how to use mdBook to create documentation for a project, and how to set up multiple versions of that documentation, including a `next` version that can be constantly worked on. By the end, we'll have a fully functional documentation system that can be deployed to GitHub Pages for free!

## Setup

To begin, you'll want to create a `docs` directory at the root of your project. This is where we'll put everything to do with the book. You'll also want to install mdBook itself, which can be done easily if you have [Rust](https://rust-lang.org) installed already, just run `cargo install mdbook`, or otherwise you can check out their installation guide [here](https://rust-lang.github.io/mdBook/cli/index.html).

This tutorial will also cover automating many common tasks with documentation management using a tool called [Bonnie](https://github.com/arctic-hen7/bonnie) (full disclosure: I'm the maintainer), which you can install with `cargo install bonnie`, or by looking through it's [installation guide](https://github.com/arctic-hen7/bonnie#installation).

## Creating a structure

What we're building is a *versioned* book, meaning that it will have a separate book for each version of your project. In beta, these should correspond to minor version (e.g. 0.1.x, 0.2.x, etc.), and in stable these should correspond to major versions (e.g. 1.x.x, 2.x.x, etc.).

To achieve this, we'll need a separate mdBook for each version, and an additional one called `next`, which will house the documentation for the next version, which allows you to work on documentation dynamically, without having to add it all at the end for fear of disrupting existing users.

To begin, we'll create a `.gitignore` under `docs/` and put the following inside:

```
book
dist
```

This tells Git to ignore directories with the name `book/` or `dist/`, wherever under `docs/` they might occur.

Then, create the following directories:

- `common` -- stores files common to all versions, which will be kept in sync with symlinks
- `next` -- stores documentation for the next version to be released
    - `src`
- A directory for each version of your project thus far (e.g. `0.1.x`, `1.x.x`, `2.x.x`, etc.)
    - `src`

## Writing a book

Now it's time to create some actual documentation! In the `src` directory for each version, you can write documentation in Markdown, and you can create a sidebar in the specially named `src/SUMMARY.md` file.

But, before you can serve your book, you'll need to create a `book.toml` to define its options. However, this will be the same for every version, so we'll store it once in `common/` and symlink it to each version.

Create `common/book.toml` with the following content:

```toml
[book]
authors = ["your name here"]
language = "en"
multilingual = false
src = "src"
title = "Name of Your Book"

[rust]
edition = "2018"
```

This sets some metadata (including disabling internationalization, which you can read more about in mdBook's [documentation](https://rust-lang.github.io/mdBook)), and also sets the version of Rust to use for your book's integrated playground as the 2018 version of Rust. mdBook comes with a Rust interpreter built-in, which essentially adds a play button to any snippets of Rust code you declare in your book (you can stop a snippet from being played by setting the language to `rust,no_playground,no_run` instead of just `rust`). If you're not using Rust, this part is irrelevant to you.

Now we need to make sure that every version of our book has access to that common `book.toml` file, which we can do with symlinks (references to another file's location on Unix systems, like Linux and MacOS). Run the following command for each of your versions, as well as `next` (while in `docs/`):

```
ln -s ../common/book.toml [VERSION-HERE-OR-next]/book.toml
```

If you're unfamiliar with symlinks, *that first argument is relative to the location of the second*, which allows the OS to find the file without an absolute path.

Now that we've done that, you should be able to `cd` into any of your versions and run `mdbook serve` to preview that version of your book in your browser at <http://localhost:3000>!

## Adding warnings

All this is well and good, but there will only be one version of your documentation that is stable at any one time, all others will be outdated, and `next` will be unpublished. Users may well accidentally navigate to the wrong version, and we should display warnings to make sure they know where they are!

Fortunately, mdBook makes this process very easy by allowing us to modify the internals of our book under the `theme/` directory. We'll start off by doing so for the `next` version.

Start off by creating a new directory `theme/` under `docs/next/`, and create a new file called `header.hbs`. This is a Handlebars file, which is like HTML with bonuses, and it's what mdBook uses internally. The contents of this file will be appended to the top of every page of your book, which means it's the perfect place to define a warning for users that alerts them that they may be looking at the wrong version of your documentation.

Inside `header.hbs`, put the following code:

```hbs
<style>
    header.warning {
        background-color: rgb(242, 222, 222);
        border-bottom-color: rgb(238, 211, 215);
        border-bottom-left-radius: 4px;
        border-bottom-right-radius: 4px;
        border-bottom-style: solid;
        border-bottom-width: 0.666667px;
        border-image-outset: 0 0 0 0;
        border-image-repeat: stretch stretch;
        border-image-slice: 100% 100% 100% 100%;
        border-image-source: none;
        border-image-width: 1 1 1 1;
        border-left-color: rgb(238, 211, 215);
        border-left-style: solid;
        border-left-width: 0.666667px;
        border-right-color: rgb(238, 211, 215);
        border-right-style: solid;
        border-right-width: 0.666667px;
        border-top-color: rgb(238, 211, 215);
        border-top-left-radius: 4px;
        border-top-right-radius: 4px;
        border-top-style: solid;
        border-top-width: 0.666667px;
        color: rgb(185, 74, 72);
        padding-bottom: 8px;
        padding-left: 14px;
        padding-right: 35px;
        padding-top: 8px;
        text-align: center;
        margin-bottom: 0px;
        margin-left: 0px;
        margin-right: 0px;
        margin-top: 30px;
    }
</style>
<header class="warning">
    This documentation is for the <strong>unpublished</strong> next version of Perseus,
    and features documented here may not yet be available in the existing releases.
    <br>
    You can find documentation for the most recently released version of Perseus
    <a href="https://arctic-hen7.github.io/perseus/stable.html">here</a>.
</header>
```

Most of this is copied from [Perseus](https://github.com/arctic-hen7/perseus), in which I have this documentation setup deployed.

Now create another file called `common/header_old.hbs`, and put the same content in there, but with different text that tells the user that version is outdated. Now, we'll symlink that into every outdated version with these commands (run once for each old version):

```
ln -s ../../common/index.hbs [VERSION]/theme/index.hbs
ln -s ../../common/header_old.hbs [VERSION]/theme/header.hbs
```

Unfortunately, at the time of writing, there's an open issue on mdBook [here](https://github.com/rust-lang/mdBook/issues/1331) whereby that warning will be half invisible, so we'll also need to modify the body of the book to fix that. This is a pretty easy fix, but it does involve a *very* large amount of copy-pasta. Note that this file will be the same for every version with a warning (so `next` and all the old versions), so we can create this as `common/index.hbs`. Then, put the code in [this](https://gist.github.com/arctic-hen7/bff44c165fdfcbfd8b5b3b7bb6ca4587) Gist in there (all I've done is switched lines 117 and 118 so the header is below the menu bar).

Now, we'll link that common file to all the versions that need a warning with this command (run once for each version):

```
ln -s ../common/index.hbs [VERSION-HERE-OR-next]/theme/index.hbs
```

Now, if you run `mdbook serve` in any of your versions with a warning, you should see it at the top of the page!

## Making the `stable.html` alias

You've probably noticed that, in the example warning I provided before, I included a link to the latest stable version of the documentation. If you have a ton of old versions, you don't want to be updated this constantly, hence, we can use a `stable.html` file to redirect the user to the latest stable version, and, when we push a new version, we only have to change one thing!

This process is very simple, just create a `stable.html` file at the root of `docs/`, and add the following:

```html
<!-- This file redirects the user to the latest stable version. -->
<!DOCTYPE html>
<html>
    <head>
        <meta
            http-equiv="refresh"
            content="0; url = https://arctic-hen7.github.io/perseus/0.1.x"
        />
    </head>
</html>
```

This document has no `<body>`, it literally just redirects the user. Of course, replace `https://arctic-hen7.github.io/perseus` with your own link. When you push a new version, you can update this by changing `0.1.x` to `0.2.x` or the like!

## Building everything

Now that we've got versioning set up, it's time to build the book into one single directory of static files that we can serve, for example on GitHub Pages. We'll do this with a simple Bash script, which we'll put at `docs/build.sh`.

In that file, put the following:

```bash
#!/bin/bash

mkdir -p dist
rm -rf ./dist/*

# Build `next`
cd next
mdbook build -d ../dist/
cd ../

# Loop through all directories that aren't `common` or `next` (or `dist` of course)
for d in */ ; do
    if [[ $d == "common/" || $d == "next/" || $d == "dist/" ]]; then
        continue
    else
        cd $d
        mdbook build -d ../dist/$d
        cd ../
    fi
done

# Copy in the redirection for the latest stable version
cp stable.html dist/stable.html
```

Here's what this code does (note that it should be executed in `docs/`):

1. Make sure the `dist/` directory exists and that it's empty.
2. Build the `next` version to the root of that directory. This means that users will, by default, be served with the latest version of your documentation, which avoids problems of initial redirection. This is also common practice with most OSS projects.
3. Loop through all directories in `docs/` that aren't `next`, `common`, or `dist` (the directories for each version), and build them to `dist/[VERSION]`.
4. Copy `stable.html` into the root of `dist/`.

You can run this script with this command (we don't need to make it executable):

```
bash build.sh
```

After that, open `dist/index.html` in your browser, and you should see the `next` version of your documentation, with a link to the latest stable version! If you go to `0.1.x` or some other old version, you should see a warning saying it's outdated and a link back to the latest stable version!

## Automating with Bonnie

This process of updating your documentation for a new version can be tedious, but it can be automated easily with a tool I built a little while ago called [Bonnie](https://github.com/arctic-hen7/bonnie). If you haven't installed this yet (`cargo install bonnie` or see [the installation docs](https://github.com/arctic-hen7/bonnie#installation)), do so now.

First off, a code dump for your `bonnie.toml` (this should be in the root of your project, outside `docs/`).

```toml
version="0.3.2"

[scripts]
docs.cmd = [
    "cd docs/next",
    "mdbook serve"
]
docs.desc = "hosts the latest version of the book locally at http://localhost:3000"
docs.subcommands.version.cmd = [
    "cd docs/%version",
    "mdbook serve"
]
docs.subcommands.version.args = [ "version" ]
docs.subcommands.version.desc = "hosts the given version of the book locally at http://localhost:3000"
docs.subcommands.deprecate.cmd = [
    "cd docs/%version",
    "mkdir theme",
    "ln -s ../../common/index.hbs theme/index.hbs",
    "ln -s ../../common/header_old.hbs theme/header.hbs"
]
docs.subcommands.deprecate.args = [ "version" ]
docs.subcommands.deprecate.desc = "marks the given version of the docs as old and links to the latest"
docs.subcommands.create.cmd = [
    "mkdir docs/%version",
    "cd docs/%version",
    "ln -s ../common/book.toml book.toml",
    "cp -r ../next/src src",
    "cd ../",
    "sed -i -E 's/perseus\\/(.+)\"/perseus\\/0.2.x\"/' stable.html"
]
docs.subcommands.create.args = [ "version" ]
docs.subcommands.create.desc = "creates documentation for a new version from `next` and marks it as stable (doesn't deprecate old versions though)"
docs.subcommands.build.cmd = [
    "cd docs",
    "bash ./build.sh"
]
docs.subcommands.build.desc = "builds the book for deployment to GitHub pages or the like"
```

First, this declares the version of Bonnie we're using (v0.3.2 at time of writing), which you can get by running `bonnie -v` in your terminal. Then, we define the following commands (you can see details by typing `bonnie help` in the root of your project):

- `docs` -- hosts the `next` version of your docs at <http://localhost:3000>
- `docs version <version>` -- hosts version `<version>` of your docs at <http://localhost:3000>
- `docs deprecate <version>` -- adds an outdated banner to `<version>`
- `docs create <version>` -- creates a new version of your docs from `next` and marks it as stable
- `docs build` -- runs `docs/build.sh` and builds your docs to `docs/dist/`

The only thing we need to change here is in the last line of `docs.subcommands.create.cmd`, which is a `sed` substitution that replaces the current stable version with the new one in `docs/stable.html`. Replace `perseus` in this with whatever comes directly before the version of your docs. For example, if your docs for `1.x.x` are hosted at `example.com/docs/1.x.x`, this would be `docs`. If they were directly at `example.com`, this would be `com`. You can use [this tool](https://regex101.com) to make sure that substitution works.

After you've done that, you can now control your documentation with ease!

## GitHub Pages and Actions

Now it's time to deploy your book to the internet! We'll do this on [GitHub Pages](https://pages.github.com), a free service operated by GitHub that lets you create a website associated with your user account (mine is <https://arctic-hen7.github.io>, which is what you're looking at right now!).

We can use another service called [GitHub Actions](https://actions.github.com) to automatically build your book and deploy it whenever you push to your repository, which is free for public repositories, and paid for private (with a small free tier). I'll assume here that you've already set up a Pages site for your account, but if not, there's a great tutorial [here](https://github.com/features/actions) by GitHub themselves.

Now, to create a GitHub Actions workflow, create a new file at the root of your repository/project called `.github/workflows/book.yml` and put the following inside:

```yaml
name: Compile and Deploy Book

on:
    push:
        paths:
            - "docs/**"
            - ".github/workflows/book.yml" # If we change this build script, it should rerun

jobs:
    deploy:
        runs-on: ubuntu-latest
        steps:
            - uses: actions/checkout@v2
            - name: Setup mdBook
              uses: peaceiris/actions-mdbook@v1
              with:
                  mdbook-version: "latest"
            - name: Build book
              run: bash build.sh
              working-directory: docs
            - name: Deploy book to GitHub Pages
              uses: peaceiris/actions-gh-pages@v3
              if: github.ref == 'refs/heads/main'
              with:
                  github_token: ${{ secrets.GITHUB_TOKEN }}
                  publish_dir: docs/dist
```

This creates a new workflow called `Compile and Deploy Book` that runs whenever you push to GitHub and there's a change to either the `docs/` folder or this script itself. Then, it runs a single job called `deploy`, which will run on Ubuntu (in the cloud). Here's what that does:

1. Installs mdBook using [this action](https://github.com/peaceiris/actions-mdbook).
2. Builds your book using `docs/build.sh`.
3. Deploys the `docs/dist` to a new `gh-pages` branch on your repository with [this action](https://github.com/peaceiris/actions-gh-pages).

Now try pushing to your repository on Github as usual (`git push origin main` or something similar), and go to the `Actions` tab of your repository on GitHub. You should see a new job running, which should complete pretty quickly (installing mdBook will be the longest-running part), and then you'll be able to see a new `gh-pages` branch in the `Code` tab! Now, this won't have deployed to Pages just yet, we'll need to configure that under the `Settings` tab. Go there, and then go to `Pages` in the sidebar, and set the branch to `gh-pages` and press *Save*! You should see a green message at the top of that page when you reload that has a URL to your newly deployed book!

## Closing Words

In this tutorial, we've built a fully versioned and automated documentation system for your project, which can be deployed for free to the wider internet!

If you're stuck on any part of this, you can see the entire setup in my [Perseus](https://github.com/arctic-hen7/perseus) project, which uses this setup verbatim.
