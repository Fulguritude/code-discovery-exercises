# Code Discovery Exercises

This is a repo containing exercises for aspiring developer, and people who want to broach new languages.



## Getting started

You might be starting out with code and want to increase your mastery and hirability, you might also come from another background (such as business analysis) and want to get acquainted with programming. In any case, welcome !

The point of this repo is to teach you the following things:

- **Using `git`**, and the collaborative GitLab workflow in particular. For starters, you can find a [small guide to using git in the `LEARN2GIT.md` file](https://github.com/Fulguritude/code-discovery-exercises/-/blob/master/LEARN2GIT.md). You might also want to [check out the `CONTRIBUTING.md` file](https://github.com/Fulguritude/code-discovery-exercises/-/blob/master/CONTRIBUTING.md), to learn how multiple software engineer friends, colleagues and I try to clarify our commit messages.

- **Using the Unix shell (the terminal)**, and the standard commands that you should know and use. You can find a small guide to using the terminal (or [cygwin](https://www.cygwin.com/), or [WSL](https://docs.microsoft.com/en-us/windows/wsl/about) if you *really* want to make your life more difficult and develop on Windows rather than Linux or Mac) in the [`LEARN2TERM.md` file](https://github.com/Fulguritude/code-discovery-exercises/-/blob/master/LEARN2TERM.md).

- **Writing code in various languages**: typically C and/or Python code for beginnes (we'll certainly try to add more bootcamps as time goes on), specifically in a way that is compatible with the "best practices" of collaborative software engineering. This includes: 

   - structuring and organizing the files in a project;

   - using scripts or build tools (such as Makefile or cargo) to aid with compilation or launching;

   - typing (or type-hinting) your code cleanly;

   - following, what we believe, are styling conventions that help legibility;

   - dividing your code into easy-to-read atoms (eg, "one instruction/expression per line", "writing small functions (everything from 5 to 20 instructions/expressions ideally)");

   - writing clear (concise, well-structured/namespaced, consistently styled, non-abbreviated) variable and function names;

   - etc.



## Words of advice

Whether you're here out of personal curiosity; or you're an intern at their first job, feeling like an impostor: you need to know one thing. You might be scared, you might be excited, you might rely on old experience. Whatever expectations you have, note the following: this is not school. You're not learning in order to get more points or more bananas like a trained monkey; you're not learning for the authority figures in your life, your parents or teachers. You are learning for *yourself*. You are improving *your own head*, to become more competent, to build up *your* life.

This is the philosophy we encourage for learning here.

That being said, you shouldn't approach the exercises here with a mentality of "doing everything as fast as possible to get to the cool stuff". If that's what you think will help you get ahead, and just copy-paste someone else's code, you won't learn anything.

Additionally, if you're looking for a job in CS or development and this is how you learn, your colleagues will definitely see that your work is poor when you start actually writing production code and they need to clean up after you. Finally, seeing a *big* difference between the quality of your work in these exercises or your portfolio *vs* the quality of your code in production will lead to one of two very undesirable things for you. It will either make your company think that you'll have difficulty being creative, adaptable and autonomous in your code: they will think that you need training wheels forever which will reduce your evolution prospects, since they won't feel comfortable delegating important tasks to you. Or worse, it will make you lose their trust, and that'll generally lead them to showing you the door.

Instead, take the time to deeply learn as many new things as you can. Take notes of what you've learned. No matter the way you've coded something, there's necessarily aspects that are good about it, and aspects that are bad (or at least could be improved). Ask yourself what these are and take regular (daily ? weekly ?) notes. Learn to evaluate (analyze the good) and criticize (analyze the bad) in your own code. This is a period where people consider that you're allowed to make a lot of mistakes: make full use of this opportunity to learn, it might not come again ! There's a saying at 42, the "school" where I learned to program (I didn't learn much about code in uni, mind you): "*cheating* means *not being able to reproduce on your own*" - this means that you should never hesitate to take inspiration from others and learn from them, but you should definitely learn things well enough to "own" them yourself. It's why 42's educational program (and the C piscine below) is mostly about "rebuilding the wheel": it helps you learn the fundamentals from scratch.

Finally, and importantly: have fun ! :) Learning is *deeply* satisfying, when you do it for yourself. Seeing yourself become able to accomplish, or perceive, all the things that you couldn't before is essential for your mind and soul. And for any society, organization or company to grow, innovate, and stay ahead of the curve, having fulfilled, passionate, creative individuals is essential. Don't hesitate to come ask for help or clarification in the issues, or to ask for help on various programming language Discords, or Stack Overflow, etc : it's not like we didn't struggle at first, or that we don't still struggle regularly.



## Exercises: rules, norms and indications

### General rules

Here are a few general methodology rules:

   - **commit often, commit small things**. "One big commit of all your files per day", or worse, "per week", is not acceptable. Making "atomic" commits makes the review process simpler. For complex exercises, it can help reviewers see your thought process, and your progress, which can lead to more helpful feedback on your code. Also, it'll prevent losing code that could be interesting if you're following various leads and are unsure which is the best. Having a rich code history is invaluable.

   - if you're stuck, don't get demoralized, instead go **ask for help**.

   - these bootcamps are a work-in-progress. We've recycled them from other places. For those who have been invited to fork this repo and get personal reviews, take the comments we make on the reviews as the most important source of feedback for you, and apply them. If you don't understand the feedback, say that you don't understand and ask for examples. Note that we aren't going to be as rigid as we could be (like if we had a more "automated" review process).

   - **make a lexicon**. You are going to learn a lot of new vocabulary, and you should deeply understand all these new concepts. We have provided a [`LEXICON.md` file](https://github.com/Fulguritude/code-discovery-exercises/-/blob/master/LEXICON.md), which can help. However, not all of it will be useful to you, so don't hesitate to make your own, use your own words... and push it to your own folder on the repo ! :)

   - read, re-read, *re*-re-read, etc, the documentation. You certainly don't have to swallow it all in one go, but use documentation as its meant to be used: ie, consult it regularly. Read this `README.md` multiple times, go over the various guides in this repo again and again and again. There will be things that you missed the first time or things you couldn't or didn't understand at first. Revisiting these documents will help you considerably.

   - almost any place where you see yourself copy-pasting your own code should instead be a place where you create a subfunction and call it in different ways. We will be strict on this rule. Code should be *DRY* ("Don't Repeat Yourself").

   - if there is anything that's unclear, or poorly written, etc, in this repo, or in our documentation in general: point it out. You can do so by raising an issue.



### Types of exercises

For now, we have:
- the 42 C piscine,
- the 42-AI Python bootcamps (both general Python and Python for data science and machine learning),
- some custom Rust cheat sheets which will present essential concepts of the language, as well as the polars data science library, and a link to Rustlings.

They can prove very useful if you're looking to broaden your skills.

Of course, don't forget to read, and re-read, the documentation provided ! There are a lot of useful concepts and tools in there. Look in the relevant folders for more information.



## How to turn in your exercises

Fork this repo. You can then do whatever you want with it. Then send us a link.




## TODO

- give code examples to make the "styling" sections more understandable
- add an OCaml section for functional programming
