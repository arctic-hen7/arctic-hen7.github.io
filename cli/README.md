# CLI

You would be very justified in asking why on earth my blog has a command-line interface. The answer is, well, I like to automate things. Since my blog imports all its posts from my Zettelkasten (the concept of which I have brutally abused to create a flat file system that basically stores my entire digital life), and decides which files shoudl be public based on a tags system, I like to be abel to control when posts are added, when they're updated, deleted, etc. This also provides a system for converting posts into HTML *before* they reach the Perseus build process, which means builds are faster (since rendering my posts to HTML requries calls to LaTeX, Emacs, etc.), and also that I have a folder that centralises all my posts, meaning I don't have to worry about trying to fetch them from a folder on my computer on CI, which wouldn't make a whole lot of sense.

Also, how cool is it to have a CLI that lets you manage your own blog? Can I use this on my phone? Obviously!

*Note: this CLI needs you to have a script `export-blog-post` in your `$PATH` that will produce an HTML version of a file given as an argument.*

<details>
<summary>Example `export-blog-post` script for Doom Emacs</summary>

``` emacs-lisp
#!/usr/bin/env doomscript

(defcli! export-blog-post (&args files)
  (require 'doom-start)          ; load your user config
  (dolist (file files)           ; supports multiple files, if you want
    (find-file file)             ; open an org file
    (setq org-html-preamble nil) ; get rid of the preamble/postamble (we explicitly need these to *not* exist)
    (setq org-html-postamble nil)
    (org-html-export-to-html)))  ; and export it

(run! "export-blog-post" (cdr (member "--" argv)))
```

</details>

## Org has a publishing backend...

Yes, it does. But that doesn't let me create extremely detailed JSON files based on intricate tag processing to generate post series, categories, and papers, and separate them all.
