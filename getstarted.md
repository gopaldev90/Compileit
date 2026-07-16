CompileIt Setup Guide

Welcome!

This guide will get CompileIt running on your system in just a few minutes.

---

Requirements

CompileIt currently supports:

- Linux
- Android (Termux)

Windows support is not currently a target.


Clone the Repository
```
git clone https://github.com/gopaldev90/Compileit.git
cd Compileit
chmod +x setup.sh
bash setup.sh
```
---


Required Compilers

CompileIt is only a build tool.

It still uses the official compilers underneath.

Rust

cargo
rustc

C++

Choose one:

g++

or

clang++

Go

go

Java

javac
java

If these programs aren't installed, CompileIt cannot summon them through sheer optimism.

---

Output Extensions

CompileIt intentionally uses custom executable extensions.

Android

.axe

Linux

.mixe

Because life is too short for executables with no personality.

---

Creating New Files

Generate source templates instantly.

compileit -newfile hello.cpp

compileit -newfile Main.java

compileit -newfile main.rs

compileit -newfile server.go

No copy-pasting from Stack Overflow.

No empty files.

No ritual.

---

Typical Workflow

Create code.

Compile it.

Run it.

Repeat.

Example:

compileit hello.cpp

compileit main.rs

compileit server.go

compileit MyJavaProject

CompileIt automatically determines whether the input is:

- a source file
- or a project

You don't need to specify it.

---

Termux Notes

Android shared storage is usually mounted with the noexec flag.

This means executables stored there cannot be run directly.

CompileIt works well.

executables into an executable location before running them.

This isn't CompileIt's fault.

It's Android expressing its feelings.

##To run binary elf file(.axe):

copy chlao function from startup.sh to .bashrc or whatever shell you using.

reload shell

```
chlao <your_project_compiled_binary>
```

---

Troubleshooting

compileit: command not found

you forgot to run startup.sh

---

cargo not found

Install Rust first.

---

g++ not found

Install a C++ compiler.

Termux:

pkg install clang

---

go not found

Install Go.

Termux:

pkg install golang

---

javac not found

Install OpenJDK.

Termux:

pkg install openjdk-21

---

Permission denied

Welcome to Android.

Your executable is probably sitting in shared storage with the "noexec" flag.

Move it into an executable directory or use your preferred helper utility.

---

Getting Help

compileit --help

If something still doesn't work, open an issue on GitHub with:

- your operating system
- compiler version
- CompileIt version
- complete error message

Screenshots of half the error are strongly discouraged.

---

Happy compiling.

May your builds succeed,

your linker cooperate,

and your compiler errors be someone else's fault.