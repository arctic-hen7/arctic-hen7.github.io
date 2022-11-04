# CLI

You would be very justified in asking why on earth my blog has a command-line interface. The answer is, well, I like to automate things. Since my blog imports all its posts from my Zettelkasten (the concept of which I have brutally abused to create a flat file system that basically stores my entire digital life), and decides which files shoudl be public based on a tags system, I like to be abel to control when posts are added, when they're updated, deleted, etc. This also provides a system for converting posts into HTML *before* they reach the Perseus build process, which means builds are faster (since rendering my posts to HTML requries calls to LaTeX, Emacs, etc.), and also that I have a folder that centralises all my posts, meaning I don't have to worry about trying to fetch them from a folder on my computer on CI, which wouldn't make a whole lot of sense.

Also, how cool is it to have a CLI that lets you manage your own blog? Can I use this on my phone? Obviously!

## Org has a publishing backend...

Yes, it does. But that doesn't let me create extremely detailed JSON files based on intricate tag processing to generate post series, categories, and papers, and separate them all.
