#!/bin/bash
cat .repomirror/prompt.md | \
        claude -p --model=sonnet --output-format=stream-json --verbose --dangerously-skip-permissions --add-dir ../fl2 | \
        tee -a .repomirror/claude_output.jsonl | \
        npx repomirror visualize --debug;
