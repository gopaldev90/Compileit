CompileIt

«One command to compile them all.»

Tired of remembering:

cargo build --release
g++ main.cpp -o main
go build
javac Main.java

Good.

Because I was too.

So I built CompileIt.

---

What is CompileIt?

CompileIt is a personal multi-language build tool designed for:

- Rust
- C++
- Go
- Java

It provides one consistent interface for compiling projects and source files.

Instead of remembering every language's rituals, sacrifices, and compiler incantations:

compileit rus myproject
compileit cpp hello.cpp
compileit gol p webserver
compileit jav e

Done.

---

Why?

Because my brain storage is reserved for actual programming.

Not for remembering whether today is:

cargo build
cargo build --release
cargo run
go build
javac
java
g++
clang++

CompileIt remembers that nonsense so I don't have to.

---

Supported Languages

Rust

Compile a project:

compileit rus myproject

Debug run:

compileit rus myproject d

Compile a single file:

compileit rus hello.rs

Output:

hello.axe     (Android)
hello.mixe    (Linux)

---

C++

Compile:

compileit cpp hello.cpp

Compiler flags are loaded from:

flags.txt

No recompiling CompileIt just because you changed optimization flags.

Imagine doing work manually.

---

Go

Single file:

compileit gol e main.go

Project:

compileit gol p myproject

---

Java

Single file:

compileit jav e

Project:

compileit jav p myproject

CompileIt automatically removes generated ".class" files afterwards because littering directories is a crime.

---

New File Generator

Create boilerplate files instantly.

compileit newfile hello.cpp
compileit newfile index.html
compileit newfile Main.java

Because typing the same template 400 times is not character development.

---

Configuration

CompileIt reads:

~/compileit_config.json

Example:

{
  "path": {
    "android": {
      "defalt_code_dir": "storage/shared/Codes"
    },
    "linux": {
      "defalt_code_dir": "Codes"
    }
  },
  "extension": {
    "android": "axe",
    "linux": "mixe"
  }
}

---

Output Extensions

Android:

.axe

Linux:

.mixe

Why?

Because seeing:

server.axe

is much cooler than:

server

---

Philosophy

Problem:

Repeated annoyance

Solution:

Automate it once.

Never think about it again.

---

Why No Python?

Because Python doesn't need CompileIt.

Python already has:

python script.py

That's the whole build system.

Same for JavaScript:

node app.js

Congratulations.

You skipped the entire compilation process.

CompileIt exists for languages that actually produce artifacts worth carrying around.

---

Official Anti-Interpreter Statement

Interpreters are welcome.

They may watch.

They may observe.

They may even be useful.

But they do not deserve CompileIt's premium compilation experience.

Rust? Yes.

C++? Absolutely.

Go? Of course.

Java? Fine.

Python?

python thing.py

You're already done.

JavaScript?

node thing.js

Go home.

This tool is for people who enjoy watching CPUs suffer.

---

License

Whatever future-me decides.
Currently powered by caffeine, frustration, and repeated compiler commands.
Why This Exists (Termux Edition)

Android has a special talent:

It sees your freshly compiled binary and says:

Permission denied

Even though:

✓ File exists
✓ Binary is valid
✓ You compiled it yourself

Why?

Because Android shared storage is often mounted with noexec.

Which means:

You may store files here.

You may admire files here.

You may look at files here.

You may absolutely NOT execute files here.

So the workflow became:

Source Code
     ↓
CompileIt
     ↓
something.axe
     ↓
loadbin
     ↓
~/compiled_own_binaries/
     ↓
PATH
     ↓
Actually runs

And for temporary execution:

Shared Storage
     ↓
chlao
     ↓
$HOME
     ↓
Execute
     ↓
Cleanup

CompileIt is not merely a compiler wrapper.

It is a peace treaty between Android and executables.

A treaty Android violates regularly.