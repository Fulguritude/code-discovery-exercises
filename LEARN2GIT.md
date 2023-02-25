Not everyone on a team starts with experience using git-based version control. However, it can be useful to know how to use git, even for non-developers. So here is a quick intro to git in a nutshell, from absolute beginner to useful professional competence. Feel free to ask any questions, add what you think is relevant, or point out any mistakes. You can do so by creating an "issue" on this repo; try to figure out how (look for "Issues" on the web page) !

Git is a powerful piece of software that allows the writing and archiving of text (including code, of course; and also raw data, to some extent) in a collaborative manner. Git's key features include: defining the current state of the text as a series of per-line changes since the beginning; analyzing changes line-by-line; keeping different versions of the same text synchronously ("branches"); being able to revisit previous steps in the text's history; etc. Since code is text that is pretty "organic" in how it evolves (growing, shrinking, cutting, adding), it's essential to be able to keep old work if some "fix" or "improvement" actually causes problems, or wasn't the right lead to follow.


Git, for a beginner, can basically be summarized in a dozen commands.

First things first, some vocabulary and context. There is a "repository" (a deposit of code, and the history of changes to that code) containing some code (or which is initially empty), online, that you want to copy into you local computer. For this, you'll `git clone [address] [local_path]` to make a local version of a remote git repo into the `local_path` folder of your computer. This can be done via HTTPS or SSH: SSH is safer, but will require you to create an SSH PPK pair (Public/Private Key pair) and to set it up on your GitHub/GitLab/BitBucket/etc account.

Now, you've got the local repo/archive on your computer, and the remote one on GitHub (or GitLab, or BitBucket...). You've also made a couple of changes to the text/code files locally.

The simplest thing you can desire to do, now, is send your code to the repo on the remote server.
This is generally done via:

```bash
git add [my file names]
git commit -m "my commit message explaining what I just did"
git push
```

- `git add` says to your local git repo "in the next commit, I want you to keep track of these files"

- `git commit` says "write the differences to the text, line-by-line, that these changes caused to the local archive, compared to the state of the text at the previous commit"

- `git push` says "send the local archive current state to the remote archive online"

If ever you're wondering what's happening at any point, `git status` will give you a list of files, with info, and some other important info. The file info consists of 3 lists: a list of files that were added ("staged for commit"), and 2 lists of files that can be added but weren't (files "not staged for commit"; they can be *tracked*, ie, already added in some previous commit, or *untracked*, never added to any commit in the past).

