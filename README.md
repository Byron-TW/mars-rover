## Goal

Solve the mars rover problem the way I would like it, in Rust.
Try to be as idiomatic as possible, and consider the answer the main user
value to provide. Thus that and only that MUST be tested, anything else 
is 'extra', like actually testing for a few things the user can run into.

## Getting the answers

Run `make answers` if your `rust` installation is at least at v1.26.
If you have no `rust` but `docker`, run `make answers-in-docker`.

In any case, you can run all `make` targets using docker via `make interactive-developer-environment-in-docker`.
Please be warned that initial compilation takes a while.

## Features

* [x] shows correct answers
* [x] fully streaming with minimal state
* [x] support for profiling
* [x] support for benchmarking
* [x] support for linting
* [x] interactive developer environment in docker

## Implementation Notes

* file format parsing is case-insensitive, just because I personally prefer that.

## Benchmark Results

The Rust release binary comes in at 482kb on OSX (stripped). Max-heap is 412kb, and it processes a 7MB file in 850ms.
Amount of lines of code is 139, with 33 lines of bash for journey tests.
