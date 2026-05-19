# VP8 Decoder Implementation Plan (TDD Execution)

## Objective
You are an autonomous Software Engineering Agent. Your goal is to convert this transpiled C-to-Rust implementation of a vp8 decoder into one that is idiomatic and memory-safe, following the design in safety/architecture_overview.md .

## The Execution Strategy (Test-Driven Development)
This project is strictly governed by TDD. You will not write the entire decoder at once.

**Your Job** is to improve the decoder by working towards struct detangling as outlined in ./safety/top_level_struct_decoupling_strategy.md .  Pick one unit of work, just one, from there, and complte it.  Once you have done this, stop.  If you uncover other bugs along the way, you may try to fix them or document them in an md file.  What you want to avoid is trying to fix every tangled structure after the one you start on; that's a rabbit hole since there are so many.  Remember that we will eventually must fix them all, so don't take hacky shortcuts that make it harder later on.  Make measurable progress, and stop.

**IMPORTANT** NO HACKS!  We want a general-purpose decoder when we're doing; do  not hardcode things that are only specific to our test case!

## Current State
    **Hints**  HINTS.md has some notes from previous instances of you that might be of use (or not - they are not human audited so your mileage may vary)
    **Tests**  ./scripts/compare.py will run the test harness - it should pass when you start, and should pass before you declare success!  Note that the "unsafe blocks" count denominator might be lower than the numerator; ignore the denominator and focus on the numerator.

Here is an overview of the directory:
* ./src - the Rust decoder ** This is what you are working on **
* ./safety/ - the overall picture of the rust decoder design you are working towards
* ./build.sh - a handy script to build the decoder.  It is basically `cargo build` with some extra flags.

## Critical Rules of Engagement
*   **No Unsafe Rust:** You are forbidden from using `unsafe` blocks, `RefCell`, or interior mutability hacks to bypass the borrow checker.
*   **You are headless** Asking the user to approve a design doc or implementaiton plan.  Don't bother; you are running headless and it is safe to proceed with implementation directly.  Doing otherwise will time out.

**Your First Action:** When you are initialized in this workspace, acknowledge these instructions, confirm you have read the `architecture_overview.md`, and run the build script to confirm that it builds or not.  If not, fix the first error (working towards our goal of memory safety!) and stop.  If it already builds, pick an tangled struct and try to untangle it.  IMPORTANT: This work will be done by other agents, too, so please leave instructions for them about where you left off in safety/HINTS.md .  Include information that might help future agents progress further and/or clean up out-of-date previous hints.  Ideally, we want our harness, "./scripts/compare.py", to complete successfully at the conclusion of your turn.  Definitely you should nt cause it to stop working, if it worked when you started!
