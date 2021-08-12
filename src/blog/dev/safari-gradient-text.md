---
title: Gradient Text on Safari
description: How to fix missing gradient text on Safari.
layout: post.njk
tags:
    - _blog
    - _categoryDev
    - tailwind
    - css
    - webdev
    - solution
---

## Problem

You've got some text with a beautiful gradient, it looks fantastic! Then you open your gorgeous new website in Safari, and half your text is missing. This is particularly from using `background-clip` to create your gradients (as you would with [Tailwind](https://tailwindcss.com)), and if the text goes multi-line, Safari will cut the second line off erratically. [Here's](https://bugs.webkit.org/show_bug.cgi?id=169125) the bug report filed for WebKit a few years ago, which still hasn't been addressed.

## Solution

```css
-webkit-box-decoration-break: clone;
```

Yep, that's all. That should fix the problem completely on Safari! If you want an example, this is in use on this very website in the `pageHeader.njk` component, which you can see [here](https://github.com/arctic-hen7/arctic-hen7.github.io/blob/cef3ee6b1715c1fb02f498188e57f2c746997dff/src/_includes/components/pageHeader.njk#L4)!

In case this solution didn't work for you, there's a StackOverflow thread [here](https://stackoverflow.com/questions/44963978/safari-on-ios-not-displaying-text-when-using-background-clip-and-text-fill-color) with a few other options that will hopefully work for you!

