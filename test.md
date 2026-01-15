# Using Claude Code and Obsidian Together: An AI-Assisted Knowledge Base

## Introduction

Hello and welcome to my video on using Claude Code and Obsidian together to create an AI-assisted knowledge base. I am a huge note-taker - I've been taking notes for a long time (it kind of feels weird to say it, but I feel like everybody takes notes). I've been using it as a productivity tool for like 15-16 years and have gone through a bunch of different systems of note-taking that were popular, like the note-taking system du jour, GTD (Getting Things Done) back then.

I've used so many different tools like Evernote, Bear, and many others, but have kind of landed on this solution of using Obsidian and Claude Code. Both of these tools are very important in this system and I can't really use one without the other now.

## Why Obsidian?

Obsidian has a really good reputation and there's not really much to say about it:
- It has great plugins
- Claude Code can understand some of those plugins and create things like queries for the DataView plugin or tables for Advanced Tables
- Obsidian uses plain Markdown
- I have several different vaults that I sync, which means I can view the same stuff on my phone, iPad, computer - wherever I want or need to
- It has a fantastic view for reading markdown

## Why Claude Code?

Claude Code does a lot of things for me. Claude Code is like my user interface - it's how I use my knowledge base for research, analysis, and more. Obsidian is more like the consumption part of the knowledge base.

I have some views on how I want to structure my own notes, and what's great is that with Claude Code you're able to figure out that system for yourself however you want to do your note-taking.

## Video Overview

In this video, I'm going to:
1. Show you my system real quick
2. Show you how I use the research tools for Claude
3. Start a completely new knowledge base with Claude

**Note:** Claude takes some time to process things. I'm not going to be speeding it up because I think it's important to notice the real experience.

## My Note-Taking Philosophy

One thing is I set up my notes for mostly research and ephemeral notes - notes that I will be archiving, notes that will go out of date very fast. I believe that's like all notes for me. After a certain amount of time with any note-taking system, I basically have to declare a "note bankruptcy" and start over. This system is built with that in mind.

### On Digital Gardens and Evergreen Notes

I don't feel that the concept of a "digital garden" or "evergreen notes" really exists. I see a lot of YouTubers talk about these concepts and it sounds great on the surface, but then I don't see anybody using it for examples other than for making a YouTube video. Knowledge and things change so fast that having an evergreen note just doesn't entirely make sense to me. Even my own perspective on the notes changes, which means I feel like I have to create a new note. I'm not a big fan of editing existing notes because I like to have the history of that note.

## My Folder Structure

Let me show you what I mean. I have:

### Image Folder
This is where Claude Code downloads any relevant images.

