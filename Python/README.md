
### Python bootcamps

This series of exercises is a couple of Python bootcamps written by the members of the 42AI association, whose interests include statistics, AI, deep learning and machine learning. The [first bootcamp](https://github.com/42-AI/bootcamp_python) (most up-to-date branch [here](https://github.com/42-AI/bootcamp_python/tree/cloisonnement) as of 2023-02-24) teaches the fundamentals of Python; the [second bootcamp](https://github.com/42-AI/bootcamp_machine-learning) (most up-to-date branch [here](https://github.com/42-AI/bootcamp_machine-learning/tree/closing) as of 2023-02-24) teaches the fundamentals of data science and ML in Python (with numpy and pandas).

You are expected to look things up on your search engine of choice regularly. There are a lot of resources, in many languages, for C. Look around, find some that suit you.

To set up your IDE for Python (eg, Sublime Text), download the Python 3 package, the LSP package (as well as the NodeJS package, required for it ton run; your IDE should ask you for it if you have the rest), and the LSP-Pyright package.


#### Why Python

The reasons we think that learning Python is important are that:

   - Python is a very effective way to write scripts, often better than the Bash scripting language;

   - Python is heavily used in cybersecurity, in data science, and even data engineering and web development; ie, it is something for which it is easy to find a job;

   - Python takes aspects of object-oriented and functional programming in its design, and can be used to learn some concepts of these paradigms;

   - Python has a wide community, so you can find a lot of help online, and lots of useful packages that can make your life easier;

   - Python has a lot of syntactic sugar, and is thus very expressive and fast to write for small programs;

   - Python, despite being dynamic and weakly-typed, now supports type-hinting and type-checking, which alleviates some of the problems it used to have.


#### Rules

The below is mandatory.

   - for each "day" (series of exercises in the bootcamp), you will provide a series of tests which test the regular behaviors of your functions for that day, and tests for expected failure cases.

   - you will provide a way to type-check your code (we recommend `mypy` or `pyright` for a start).

   - for each "day" you will either provide:

      - a `RUN.md` file that says how you should run or type-check your code (worst),

      - a `run.sh` executable script that type-checks then launches your code (average),

      - a `Makefile` with an `all` rule, a `type-check` rule and a `run` rule (good),

      - a custom `Makefile` for your whole project, may have a `help` rule, a `prereq` rule, etc (best).

   - global-scope variables are forbidden, except:

      - within a specific `env.py` file common to a project;

      - within the scope of the `__main__` function (technically not global-scoped, but better to insist);

      - if these variables are actually type aliases.

   - all functions will be properly typed (type-hinted); this means that all arguments to functions should be typed (type-hinted), and the return type will be given explicitly, even if it is `None`.

   - classes should not be used unless absolutely relevant. In the vast majority of cases for beginners and even professionals, a simple series of functions suffices.

   - any custom type that is complex enough, and not a class, should be given a type alias.

   - any relevant constant should be made into a properly-named, scream-case, global variable in the `env.py` file.

   - except in "comprehensions" or ternaries, or in specific cases where vertical alignment improves legibility, you will have only one instruction per line.

   - default arguments to functions shall be kept constant (ie, never a list or dict as a default arg; instead use `None` as a default, and create a new list or dict within the function as needed).

   - processes the duration of which is known ahead of time, or have a simple iteration mechanism, will use `for` loops, processes where this iteration amount is unknown will use `while` loops. Remember: a good `for`-comprehension or a good `map/filter/reduce` is often better than an explicit `for` loop (at least in python).


#### Styling

The below is strongly advised, but not mandatory.

   - you will not use abbreviations in your variable or function names, except in the very rare cases where that can help legibility.

   - if you are not declaring the type of a variable speficically, you will name your variables according to some form of [Hungarian typing](https://en.wikipedia.org/wiki/Hungarian_notation).

   - variable names and function names will be written in `unix_case`. Type names will be written in `Snake_PascalCase`. Global variables will be written in `SCREAM_CASE`.

   - brackets, curly braces and parentheses over multiple lines will use the Allman style, wherever this does not cause a syntax error in python.

   - indentation will use "1 tab per level of nesting" (where each opening parenthesis, bracket, or curly brace adds one level of nesting);

   - vertical alignment of instructions will be used where appropriate, an "inappropriate" use of vertical alignment is one where writing a small function and calling that is better. Vertical alignment will use spaces in the middle of a line.

   - variable declarations will be done at the top of the relevant scoping block.

   - an ideal function contains somewhere between 1 to 15-20 instructions. Functions that "are better kept long" are *very* rare. If your function is too big, it should be subdivided into smaller, appropriately named functions. In Python, these can very well be internal functions.

   - where a variable can be shadowed, it should instead be passed as an explicit argument.
