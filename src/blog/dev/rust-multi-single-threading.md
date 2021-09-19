---
title: Rust Single and Multi Threading in One Interface
description: How to make something multi-threaded single-threaded effortlessly in Rust.
layout: post.njk
date: 2021-09-17
tags:
    - _blog
    - _categoryDev
    - rust
    - multithreading
    - async
---

Rust is famous for its incredible multi-threading capabilities, which are fully memory-safe, and prevent nearly every error that can occur in C multithreading through the ingenious ownership system. However, there are cases in which you may want a multi-threaded system to have a single-threaded option, particularly for machines on older Intel processors (which often have poorer multi-threading performance). I recently encountered this while working on the [Perseus CLI](https://arctic-hen7.github.io/perseus/cli.html), which parallelizes its various build commands to make builds *really* fast, but this doesn't work so well on older systems, and there may well be other cases in which single-threading may be more desirable for an end user.

In light of this, I was wondering if it would be possible to make a multi-threaded system have an option for single-threadedness **without changing its existing interface** beyond changing a few types.

TODO
