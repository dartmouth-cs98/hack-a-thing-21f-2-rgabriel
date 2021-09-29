# Hack Technology / Project Attempted


## What you built? 

For this assignment, I sought to get some hands-on experience with Rust
by developing the foundations of an operating system. Within the first four posts in this tutorial, I developed a structure used for printing basic strings on bootup, printing to console rather than buffers, and a framework for unit and integration testing. Admittedly, there's less to show for it visually than in the code itself.

[Basic Bootup](img/textBoot.png)
[Tests](img/testOutput.png)

## Who Did What?

I worked alone for this hack-a-thing as well.

## What you learned

What didn't work: Because the task was to develop a bare-bones operating system,
I was not able to utilize Rust's standard library. Most of the functions in the 
standard library serve as abstractions atop what the user's home operating system
provides. Without that aspect, many of the tools that are easy to use in base Rust weren't available. They had to be incorporated in more complicated ways. Regardless,
it was not a bad experience, it simply demonstrated a side of the language that I'd most likely not deal with on a personal basis. Other than that, I guess the syntax for macros was a bit odd.

What worked: Despite being an older tutorial, everything that was mentioned in the blog still worked to this day. Some of the operations relied on unstable versions of Rust, in which the developers might completely purge features out of the language if they find that it wasn't beneficial. Because of this, I found it rather pleasant to see that everything stil worked as specified (granted this is the second time the blog has been made).

Rust's compiler is perhaps the most useful one I've ever used. The error messages, in addition to indicating the line where the error occurred, often give suggestions that actually fix the issue (or lead to more specific errors that are searchable).

Not once did I face any of the memory issues that are present in a language like C. Obviously, the chance of this happening is small because the code has already been tested by the creator of the tutorial. Regardless, whenever there was potential for a memory issue, it was clearly indicated with the unsafe keyword. And, as is typical in the Rust community, when unsafe is used it is then wrapped in an abstraction that makes working with the memory safe, leading to no valgrind headaches.

Overall, I enjoyed using the language and would love to work with it for the project, especially given that it does have a community interested in enhancing it with packages and stuff.

## Authors

Roberto Gabriel Brito

## Acknowledgments

I followed the first four posts of this tutorial: [Rust OS](https://os.phil-opp.com/)
