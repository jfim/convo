# Convo

A simple tool to view LLM conversations stored on your computer, so that they can be linked to from other places. Local only, your conversation files stay on your computer.

![Convo screenshot](convo-screenshot.png)

## Installation

`just install`

## URL support

Currently only supports `claude-code` URLs: `convo://claude-code/path/conversation_id`, eg `convo://claude-code/-home-jfim-projects-convo/d1449295-5c92-42db-a10f-571fd4ab36b1` will render the conversation in `~/.claude/projects/-home-jfim-projects-convo/d1449295-5c92-42db-a10f-571fd4ab36b1.jsonl`

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