If, *before* adding, you want to check what you did (because you don't remember exactly what changed), `git diff` will show you what lines were changed. *After* adding, you can use `git diff --cached` to see the changes in the added files.

NB: `git status` and `git diff` are probably the commands I use most often, personally.

That's basically it for "managing a git repo on which you are the sole contributor, when coding on a single computer".

A few additions and tips for basic git usage:

- `git add --all` will add everything (even untracked files, ie, those that were recently created and never yet committed). Useful for adding a bunch of new, thematically related, files. Be careful not to abuse it and unwittingly add some secure information like a private key or Personal Access Token in a `.env` file...

- `git commit -am "my message"` will add everything that is tracked (whether added or not-added; ie, everything but new files that were never committed yet) and commit, in a single command. Useful for small changes to a single file (or small amount of files).

- `diff-so-fancy` is a small plugin that you can use with `git` that makes `git diff` a LOT more legible, I highly recommend installing it (`apt` and `brew` both have it, IIRC). Once installed, you can use it as `diff -u [file_or_directory_old] [file_or_directory_new] | diff-so-fancy | less -r` on arbitrary files. There's some "git config" setup to use it globally with `git` on your machine (see the [`diff-so-fancy` github page](https://github.com/so-fancy/diff-so-fancy/) for more info).

- `git log` will show you a list of previous commits. `git diff [LOG_HASH_ID] [file_paths]` will give you a diff between the current version and the logged version referenced by `LOG_HASH_ID` for the files or directories specified by `file_paths`. This is VERY useful when trying to see where a problem might have come from, after some multi-day work that changed a bunch of things.


As for using git when collaborating, you'll inevitably run into conflicts when trying to push, because someone else pushed before you. Here we have our last "very important" command, `git pull`.

`git pull` gets the remote code and tries to apply the new differences to your local code. If there are no conflicts, you'll get an Emacs/Vim/Nano prompt with a message saying that the merge was successful (quit Vim with Escape to enter the vim command mode, then `:q` or `:wq` or `:q!` ie, quit/write+quit/force quit). If there are conflicts, your terminal will tell you the file where auto-merging failed. There are two possible modes to resolve conflicts during a pull: *merge* and *rebase*. *Merging* is all about resolving all the changes at once, by going into the code file-by-file and choosing how to rewrite the code, based on a comparison the two versions where there's a conflict. *Rebase* instead walks you through potential conflicts one *commit* at a time. At first, I'd suggest doing `git merge` as a default. Go into your files, fix the conflicts in the code, **make sure it compiles, that you can run it, and that you thus didn't remove some important new addition**, and add/commit/push.

As for branches, they basically allow you to keep committing and pushing (so you can keep your work online/show your progress, etc), while never causing problems for the `master` branch (where the working build is kept, well, working). This becomes essential in very large repos where multiple teams are working on different submodules or features together.

For this you have `git checkout -b [new_branch_name]` to create a new branch and `git checkout [some_branch]` to move the git head to another branch (this'll change your local code to that of the other branch, beware if your code is currently open in a text editor, you might get unwanted behavior).

Branches lead to pull requests. Pull requests should be understood as "I, dev peon, am politely requesting that you, dev master, do a `git pull [some_branch]` on my branch, from the master branch, to integrate the changes from my local branch into the master branch". In most organizations, multiple devs review the code before branches are pulled into master.

As a side note, you also have `git stash` to remove all changes you made to the code since the last commit. The stash (consisting of a single layer, or multiple layers, of uncommitted changes; one layer per call to `git stash`) is kept until you delete it. You can get it back via `git stash pop` (removing the last added layer in the stash) or `git stash --apply` (re-affecting the changes to the code, but keeping the last layer on top of the stash). Note that this may lead to a merge conflict, which you'll have to resolve.

Beware of stashing things that were added but uncommitted, or pushing from the wrong branch... It can create annoying messes that can be thorny to get out of.

Final note on good practices: avoid using `git add --all` or `git commit -am "my message"` if your commit is large (applies to many files, changes many lines...). It's something I personally have trouble with because I code very fast, and on many things at once as I notice them. But it does help to do the effort of going through `git diff`, `git add`-ing file-by-file, `git diff` again, etc. This can help when lots of changes were made, and you're looking to commit in coherent batches of files with a common "theme". The goal is to fragment your commits into as small segments as possible, with helpful labels (commit messages). It can be annoying to do multiple commits, but your stats will be better (for (dumber or busier) employers that only look at commit amount+regularity rather than content), and most importantly it makes code a LOT easier to review. I mean, a LOT. This can be helpful for your superiors (whether you ask for help or are attempting a pull request), for maintainers, and for yourself. You never know when you'll need to do a `git diff` between the current version and a _biblically old_ log for a specific file; and you'll thank yourself for taking the time to do atomic commits.

[See the `CONTRIBUTING.md` file](https://github.com/Fulguritude/code-discovery-exercises/-/blob/master/CONTRIBUTING.md) for more details on good practices (in particular, commit message styling).

There's also, of course, the questions of reverting code to a previous build, of pushing with argument to a specific branch (rather than the current branch), of rebase rather than merge, of fetching, etc. But I'll let you google the rest; what I've listed is more than enough for everyday use, and should get you well on your way to using git in a way that helps you (a lot) rather than hinders you.

A final note is cloning the submodules (other repos) on which a given repo depends. For this, you can either go in the submodule's folder and run `git clone [my submodule ssh or https address] .`, or `git submodule update --init --recursive`.
