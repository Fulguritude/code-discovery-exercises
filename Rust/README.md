#### Why Rust

The reasons we think that learning Rust is important are that:

   - Rust is a very performant and safe language. It is the safest (memory-wise, type-wise) of imperative programming languages (along with Ada), and the only language with equal performance to C.

   - Although rather young, it has taken the software engineering world by storm. The Rust ecosystem is growing fast, and is much larger than that of Ada. Many things that were for decades written in C or C++ are being ported to Rust. In fact, there are a bunch of data engineering and data science libraries for Rust, even though front-end GUI libraries are pretty young and imperfect (as of 2022).

   - Rust is an imperative programming language at heart, but takes decent inspiration from functional programming (like pattern matching, and strong typing). Its most innovative construct is that of memory "ownership": a way for the strict Rust compiler to handle, at compile-time, the allocation and deallocation of memory. It is how Rust was able to become such a safe yet performant language.

   - Although a strong compiler like Rust's is often very frustrating at first, it becomes quickly preferable to debugging. When a program with a strong compiler passes compilation, generally, it'll run as expected. Additionally, Rust's compiler is pretty chatty and helpful (compared to many other languages).

   - Rust's community is already pretty large, and quite helpful, so you can find a lot of help online. There are lots of useful packages that can make your life easier. Most Rust crates (packages) are pretty well documented.

   - Sadly, Rust is often considered pretty ugly/verbose. However, things like good styling practices and macros can help alleviate that, somewhat. However, the online code examples and sources can be difficult to read without some practice, or without reformatting the code into something legible. Having the Rust linters (eg, rustfmt) improve, in the future, should help with this.

   - All-in-all, Rust is an excellent language, but is still quite young and will still grow. It will probably continue to improve for the better as time goes on.



#### How to install & getting started.

Check the `Rust cheat sheet` section of this directory for more info and language examples. You can also check the page https://learnxinyminutes.com/docs/rust/.

On your IDE (for Rust, we recommend Visual Studio Code, and to a lesser extent Sublime Text 3; if you're a Vim/Emacs/Neovim guru, you can probably set up what you want), you also want to download the following extensions (VSC) or packages (ST3):

   - rust-analyzer (available on VSC) or LSP-rust-analyzer and Rust Enhanced (available on ST3)

   - crates (available on both VSC and ST3)

   - CodeLLDB (available on VSC)

   - Even Better TOML (available on VSC) or TOML (available on ST3)



### Rust exercises

For beginner exercises, we recommend [Rustlings](https://github.com/rust-lang/rustlings).

Once you're done with these, you should try making a small Rust project of your own, say something like rendering some Escape-Time Fractals, and displaying them with SDL. Visual things make for good portfolios !



#### Styling

For now, we do not have an automated code styler/linter in Rust, and we consider `rustfmt` to be too imperfect for practical use at this point in time.

Instead, here are a few guidelines to keep in mind when styling your code.

   - All of these guidelines are optional, and many go against usual Rust styling conventions. We do consider that they make things more legible than the usual conventions, and recommend them for learners and professionals alike.

   - Except in the most obvious of cases, it is better to explicitly type your code. This makes it more legible and more strictly checked. Some IDEs have tools that allow you to see what type is inferred at each line, which can also be useful. If you dislike typing explicitly because "it makes changing a type throughout your code more time-consuming", read the following point.

   - It is **HIGHLY** encouraged to alias your types (or use the `newtype` Rust pattern) to give a type some semantic meaning: after all, even a simple `integer` (`i8`, `u32`, etc) can mean a LOT of different things. It is also encouraged to define new aliases using old semantic-rich type aliases as components. This makes code a lot more meaningful and legible, and it also makes it so that type refactors are much less time-consuming, and much easier to do well, even when the code is explicitly typed everywhere.

   - Except for very small chains, you will have only a single instruction per line. This rule forces you to divide your code into smaller steps, and forces you to name each intermediary step. This makes a lot more debuggable, understandable, and legible.

   - Wherever applicable, constructs will be vertically aligned. This includes, but is not limited to: the arguments in a multiline function signature or call; enum declarations; match statements; multiline struct declarations or definitions; imports; etc. The role of vertical symmetry is to make immediately visible differences and commonalities between successive lines of code.

   - The ideal length of a function is somewhere between 1 to 10 instructions. More is acceptable if it really helps to have the code of a given function be more legible. However, well-named internal functions are a very useful construct to use, in order to make the code of any bigger function more legible.

   - When returning multiple values from a function, and above 3 values are returned, a well-typed struct/dict/key-value-pair -like structure will be returned, rather than a tuple. This prevents ugly tuple unpacking on multiple lines, and is much clearer semantically, since the key constrains which argument is unpacked where in a legible manner.

   - Lines of abusive length should be avoided. An exception to this rule is for a series of function calls to the same function that has a lot of arguments, and where these calls are better understood vertically aligned. If such a series of vertically-aligned statements is very long, consider whether each line can be shortened by writing a small internal function.

   - Anything that is listed should be aligned vertically for 3 values and above, except if the character length of these values is very small. This philosophy also includes multiline strings, where `concat` can be used to better show scoping. 

   - Indentation will use tabs. This is mostly so that anyone in their own IDE can set the length of tabs to whatever number of columns they desire.

   - While any indentation before text is done in tabs, vertical alignment after the first non-whitespace character should be done with spaces. This allows text to be vertically aligned no matter the choice of column amount per tabs, and to simplify the refactor or large vertically aligned lists.

   - Code blocks defined within brackets, parentheses, and curly braces will all follow the Allman style: opening and end symbols vertically aligned, and within, the amount of indentation defines the level of scoping. This gives better readability to scoping blocks, which are better visually isolated by empty spacing. In some cases, where the content of a block is very small (like one or two instructions), and/or if this can serve vertical alignment, one may sometimes ignore the "single instruction per line" rule and have a the block on a single line.

   - "return via a single expression" will be replaced with an explicit return statement. This enforces consistency in functions with multiple return statements. An exception can be made for lambdas, or functions which contain a single expression.

   - Binary operators will be surrounded by at least one space on each side.

   - If a chain is too long to fit on a single line, you will have one method call per-line (with eventual vertical subblock for the arguments), and each method be aligned at one extra tab from the start of the chain.

   - Lists, match statements, etc, should have trailing commas when written vertically.

   - Where applicable (ie, after a `break`, `return`, or `continue`), you will add a trailing semicolon.

   - In any file, imports will be divided into 3 sections, according to the `StdExternalCrate` format:

      - the first section contains imports from the standard library, followed by an empty line;

      - the second section contains the 3rd party dependencies, followed by an empty line;

      - the last section contains the imports from the local project/crate, followed by 3 empty lines.

   - Hexadecimal literals will have letter symbols be uppercase.

   - The word "number" is forbidden in variable and function names; prefer "index", "amount", "measure", "id", etc, which are more precise.

   - If you have dyslexia, consider setting the text font of your IDE to Comic Sans (this is not a joke, the asymmetry of the letters will make it easier for you to read).



#### Extra indications

This will be hard in the beginning. Set your IDE up well, rely on it, go back to the cheatsheet regularly, and ask for help !
