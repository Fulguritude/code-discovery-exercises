
## Why OCaml

The reasons we think that learning OCaml is important are:

   - OCaml is a programming language that is very much in the functional paradigm, but has a couple of features that it inherits from the imperative and object-oriented paradigms. For this reason, it's often used as people's first true, practical discovery of functional programming. Indeed, a lot of things which are much more straightforward in imperative and OOP, such as `for`-loops, are greatly deterred by the language's syntax and its standard library. It'll naturally force you to gravitate towards "functional patterns", such as map/filter/fold, internal functions, partial application, recursion, generic modules as functors, etc.

   - Though building full-on OCaml projects has been complex for a while, the build system has been improving with time (ie, `dune`). This is still imperfect, and somewhat hard for beginners, though.

   - The language has a lot of high-level features and is very secure, mathematically. It's not every language in which the build system itself takes into account the fact that you'll be generating code from strings and compiling it immediately, and that this is normal procedure and needs to be done safely and simply. It's really a gateway into advanced code and math (notably through the theorem prover `coq`).

   - Since it's pretty niche, it's actually pretty easy to find an OCaml job, since the companies that are the source of the demand is ready to hire people they'll be willing to train somewhat. However, this does also imply that the community is small, and it's harder to get help if you're working alone and you're stuck.

   - There are probably even "better" production functional languages when it comes to ease-of-build, lots of available libs, language maturity, community size, frequency of use in industry, etc. Scala is probably the best contender here. However, OCaml is probably a simpler, more straightforward way to start learning functional programming.



## Tutorials

### First steps bootcamp

