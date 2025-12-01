# Advent of Code Rust Template

![Build Status](https://github.com/chriswmann/aoc2025/actions/workflows/rust.yml/badge.svg?branch=main) [![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Template with Santas Little Helpers to do AoC in rust.

## Configuration

Set the environment variable `AOC_SESSION_ID` with your session cookie (e.g. `AOC_SESSION_ID`).
The year is automatically extracted from the environment variable name.

### Obtaining your session cookie

1. Log in to [adventofcode.com](https://adventofcode.com) using GitHub, Google,
   or another provider
2. Open your browser's developer tools (right-click → Inspect)
   and navigate to the **Network** tab
3. Navigate to any input page, e.g. [adventofcode.com/2016/day/1/input](https://adventofcode.com/2016/day/1/input)
4. Find the request in the Network tab and look at the **Request Headers**
5. Copy the value after `session=` from the `Cookie` header

You can use [direnv](https://direnv.net/) to automatically set the
environment variable when entering the project directory.
Create a `.envrc` file with:

```sh
export AOC_SESSION_ID=your_session_cookie_here
```

Note: `.envrc` is included in `.gitignore` to keep your session cookie private.

## Usage

To run a specific day, use `cargo r --bin <DAY>`, where `DAY` is like `day01`.
Note: Some days may use features requiring nightly rust. These can be run using
`cargo +nightly r --bin <DAY>`.

Or run `just run-day <DAY>`, e.g. `just run-day 03` (zero padding is required).

To run unit tests across the workspace use `cargo t --workspace`.
Or run `just run-tests`.

To add a new day, run `just add-day <DAY>`, with `DAY` a zero-padded two-digit number.
