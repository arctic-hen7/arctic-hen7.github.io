---
title: Introducing Perseus for Rust Web Development!
description: Perseus aims to be NextJS for Rust.
layout: post.njk
date: 2021-09-05
tags:
    - _blog
    - _categoryDev
    - _pinned
    - web
    - rust
    - perseus
    - frontend
    - backend
---

[Perseus](https://github.com/arctic-hen7/perseus) is a high-level web development framework for Rust, with full support for SSR, SSG, revalidation, and even incremental generation! It's based on the low-level VDOM-less reactivity framework [Sycamore](https://github.com/sycamore-rs/sycamore), and it adds higher-level functionality, like an Actix Web integrations for serving apps and even a CLI for easier development!

## Documentation

Like most of my larger projects, Perseus has two main sources of documentation:

- [API Docs](https://docs.rs/perseus)
- [The Book](https://arctic-hen7.github.com/perseus)

I've also written a tutorial on building your first app with Perseus, which covers everything from installation to serving! You can find it in the book [here](https://arctic-hen7.github.io/perseus/tutorials/first_app/intro.html).

## Project Status

At the time of writing, Perseus is in v0.1.2, and supports a lot of features. It's not ready for production yet, but it soon will be hopefully! The big issues right now are support for serverless platforms, for which I need to be able to access their filesystems, and SEO, which still needs some improvements.

That said, Perseus is perfect for really anything that's not public-facing, and, provided no one points out any gaping holes soon, it's probably great for small projects. I wouldn't advise using it in a nuclear power plant (at all), but I can't stop you!

## The Future

I want Perseus to become the NextJS of WASM, and I want to open up the phenomenal opportunities of diverse rendering strategies to Rust web developers, which for too long have only been the domain of JavaScript. There's still more work to be done, including native support for internationalization, but then I'll work on an app that uses it (and [Diana](https://github.com/arctic-hen7/diana)) to try to iron out any flaws and get the interface perfect.

For now though, you can chat about Perseus in it's [Gitter community] or on its [channel] on the Sycamore Discord server (for Sycamore-related stuff, graciously provided by [@lukechu10](https://github.com/lukechu10)), and any feedback would be greatly appreciated!