We recommend the [42 OCaml piscine](https://github.com/Binary-Hackers/42_Subjects/blob/master/01_Piscines/OCaml) as a start.

Here's a brief summary of what you'll learn

   - D00: basic syntax through standard "programming 101"-style exercises in OCaml
   - D01: intro to recursion in functional programming (and it's much more important here than for other paradigms)
   - D02: "programming 101" exercises, but with more kick to them (discovery of native data types such as List, and concrete ("union/enum") types, more advanced match patterns, internal functions, etc.)
   - D03: entry-level graphics programming, using external packages, compilation, some tree-based algorithmics, etc.
   - D04: using internal packages, code structuring, complex "class-like" types.
   - D05: file IO and imperative-style OCaml (including Arrays), a hint of ML/stats algorithmics.
   - D06: modules and functors
   - D07: object-oriented-style OCaml (classes)
   - D08: more object-oriented-style OCaml, making class-based modules
   - D09: monoids and monads
   - Rush00 & Rush01: small, but larger, scale projects (completable in a weekend)


### 42 pedagogical projects

There are other OCaml projects from 42 that might interest you: https://github.com/Binary-Hackers/42_Subjects/tree/master/00_Projects/05_OCaml


### Personal project

TODO



## Setup

We'll go with a Linux / WSL (Windows Subsystem for Linux) description of the installation, as well as setup for Sublime Text.

First, you'll want to download the OCaml language.

```bash
sudo apt install ocaml
```
Then, you'll want to install the OCaml package manager (called OPaM)

```bash
sudo apt install opam
```

Then, you'll want to install the OCaml build util (called Dune).
```bash
opam install dune
```

Then, you'll want to install an OCaml LSP server (more instructions and details [here](https://github.com/ocaml/ocaml-lsp) ). In short:

```bash
opam install ocaml-lsp-server
```

Finally, to have sublime recognize it, find the path to `ocamllsp` with `which ocamllsp` (generally, the path should be `~/.opam/default/bin/ocamllsp`), then add the following to your LSP settings (`Ctrl+Shift+P` then write `LSP settings` and press enter; edit the file on the right; if it's empty you can just copy paste the below over it; otherwise you'll need to add the "`ocaml`" element to the "`clients`" dictionary).

```json
// Settings in here override those in "LSP/LSP.sublime-settings"
{
  "clients": {
    "ocaml": {
      "enabled": true,
      "command": [
        "/absolute/path/to/ocamllsp"
      ],
      "scopes": [
        "source.reason",
        "source.ocaml"
      ],
      "syntaxes": [
        "Packages/Reason/Reason.tmLanguage",
        "Packages/sublime-reason/Reason.tmLanguage",
        "Packages/OCaml/OCaml.sublime-syntax"
      ],
      "languageId": "ocaml"
    }
  }
}
```


## Helpful pages

https://ocaml.org/docs/ ; the official language docs are pretty great, and you should probably read them in full, it'll teach you a lot about OCaml and functional programming in general.

https://dev.realworldocaml.org/toc.html ; probably the best / most thorough OCaml textbook out there.

https://cs3110.github.io/textbook/chapters/preface/about.html ; another online textbook on OCaml, goes more in depth, if you want to go further with the language.

https://stackoverflow.com/questions/9997822/ocaml-module-include-and-open ; how to manage projects and imports.



## Rules

The below is mandatory.

   - **You will explicitly type your function's input and output types.** Yes, OCaml has type inference; no, this does not lead to immediately legible and maintainable code; yes, you want to type explicitly, especially when it comes to things you'll use to compose, or which you'll expose to other programmers. Only trivial lambdas (eg `fun x y -> x ^ " " ^ y`) can be excepted from this rule. Typing variables within a function is unnecessary if each sub-operation is neatly assigned to a well-named intermediary variable.

   - **Every `in` keyword, every semicolon `;`, and every pair of semicolons `;;` should be followed by a newline.** A possible exception is for tiny `print_endline(); let ... in` or `let ... in let ...in` expressions that can be aligned vertically and would be less legible if converted to calls of a two-line internal function (quite rare).

   - **You should only have one (complex) expression per line** (ie, you should create a bunch of intermediary variables). By "complex", we mean that some exceptions can be considered for trivial expressions (ie, things like `n - 1`, `f x` or eventually things like `fun x y -> x ^ " " ^ y`, so long as the rest of the expression which contains these isn't too complex).

   - **You should scope multiline `let ... in` (resp. `let .. ;;`) definition statements properly.** This means you will have the final `in` (resp. `;;`) be alone, on its own line, and vertically aligned with the corresponding `let` keyword. The content of the definition will display the scope with an extra indent. This applies to `type` rather than `let`, as well.

   - **You will vertically align the arguments of function with many args.** If the number of input args is 3 or above, they will be aligned vertically with an extra indent, vertical alignment of colons and parentheses, and the return type annotation will be on its own line, unindented, aligned with the `let` keyword of the function declaration.

   - **You will give breathing room to visually distinguish subcomponents of every expression:**

      - every comma `,` should be followed by (at least) one space.

      - every colon `:` should be followed by (at least) one space. Whether a colon is preceded by a space is optional, but is mandatory in the case of multiple vertically-aligned colons.

      - function arguments, even when surrounded by parentheses, should have a space between them. Uncurried functions (or single arg) that return `unit` may ignore this (because this looks like C, and thus helps emphasize the fact that this is an IO function with side-effects, which is generally the case for such functions).

      - every operator should be surrounded by 2 spaces, except in the case where the operator itself is used as a function argument (ie, using concatenation as an example, you'd have spaces with `"a" ^ "b"`, but no spaces with `List.fold_left (^) ("") (str_list)`).



## Styling

The below is strongly advised, but not mandatory.

   - prefer modules to classes.

   - explicit terminating `;;` even if they are unnecessary.

   - avoid non-obvious abbreviations as much as possible (eg, `acc`, `f`, `x`, `h`, `t` are allowed, but generally, if these are not generic `'a` types, some semantically relevant name (or hungarian type-hinting) should be preferred, such as `intlist_h`).

   - match statements that contains multiple arms (ie, anything other than unpacking a tuple or list into component parts) will start with the `match` statement on a new line with an extra indentation. Conversely, an "unpacking" match statement should have everything from the `match` keyword up to the `->` on a single line, unless this makes reading it too complex (though the simpler `let (arg1, arg2) = my_pair in` syntax is of course preferable in the vast majority of cases).

   - vertical bars (pipes, `|`) after a match statement will be aligned vertically at the same indentation as the `match` keyword.

   - any complex `match` statement (multi-expression match arm, match arm lengths of diverse size, etc.) will scope their internal expression sequence in a pair of parentheses, in the Allman C style, with the internal expression sequence having an extra indent, compared to the `|` symbols and `match` keyword.

   - similarly, complex `if ... then ... else ...` sequences should be "indented and scoped with parentheses" in the Allman C style mentioned above. Trivial ternaries can be kept on a single line.

   - Arguments to functions that are in the "curried" style (ie, the conventional OCaml style for function declaration, eg `let f (x: 'a) (y: 'b): 'c = ... ;;`, rather than the "uncurried" style, eg `let f ((a, b): ('a * 'b)): 'c = ... ;;`), when:

      - some of these arguments need to be surrounded by parentheses, or

      - some of these arguments are function calls,

   then the arguments should *all* be surrounded by parentheses, in order to visually isolate the amount of args and the level of expressions, even if these parentheses are unnecessary. Eg: you will write `test (f) (n - 1)` rather than `test f (n - 1)`; however, writing `List.map (f) (l)` rather than `List.map f l` is optional.

   - Uncurried function are to be mostly avoided. If used, uncurried functions shall have only a single argument. If an uncurried function has more than 3 arguments, its variable names will be aligned with their respective type on the next line.

   - when all match arm output expressions are small, the `->` symbols will be aligned vertically.

   - internal functions and value declaration blocks will always be followed by a single blank line.

   - enum/union type declarations with 4 or more variants will be vertically aligned and indented with a leading pipe `|`. This also applies to enums/unions with less variants, but with complex `of` type declarations for certain variants.

   - in sequences of single-line `let ... in` statements, the assignment equal signs should be aligned vertically, and if the lines are very similar, the `in` keywords should be as well.

   - if a function call gets too long, consider multilining it (with each arg on a new line and with an extra indent compared to the function name itself), or storing its content into intermediary variables.

   - when dealing with mutually-recursive functions (ie, declared using the `and` keyword, have the `let rec` be on its own line, aligned with the `and` keyword and have the internal functions indented.

   - in a module, separate your functions by 2 blank spaces.


## Rule and styling example

The below is a semantically meaningless function that shows various points mentioned above.

```ocaml
(* more than 4 args, so vertically aligned with leading pipe *)
type cases =
	| Case1
	| Case23
	| Case4
	| Case567
	| CaseThreeDigits
	| CaseOther
;;

(* more than 3 args, so vertically aligned with alignment of colons and parentheses *)
let example_function
	(* having a space between the parenthesis and the variable is optional, all that matters is that they are vertically aligned; I tend to add such spaces as they make multi-cursor usage simpler in IDEs when formatting *)
	( n1      : int          )
	( n2      : int          )
	( n3      : int          )
	( value   : 'a           )
	( value3d : 'a * 'a * 'a )
	( values  : 'a list      )
	( display : ('a -> unit) )
: int = (* return type unindented, to stand out better *)
	let internal_function_if_scoping (a: int) (b: int) : int =
		if a > 0 then
		(
			let f_a   = float_of_int(a)          in (* align equal signs, hungarian type-hinting *)
			let f_b   = float_of_int(b)          in (* align equal signs, hungarian type-hinting *)
			let c_sqd = f_a *. f_a +. f_b *. f_b in (* align in keywords, acceptable abbreviation given universal stdlib nomenclatures *)
			let f_res = Float.sqrt (c_sqd)       in (* everything was unpacked neatly on multiple lines, so the final expression is immediately semantically legible *)
			int_of_float (f_res)
		)
		else
		(
			-1 (* it's question of consistency/style to echo the `if` block Allman style, but admittedly, `else -1` could also work *)
		)
	in (* notice the blank line below *)

	let internal_function_match_and_letin_scoping (a: int): (cases * int) =
		let result1 = (* multiline let-in example *)
			match a with (* scoped with an extra tab for the let in *)
			(* pipes aligned with match *)
			(* `->` vertically aligned because small/simple output expression *)
			| 1                           -> Case1
			| 2 | 3                       -> Case23
			| 4                           -> Case4
			| 5 | 6 | 7                   -> Case567
			| n when 100 <= n && n < 1000 -> CaseThreeDigits
			| _                           -> CaseOther
		in
		let result2 = (* simple match-unpacking on a single line example *)
			match value3d with | (v1, v2, v3) ->
			( (* Allman C parenthesis scoping for a multiline expression *)
				display(v1); (* single arg function that returns unit, so space before parentheses can be omitted *)
				display(v2);
				display(v3);
				display(value);
				if value = v3 then 1 else -1 (* trivial ternary on a single line *)
			)
		in
		let result3 =
			match values with
			| [] -> 0 (* can be kept as-is, or put in an Allman C parenthesis block *)
			| h :: t ->
			(
				display(h);
				display(value);
				List.length t
			)
		in
		(result1, result2 * result3)
	in

	(* a block of constants can be kept together, but should be followed with a blank line *)
	let constant_1 = internal_function_if_scoping              (n1) (n2) in
	let constant_2 = internal_function_match_and_letin_scoping (n3)      in

	let example_uncurried_function
	(
		( a,    b,    c   ): (* vertical alignment of variable names and their type *)
		( int * int * int )
	)
	:
		int (* other way of making the return type stand out *)
	=
		a * b + c
	in

	(* version 1, quite ugly, with rather complex statements, but still somewhat legible since they are well-isolated with parentheses and commas *)
	(*
	example_uncurried_function ((constant_1), (match constant_2 with | (_, n) -> n), (internal_function_if_scoping (n3) (constant_1)))
	*)

	(* version 2, a bit better, with verticalization of args, which makes each sub-expression more legible *)
	(*
	example_uncurried_function
	(
		(constant_1),
		(match constant_2 with | (_, n) -> n),
		(internal_function_if_scoping (n3) constant_1)
	)
	*)

	(* version3, best, with semantically-relevant unpacking *)
	let constant_2_int_content = match constant_2 with | (_, n) -> n in
	let constant_3             = internal_function_if_scoping (n3) (constant_1) in
	let input_triplet          = (constant_1, constant_2_int_content, constant_3) in
	example_uncurried_function (input_triplet)
;;
(* TODO some versionX examples with a curried end of function as well ? *)
```
