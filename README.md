# Lish

Lish is a shell with an embedded lisp. The idea is to provide only the most
common shell use cases with shell syntax, but replace scripting with a general
purpose lisp dialect.

Simple commands can be run normally:

- `cat file`
- `echo text`
- `ls -la`

But if the line starts with a left parentheses, the lisp parser is used instead:

- `(def a 1)`
- `(+ a 1)`

The project is in very initial state. Many cool decisions still need to be made
on how to integrate the two languages and what features should be added to the
shell or only to lisp.

My plan is to work on an MVP that is at least usable for common operations like
navigating directories and using git, and then figuring out where to go
from there.

It's not meant to be POSIX compliant, but the basic forms should be similar.