### Inbox Folder
This is where I put my human-written notes. For example:
- Quick note for ripgrep
- Notes from the FFmpeg ASM lessons (though they don't really mean anything specific - I wanted to showcase this)

I don't use the handwritten/human-written notes that often, but it is an option.

### Claude Code Commands and Agents
I have a series of Claude Code commands and agents that help me process this data, which I'll show off.

### Output Folder
This is where I ask Claude Code or Gemini to put analysis outputs. For example, if I want to run an analysis of my notes to a repository, that's where the output goes.

### The Feed Command
I have a really cool command called "feed" for Claude Code that:
- Looks at the 20 most recent notes
- Compiles them together with summaries
- I call this my RSS feed for my own notes
- I pull this up on my iPad or phone and look at what I've been researching
- Everything's linked together
- Because of Obsidian, you get previews and everything

**This is my favorite feature** - this is how I consume the majority of my notes. I typically just need to look up stuff that I looked up in the past week or two. If I need to look up anything before that, I always have Command+O to find whatever file I want, or I can ask Claude Code to find it for me.

This is a research knowledge base that is constantly updating, and I can update it however I want to.

### Research Inbox
I have a research inbox with notes like:
- Kids sandals for my kids
- Lord of the Rings Blu-ray denoising
- Revision history
- Music tagging
- Terminal user interfaces
- Getting started with Rust

My research inbox is essentially notes for stuff that I want to look up, research, and learn more about.

## The Problem with Search Today

My problem has been recently over the past few years: whenever you look stuff up on Google or DuckDuckGo, the results are subpar and not very usable. You get:
- A Reddit post from six years ago that's irrelevant
- A Medium post from three years ago that's irrelevant
- AI slop (which I know is ironic saying that because I use Claude Code to generate my own slop, but it's my own slop in my own way that works for me)

## How I Do Research

Whenever I want to research something, I just create a new note and tell it to research something. For example, I created "Getting Started with Rust" and "Getting Started with Golang."

### Example Research Note Structure

Here's what I know:
- I'm a software engineer
- I have knowledge of Python, JavaScript, a little bit of Scala
- I've done some Go as well

Here's what I want to know:
- How to set up a new web project
- Basics of syntax plus gotchas
- Best practices
- What every new dev in Golang should know

This is how I use it. I then fire up Claude in my notes repository. I have a bunch of commands and stuff, so I can just tell it to research and it'll do its job in the background.

## Claude Code Commands and Agents

### The .claude Folder
I have a `.claude` folder in my notes folder. It doesn't show up for Obsidian (though I think there's a plugin for syncing hidden folders).

### Commands
Claude has commands that allow you to create commands for anything you really want to do. For example:
- Archive all notes that are a month or older
- Create a special project setup
- Create a new project that links correctly and does initial research

I have sections like:
- Maps and MOCs (which I archived this morning as I don't use them)
- Project section with manga recommendations, how to do Roblox games, etc.

### The Research Command
Looking at the research command:
- We use the research synthesizer agent that's an agent I set up
- We have a comprehensive long prompt for this command
- We leverage both Claude and Gemini
- We prioritize authoritative sources
- We cross-link stuff

**Important:** This prompt is generated by Claude. I typically tell Claude what I want it to do, then tell it to create a prompt from this or create a command or agent from it, and it'll create this comprehensive list. I would recommend manually reviewing it, but it does a really good job for creating prompts for itself.

### Updating Prompts
Let's say it doesn't do it right next time you run it. You can just tell it to fix it and update all the relevant prompts with that fix.

For example, this morning Claude was having a hard time linking together images for Obsidian (Obsidian doesn't use the regular markdown image reference tag). I knew there would be an issue. Claude used to be able to do this correctly but then it started messing up. I asked it to fix it, it fixed it, and I said "Hey I need you to update the prompts to make sure that it saves it correctly."

You can mark things as critical, show correct vs wrong examples, and be very explicit about it. Claude was able to figure that out for itself.

### Agents
I have agents (or "personas" might be a better way to put it):

**Research Synthesizer:** It's critical for it to use Gemini CLI. It has a description of how to use the Gemini CLI, so Claude calls into Gemini CLI and uses those results.

You might want to have these agents/personas because:
- Your research synthesizer might be skeptical, comprehensive, do this and that
- But for example, I have a maintenance specialist that I want to NOT be skeptical
- I want it to be precise, not think outside the box, just follow instructions

Claude can pick up on these and use the correct agent.

### Permissions
I have all the permissions turned on so it asks me for every single link it will fetch. You can turn this off (people call it "YOLO mode") so it just does whatever it does.

## Use Cases for Commands and Agents

Whatever you're trying to do in your notes repository, you can create a command and an agent for it (or one or the other).

If you're doing programming:
- You might want to have an agent that knows how to run code
- A Node runner agent that knows how to use the Node CLI, what to run, where to run it
- You might ask it: "I'm taking notes, can you extract all of the code blocks and run those in Node for me?"

Whatever works for you, you can make happen.

### The Challenge
One thing that's kind of hard is people coming up with their own systems:
- What do you do?
- How do you structure it so it works for you?

I think that's a great place for being able to set up a brand new repository for notes and say "Hey make it Zettelkasten, use the Zettelkasten method." Then you notice that you don't use certain things or use certain things a little bit differently, and you're able to talk to Claude and figure that out and fix things.

## The Inbox Command

I can run the inbox command - I have an inbox command that processes my inbox how I want it to. The research synthesizer is already creating a new folder in my reference folder for Golang and will save the note there.

You might notice I'm using this for software engineering, but I've even used this to look up the best kids sandals for my kids for school. Honestly, it works a lot better than just going through Amazon and reading fake reviews or going through Google and seeing people on Reddit argue with each other. It just creates a list and makes it a lot more doable to compare five different types of sandals rather than the million that exists.

## My Notes Structure

With my notes structure, I had things like fleeting notes, learning paths, and all this other stuff. I've noticed that I never went to those notes, never checked them out. I always used my RSS feed because that's what worked way better for me, so I just got rid of it. Then I told Claude to get rid of any reference to MOCs from all the READMEs and prompts so it wouldn't deal with them anymore.

I just move everything into the archive. Then I have these projects - longer-running research projects where it's not just a one-off note.

### Example: Manga Recommendations Project
If I wanted to really learn Go, I would create a new project. I have a few active ones, like manga recommendations (kind of weird, but I created a list of manga I wanted to research for my kids to be able to read).

I went through a bunch of lists, asked it to go through lists, fetch all the age ratings (it'll figure out how to get them), then it compiled a list for me. I selected manually from that list, and I told it: "Hey collect all the selected manga from all the notes that I read through and create the selected manga collection."

The result shows:
- The Witch and the Beast
- Age rating
- Format (black and white - important because my kids wanted to read colorized comic books, which is more like manhwa/Korean comics rather than manga)
- Status
- Little Witch Academia (couldn't find a cover image)
- Sailor Moon (fantastic for kids, really cool manga I'd never heard of)

I was able to do this kind of research with an active project.

### Reference Notes
I have reference notes where the majority of my stuff goes - different folders and different things I'm trying to learn about, or one-off research tasks like the Rust one, "Getting Started with Rust."

## Fixing Issues in Real-Time

Let's look at "Getting Started with Go." One thing it did was get the formatting incorrect in the front matter.

I can say: "The formatting in the front matter is incorrect, fix it and explain what is wrong so that I know exactly what's wrong." I can also tell it to update its prompt.

For example: "Remove the dependencies front matter altogether. Remove that key from any prompt so that no new notes will contain this property."

I don't have the best way of communicating, but Claude just seems to understand me well enough.

### What Was Wrong
I left that in there on purpose - this was something I noticed was wrong this morning and I figured let me not fix it so I can show you. It was trying to put links into the front matter and that's just not working correctly.

## The Research Output

Looking at the research output for Golang:
- Where to download and install it
- Set up our first project
- Initialize Go module (I don't remember seeing this when I worked on Go because I haven't worked in Go in a long time)
- Code syntax fundamentals

This is all over the place but I like this. The syntax fundamentals, variable types - this all looks correct.

### Functions
This is how you declare functions. I didn't know you could declare return values this way (been out of Golang for a while). It's at the right level for me - I already know multiple programming languages, so I don't need to know what a string is. I don't need an explanation of what an int is.

The short declaration I'm familiar with, but I'm glad it has the long one and what people actually use. Multiple variable declarations - this is great.

### Web Project Setup
Setting up a web project with Gin (I've used Gin before):
- Setting up first routes
- This very much looks like Express for Node
- You pass in the context with a star
- I'm not familiar with the star syntax

I can say: "I read the Getting Started with Go. I'm not familiar with the star syntax `*gin.Context`. Add a new section explaining what this does and what it means."

I just communicate with Claude throughout as I do this research. Having the standard project layout with error handling - this stuff might not be exactly correct, but I can recognize all the stuff that is correct.

It even downloaded the mascot! Based on my background, it created sections and has all the references to it at the bottom. So if I start going through it and something doesn't compile right or I'm not sure about something, I can go to the source material.

### Pointer Syntax
It added a section for the context. This is a fairly long note, so I might ask it to separate it. Let me see if we can find the pointer syntax section.

I can read about pointers and how they work and what they do. This is a fairly good explanation with examples of what it does - here it copies, here we're modifying the original. I'm glad to know these things.

## Processing Human-Written Notes

For handwritten notes, I had the ASM lessons and ripgrep. The way I set this command up is for it to do a summary of my notes and to preserve the original content. This note looks as bad as I wrote it (which is great!), and then I have a summary. It already did a bunch of research for me on some basics.

It worked on top of my notes and this is all stuff you can set up.

### Adjusting the Behavior
I noticed Claude did its own research when processing inbox, but I don't want any additional research to be done when processing my inbox notes. They should be formatted, moved, etc., but no new research. I can tell it to update all relevant prompts to follow these rules.

Looking at the updated prompts, it's being very adamant:
- Only based on user's original content
- No research, no web searches, no content expansion

For the research inbox, we DO want all that extra stuff. For the regular inbox, we don't want it.

Whatever your flow is, you can customize these things however you want. Isn't this cool? It tagged it correctly, put it into the right folder and everything.

Imagine you can just write notes and not worry about doing that 45-minute post-processing at the end of the week where you figure out where notes go, how you tag them, how you search them, are all your MOCs updated. Just throw that all away and ask Claude to do this.

Some of these tags I might not be a fan of, so I can just ask it to retag my entire system: "Hey come up with a new tagging system, this is kind of useless."

## Setting Up Your Own System

Now let's look at setting up our own system. I'm going to go from my notes to a demo vault that I already set up (when I say setup, I mean I just created it).

### Starting with a README
The first thing you want to do when creating your own note-taking system is create a README file, because Claude can read this and infer what you want.

You want to describe, as if you're describing to somebody else, what your note-taking system looks like:

```
This is a knowledge base for all of my note-taking and research.

Topics in this knowledge base:
- Software engineering
- Photography
- Gardening and plants

Format:
- Utilizes Zettelkasten method
- Obsidian first markdown
- Uses a lot of tables
- Utilizes Mermaid for graphs (mermaid.js - markdown converted into graphs)

AI Assisted:
This knowledge base will be AI assisted utilizing Claude for research, note maintenance, and note organization.
```

### Initializing Claude
I'm going to fire up Claude. It'll ask "Can I initialize here?" Yeah, go ahead and create your `.claude.md`. It's going to read the README file and create a `.claude.md` file that helps us do exactly what we need to do.

This might take a little while. Claude takes time sometimes and that's okay. I typically need this stuff async, not right away.

### Cost Considerations
By the way, Claude does cost money. It's important to realize that while you're not paying for a subscription for Readwise, Evernote, Bear, or whatever else, you will be paying for Claude Code and some of that research will cost you more money. But I found it to be extremely worth it.

If you give yourself an amount you're willing to pay for a subscription service, I would just translate that into Claude and use that money there.

### Initial Setup Complete
We have the `.claude.md` setup now with:
- Zettelkasten approach
- Obsidian compatible settings

I'm going to ask it: "Update the README to look nicer and be more comprehensive."

Again, Claude is my user interface for this note-taking system, which is fantastic. I really like it.

### Removing Emojis
One thing - I hate emojis. That's going to be something I change: "Important: do not use emojis in file names or headings."

I really don't like it. That research cost 37 cents, by the way. If you just press up and edit, I can reprocess that same note because we realized something we don't like about it.

### Creating Folder Structure
While it's doing that, you can fire up even more Claude instances. I'll tell it: "Read over the README and create a folder and file structure following best practices for knowledge bases, the Zettelkasten method, etc. Seed initial notes with examples of how to use this note system."

### The Updated README
It updated the README - no emojis finally! Looking at it:
- Overview: AI-assisted knowledge base
- I wasn't that great at writing down all this stuff initially
- Knowledge domains we care about
- Zettelkasten method formats
- Prerequisites: you have to have Obsidian, be familiar with Markdown
- Setup with recommended plugins like DataView, Tag Wrangler (I've never used that but it's a really good one)

Claude is going to be able to use these plugins. When I say it's going to use them, it can't connect with Obsidian directly, but because DataView runs off markdown files (that's the beauty of Obsidian), Claude can update a DataView query in a markdown file and Obsidian can run it against the DataView plugin. That's fantastic.

### The Structure
We have:
- Notes and how they're going to look
- Attachments
- Templates
- Notes folder with different topics
- Usage: creating notes, linking strategies, AI assistance, best practices

This is so overly comprehensive - kind of what people call "AI slop" where AI takes one sentence and makes it a paragraph. But it still communicates what that one sentence said. We can ask it to pare down and compress it.

### Seeded Content
It's creating a bunch of stuff:
- Index (this is the MOCs - Maps of Content)
- Knowledge base documentation
- "How to use this vault"
- Zettelkasten principles with a Mermaid graph showing the loop
- Tagging system, taxonomy
- Knowledge growth
- Photography exposure triangle

It's creating all these demo notes and I didn't even tell it to do that - I just told it to seed with some initial notes.

### Adjusting the Setup
Some things here I kind of don't like. Once it finishes through all these different tasks, I'll be able to tell it:
- "Utilize front matter"
- "Double check on your research of the Zettelkasten method" (I remember there's the fleeting notes, evergreen notes, daily notes, etc.)

This is the positive part of doing notes this way - we can change the structure.

### Restructuring
I'll tell it: "I'd like you to research Obsidian Zettelkasten setups and reorganize all of my notes to utilize that structure. Make sure to use Obsidian markdown front matter for tagging and metadata. Once you're done with restructuring and editing files, update `.claude.md` with the correct prompt directions."

It's going to redo some research, analyze the current structure. It's using one of my research analyst agents that works on my entire machine, not just this repository. It's utilizing that automatically - I didn't realize it would do that, but it does and that's cool.

### Adding Commands and Agents
I can also say: "Hey I'd like you to add Claude commands and agents that would be relevant to this repository."

Here's a trick: I have some research in my vault about the sub-agents guide, and I remember there is a community agents examples page. "Check the community agent repository for ideas."

It'll do that web search, read through it, and set up all this stuff for us.

## The Final Results

### My Existing Notes Repository
After all the Claude instances finished:
- Created latest feed/notes feed project
- Updated README, search setup
- Two new notes on learning Rust and Go
- Tags, quick summary, developer CLI tools
- Assembly and language learning
- Active projects
- Archival stuff

This is my little RSS feed of all the different changes and research. I load this up on my iPad and scroll through it.

### The New Demo Vault
The folder structure looks way different now:

**Sub-agents created:**
- Knowledge architect
- Research synthesizer
- Vault maintainer
- Mermaid diagrammer
- Note connector
- Many more

**New structure:**
- Fleeting notes
- Literature notes
- Permanent notes (where all the previous notes went)

The front matter is now correct - we don't have that dependencies problem from earlier. We have aliases connected, everything we need.

We have MOCs:
- Gardening MOC
- Software engineering MOC with fundamentals and different notes

Some notes aren't even created yet, so I can ask Claude to make sure any note being linked gets created and researched.

### Commands Example
Looking at the sub-agents folder, the Mermaid diagrammer tells us:
- Different diagram types it supports
- The domains we want to research over and diagram
- Visual design principles
- Best practices

I asked it to set up a command to transform fleeting notes into permanent notes. It:
- Created templates for fleeting notes and permanent notes
- Created a specialized agent for transforming those (not exactly what I wanted, but I can tell it to make sure it creates a Claude command)
- Created a test fleeting note about test-driven development to demonstrate

After running the transform, it did all the research, took that fleeting note, expanded it, explained everything - kind of like that research command I had in my own repository.

## Conclusion

This is what you can do with it - this is the power of it. I can go into another Claude Code instance and do some other research or ask it to move things around, connect everything correctly, and it'll be exactly what I want it to have.

This is the power of having Obsidian for consuming the notes and writing fleeting notes or research inbox notes, and using Claude Code for everything else.

It's great because I can change this into a GTD system if I want to go that way or some other system, and it's fairly straightforward - you just talk to Claude Code and it'll do what you need it to do. Sometimes not perfectly, but that's totally fine because even if it doesn't do it perfectly, we can iterate on that process.

Thank you for watching! If you have any questions or comments, obviously leave them and let me know.
