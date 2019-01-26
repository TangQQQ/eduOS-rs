---
layout: page
title : Building eduOS-rs
sidebar_link: true
---

## Requirements to build eduOS-rs
eduOS-rs is tested under Linux, macOS, and Windows.

### macOS
Apple's *Command Line Tools* must be installed.
The Command Line Tool package gives macOS terminal users many commonly used tools and compilers, that are usually found in default Linux installations.
Following terminal command installs these tools without Apple's IDE Xcode:

```sh
$ xcode-select --install
```

### Windows
To build eduOS-rs you have to install a linker, [make](http://gnuwin32.sourceforge.net/packages/make.htm) and a [git client](https://git-scm.com/downloads).
We tested the eduOS-rs with the linker from Visual Studio.
Consequently, we suggest installing Visual Studio in addition to [make](http://gnuwin32.sourceforge.net/packages/make.htm) and [git](https://git-scm.com/downloads).

### Linux
Linux users should install common developer tools.
For instance, on Ubuntu 18.04 the following command installs the required tools:

```sh
$ apt-get install -y curl wget nasm make autotools-dev gcc g++ build-essential
```

### Common for macOS, Windows and Linux
It is required to install the Rust toolchain.
Please visit the [Rust website](https://www.rust-lang.org/) and follow the installation instructions for your operating system.
It is important that the *nightly channel* is used to install the toolchain.
This is queried during installation and should be answered as appropriate.

Afterwards the installation of *cargo-xbuild* and the source code of Rust runtime are required to build the kernel:

```sh
$ cargo install cargo-xbuild
$ rustup component add rust-src
```

eduOS-rs is able to run within [ehyve](https://github.com/RWTH-OS/ehyve), which a specialized hypervisor for eduOS-rs.
Therefore [ehyve](https://github.com/RWTH-OS/ehyve) must be installed.

```sh
$ cargo install --git https://github.com/RWTH-OS/ehyve.git
```

Please check if your system fullfil ehyve's [system requirements](https://github.com/RWTH-OS/ehyve).

## Building
The final step is to create a copy of the repository and to build the kernel:

```sh
$ # Get our source code.
$ git clone https://github.com/RWTH-OS/eduOS-rs.git
$ cd eduOS-rs

$ # Build kernel
$ make
```

From here, we should be able to run the kernel in ehyve:

```sh
$ make run
```

## Overview of all branches

Step by step (here branch by branch) the operating system design will be introduced.
This tutorial shows the steps to develop from a minimal kernel to a Unix-like computer operating system.
Currently, following stages of development are available:

0. stage0 - Smallest HelloWorld of the World

   Description of loading a minimal 64bit kernel

1. stage1 - Cooperative/non-preemptive multitasking

   Introduction into a simple form of multitasking, where no interrupts are required.

2. stage2 - Priority-based cooperative/non-preemptive multitasking

   Introduction into a simple form of priority-based multitasking, where no interrupts are required.

3. stage3 - Synchronization primitives

   Introduce basic synchronization primitives

4. stage 4 - Preemptive multitasking

   Introduction into preemptive multitasking and interrupt handling

5. stage 5 - Support of user-level tasks

   Add support of user-level tasks with an small interface for basic system calls

6. stage 6 - Support of paging

   Add support of paging and a simple demo for process creation

7. stage 7 - Integration of an in-memory file system

   Introduce a virtual file system with an in-memory file system as example file system.

8. stage8 - Run Linux application as common process

   Start a simple Linux application (_HelloWorld_) on top of eduOS-rs. The application is a _position-independent executable_ (PIE) and use [musl-libc](http://www.musl-libc.org) as standard C library.