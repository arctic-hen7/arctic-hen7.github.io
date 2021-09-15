---
title: How to Fix Ubuntu Broken After Deleting Rollup
description: A pretty simple fix to a downright weird problem.
layout: post.njk
date: 2021-09-15
tags:
    - _blog
    - _categoryDev
    - ubuntu
    - linux
    - fix
---

This post details one of the strangest failures I've ever seen in Ubuntu and how to fix it. For me, this occurred after deleting `rollup`, but it may have been something else for you! Please leave a comment if you've had this problem and if you have any idea how it works!

## Scenario

I was working on [Perseus](https://github.com/arctic-hen7/perseus) this morning, specifically removing the dependency on [Rollup]() for the project. So, I naturally decided to remove Rollup completely from my system to test that there was no longer any dependency.

So, I did what made sense; `yarn global remove rollup`. And, to my dismay, `rollup` still worked as a command. I ran `which rollup` to find the exact location of the binary, which was in `~/.yarn/bin`. And then I did what you should **never** do. I decided to delete is manually with `rm`.

Now it's worth noting that around this time my system had become very slow due to some kind of swap glitch, and so I decided to restart, having just deleted a binary manually. Now it's worth noting that I installed `rollup` through `yarn`, which I installed through `npm`, which I installed through `nvm` (Node version manager), which I installed with `curl`. That's *five* degrees of separation from anything that should be remotely system-critical, hence my confusion at what happened next.

So, booting my computer back up, everything was working as normal. I got to the login screen fine, the fan still whirring a bit from the swap glitch, and I typed in my password as usual. Then, as my desktop loaded, my mouse froze. I could not interact with my system in any way whatsoever. I rebooted *three* times, the exact same thing.

## The Solution

My first thought was to change my login shell, which is `fish`, to something more mainstream like `bash`, as that could have some `rollup` dependency that might be causing a system hang. I did that with recovery mode, which I'll explain now (you'll need this), but to no avail.

### Ubuntu Recovery Mode

Ubuntu has a very helpful boot mode called recovery mode that you can use to repair a broken system. You'll need to be able to boot your system for this, which was fine for me because it was a post-login issue.

You can access recovery mode by spamming `Esc` as your computer boots to stop UEFI fast boot, and then press `Enter` to continue boot. Immediately after that, hold down `Shift` (don't spam it) to get the GRUB menu, the bootloader. Then go down to something like `Advanced Options`, and then select `Recovery Mode` with the newest kernel (for this issue at least, but this can also be used to revert broken kernel upgrades). You should now be taken to a terminal-only screen (where you'll need to decrypt your hard drive and the like), and then you'll get a menu of options. This will boot your filesystem read-only, so it's practically impossible to do any damage without changing that, which you can do in 20.04 and later by selecting `Enable networking` and pressing `Enter`. Then press the option that lets you drop into a root shell, and you now have a root terminal on your system!

*Aside: THIS is why you should encrypt your hard drive! Otherwise, yeah, anyone could use this to do [anything they want](https://r3xnation.wordpress.com/2016/11/01/how-to-hack-into-ubuntuany-version/) (weird link but good explanation) to your system.*

### Actually Fixing It

After logging into a hung desktop with 10 minutes until I was needed on a Zoom meeting, I booted back into recovery mode, enabled networking, and switched to my user account with `su arctic_hen7`. I confirmed that `rollup` didn't exist, and then found that `yarn` didn't either. That was weird, and neither did `node`! In case you're thinking of weird `fish` stuff with loading custom binaries, I was in `bash` at this point. I still have no idea how deleting a binary manually managed to corrupt the entire JavaScript stack on my computer.

Nevertheless, I reinstalled Node using [NVM]() (which I already had installed), reinstalled `yarn`, and then ran `yarn global add rollup` for good measure. I then tried popping into `fish` to see how it was going. I use `fish` with [Starship](), and it was telling me that the latter wasn't working. Two minutes until that Zoom meeting by the way.

I frantically rebooted and crossed my fingers, noting that my shell was still set to `bash` in case Starship was still an issue. And it worked! Until a spontaneous logout 12 minutes later, which I think was unrelated, because it didn't happen again and still hasn't yet (I am so expecting one as a write this...).

After my Zoom meeting, I then tried using `fish` again: errors were gone. I assume Starship was trying to do something that's not allowed in recovery mode, but I can't yet remember the error message, so I have no idea what.

## Closing Words

So, that was one of the weirdest issues I've ever encountered in all my time using Linux, but thankfully it was a pretty easy fix. I have still found no documentation online about this, so hopefully this helps someone at some point in the future!

Also, if you've had this issue, please let me know in the comments!

Oh, and the lesson from all this is to **NEVER MANUALLY DELETE BINARIES!!!**
