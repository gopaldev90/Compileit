CompileIt

One command to compile them all.

CompileIt is a lightweight multi-language build tool written in Rust.

Its goal is simple:

Stop remembering compiler commands.

Instead of switching mental gears between Cargo, g++, go build, and javac, CompileIt gives you one consistent interface.

Because your brain should remember algorithms, not compiler rituals.

Why CompileIt?

If you've ever caught yourself typing:

cargo build --release cargo run g++ main.cpp -o main go build javac Main.java java Main 

...only to forget which command belongs to which language five minutes later...

CompileIt fixes that.

One tool.

One workflow.

No compiler archaeology.

Features

Automatic language detection

Automatic project vs. single-file detection

Supports: 

Rust

C++

Go

Java

New file template generator

Compilation timing

Android and Linux support

Configurable output extensions

Designed specifically for Termux as well as desktop Linux

Supported Languages

Languag Single File Project
Rust         ✅     ✅
C++          ✅     ✅
Go.          ✅     ✅
Java.        ✅     ✅ 

CompileIt detects the language automatically.

You don't have to tell it.

Usage

Compile a Rust project

compileit myproject 

Compile a Rust source file

compileit hello.rs 

Compile C++

compileit hello.cpp 

Compile Go

compileit server.go 

Compile Java

compileit Main.java 

Compile a Java project

compileit MyJavaProject 

No separate commands.

No memorization.

Create New Source Files

Generate boilerplate instantly.

compileit -newfile hello.cpp
compileit -newfile Main.java
compileit -newfile server.go
compileit -newfile main.rs 

Because typing the same twenty lines of boilerplate hundreds of times is not a productive use of human life.

Configuration

read getstarted.md
Output Files

CompileIt intentionally uses custom executable extensions.

Android

program.axe 

Linux

program.mixe 

Why?

Because

server.axe 

looks considerably cooler than

server 

Scientific evidence is still pending.

Why This Exists (Termux Edition)

Android has a remarkable ability to look directly at a freshly compiled executable and confidently announce:

Permission denied 

Even though:

The file exists.

The binary is valid.

You literally compiled it yourself.

The reason is that Android's shared storage is commonly mounted with the noexec flag.

Translation:

You may store executables.

You may admire executables.

You may even back them up.

Running them, however, is apparently unacceptable.

CompileIt embraces this reality.

Typical workflow:

Source Code │ 
   ▼
CompileIt │
   ▼ 
program.axe │ 

It's a peace treaty between Android and executable files.

Android continues violating that treaty on a regular basis.

Philosophy

Programming already contains enough repetition.

If something annoys you every day,

automate it once,

then never think about it again.

CompileIt exists because compiler commands should become muscle memory—

or preferably,

someone else's problem.

Why No Python?

Because Python already solved the problem.

python script.py 

That's the build system.

Likewise JavaScript:

node app.js 

Done.

CompileIt focuses on languages that actually compile into standalone artifacts.

Official Anti-Interpreter Statement

Interpreters are welcome.

They may observe.

They may participate.

They may even be useful.

But they do not qualify for CompileIt's premium compilation experience.

Rust?

Absolutely.

C++?

Without question.

Go?

Naturally.

Java?

Accepted.

Python?

python thing.py 

You're finished already.

JavaScript?

node thing.js 

Go home.

CompileIt is for people who enjoy watching CPUs suffer.

License

Whatever future-me decides.

Until then, the project is powered by:

Caffeine

Mild frustration

Repeated compiler commands

The unwavering belief that typing cargo build --release for the thousandth time is unnecessary

CompileIt: because remembering compiler commands is not a programming skill.

