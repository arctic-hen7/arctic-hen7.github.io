---
title: Automation with Bonnie
description: How to automate repetitive tasks in your projects with Bonnie.
layout: post.njk
date: 2021-08-12
tags:
    - _blog
    - _categoryDev
    - bonnie
    - automation
    - promotion
---

As developers, we're faced with, generally speaking, three kinds of work: **problem solving**, **implementation**, and **mundane tasks**. Problem solving can be hard, but it's extremely rewarding. Implementing code is generally great, except that it inevitably generates problems for you to solve, but that's part of the joy!

In this post though, I want to focus on the third type of work we do -- mundane, repetitive tasks. This could be anything, like a long build command, typing the same sequence of commands over and over again, or banging your head against a wall when you try to read your own code six months later.

Bonnie is a tool I built a few months ago as my first ever project in [Rust](https://rust-lang.org), and since then I've made significant updates that have improved every facet of it. It was originally intended as a replacement for NPM scripts, the simple little command aliasing system many JavaScript developers will be familiar with. Problem is, Rust doesn't have that, nor do a lot of languages! Bonnie was designed to be a cross-platform system to enable command aliasing in any project, regardless of programming language!

Today, Bonnie supports custom arguments to scripts, environment variable interpolation, subcommands, conditional command ordering, and so much more!

## Installing

Bonnie has pre-built binaries for Linux (and musl), MacOS, and Windows, which you can download from [here](https://github.com/arctic-hen7/bonnie/releases). After downloading the right one, put it in a directory from which you can execute scripts, like `/usr/local/bin` on Linux.

If you're using Docker or a platform for which there's not a pre-built binary, check out the installation instructions on the homepage [here](https://github.com/arctic-hen7/bonnie). If you'd really like me to add a binary for another platform, open an issue on the project and I'm happy to oblige!

## Setup

Once you can run Bonnie, make sure everything's working by running `bonnie -v`. You should see the version of Bonnie you're running. Bonnie will now work. If it doesn't, I've stuffed up, and you should slap me in the face with a cold fish (or a GitHub issue).

## Writing scripts

So, let's say you've got a build script that's _really_ long. Easy! Run `bonnie -i` to create a new configuration file in your current directory (assuming you're in your project's directory), and pop this into `bonnie.toml` underneath `[scripts]`, substituting in your build command.

```toml
build = "super long build command..."
```

Congrats! Now run `bonnie build` and your command should run! That's the beauty of Bonnie, **it gets out of your way and just works**.

You can repeat this process for as many things as you like, and Bonnie scripts can call other scripts easily! Every script will be executed as if you'd run it in a terminal (PowerShell on Windows and `sh` on MacOS and Linux), so just add `bonnie command-here` into a script!

If you want to replicate the functionality of NPM scripts, where any extra arguments you give after the command name will be added to the end (e.g. `npm build blah blah` becomes `[build command] blah blah`), just add `%%` wherever in your command you want those arguments to be put (so you can put them at the end, the beginning, in the middle, anywhere!).

Bonnie will also let you change the shell it runs things in (so you could use `zsh` or `fish` instead), specify multi-stage commands (just provide an array!), or even add subcommands! If you're feeling adventurous, you could try out [Bones](https://github.com/arctic-hen7/bonnie/wiki/Getting-Started-with-Bones), which lets you run subcommands one after the other in a custom order, with if statements based on the exit codes commands give out! All the details are in [the wiki](https://github.com/arctic-hen7/bonnie/wiki)!

## Final words

Bonnie is a great tool for automating repetitive work, it's full of features, lightning-fast, and it just gets out of your way. Check it out on GitHub [here](https://github.com/arctic-hen7/bonnie)!

_Note: I am the maintainer of Bonnie._
