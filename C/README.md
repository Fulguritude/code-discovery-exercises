### C piscine

This series of exercises will be loosely based on the 42 school's C "piscine". This is a monthlong bootcamp that serves as an entry contest to the school. We will be ignoring the time limit, and only doing the "days" (ie, a specific kind of independent series of exercises), but doing them fully. The first 2 days are used to get acquainted with the Unix terminal. The rest of the 14 "days" teach you the fundamentals of C, and with, important fundamentals of general programming, as well as key components of imperative programming in particular.

You can find the subjects for the C piscine [here](https://github.com/Binary-Hackers/42_Subjects/tree/master/01_Piscines/C/EN). These subjects also exist in French (and various other languages), but we require you to write all your code, variable names, comments, etc, in English.

You are expected to look things up on your search engine of choice regularly. There are a lot of resources, in many languages, for C. Look around, find some that suit you.

To set up your IDE (eg Sublime Text) to use C, you'll want to download the "C Improved" and "EasyClangComplete" packages.


#### Why C

The reasons we think that learning C is important are that:

   - C is a typed language, and the concept of "type" is the most important in computer science after "computation" itself;

   - types in C reflect how a computer encodes data into its raw memory, and this knowledge of "encoding" is an essential piece of general culture to have when considering a lot of (really, almost all) advanced subjects in computer science and software engineering;

   - C is a compiled language, and this can be used as a good basis for learning how to use the terminal, scripting, Makefile, and binary linking;

   - a lot of programming languages inherit parts of their design from C, or can be interfaced with C, since C is one of the oldest and most prevalent languages in history - C is the Rosetta stone of programming;

   - it can teach various fundamentals of general computer science, and is a (*the*) fundamental example of the "imperative programming" paradigm.


#### Rules

Now, here are a couple of rules to follow. Note that you won't understand all of these rules immediately, but should understand them by the end of the bootcamp.

   - for each "day", you will provide a series of tests which test the regular behaviors of your functions for that day, and tests for expected failure cases.

   - for each "day" you will either provide:

      - a `COMPILE.md` file that says how you should compile your code (worst),

      - a `compile.sh` executable script that compiles and launches your code (average),

      - a `Makefile` with an `all` rule, a `clean` rule, and a `run` rule (good),

      - a custom `Makefile` for your whole project, that only recompiles when necessary, may have a `help` rule, a `test-memory` rule, etc (best).

   - your programs will contain no leaks. If your tested C function allocates memory, the tests should free it. You can use `valgrind` or `libasan` to test for leaks.

   - global variables are forbidden.

   - all variables will be set to their type's appropriate NULL value at declaration.

   - compilation will use the flags `-Wall`, `-Wextra`, `-Wpedantic`, `-fstrict-aliasing` and `-Werror`.

   - compilation will first compile `.c` source files into `.o` object files before making an executable.

   - `.h` files should be used as intended: as specification "contracts" for the rest of your code, and contain all relevant type declarations and function declarations.

   - you will make use of `typedef`s in your `.h` files to make your code more semantically legible.

   - any relevant constant should be made into a properly-named macro.

   - except in the header of a `for` loop, or in specific cases where vertical alignment improves legibility, you will have only one instruction per line.

   - processes whose duration is known ahead of time, or that have a simple iteration mechanism, will use `for` loops, processes where this iteration amount is unknown will use `while` loops.


#### Styling

   - you will not use abbreviations in your variable or function names, except in the very rare cases where that can help legibility.

   - variable names will be written in `unix_case`. Function names will be written in `Snake_PascalCase`. Macros will be written in `SCREAM_CASE`.

   - the two rules above are more important than the indicated function signatures in the PDF.

   - type names should have appropriate prefixes:

      - typedefs will be prefixed with `t_`,

      - struct names will be prefixed with `s_`,

      - union names will be prefixed with `u_`,

      - enumeration names will be prefixed with `e_`,

   - brackets, curly braces and parentheses over multiple lines will all use the Allman style.

   - indentation will use "1 tab per level of nesting" (where each opening parenthesis, bracket, or curly brace adds one level of nesting;

   - vertical alignment of instructions will be used where appropriate, an "inappropriate" use of vertical alignment is one where writing a small function and calling that is better. Vertical alignment will use spaces in the middle of a line.

   - variable declarations will be done at the top of the relevant scoping block.

   - an ideal function contains somewhere between 1 to 15-20 instructions. Functions that "are better kept long" are *very* rare. If your function is too big, it should be subdivided into smaller, appropriately named functions.


#### Extra indications

The Kerberos / LDAP exercises (d00, exercises 03 to 06) are to be ignored.

For d09, only the exercises 04, 05, 06, 07, 08, 10, 11, 12, 13, 16, 17, 18, 19, are interesting to do, the rest you can safely ignore.
