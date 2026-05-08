<div align="center">

<h1> Terse </h1>

</div>

Terse is a minimalistic, lightweight and direct terminal search engine. It is designed to give you answers to programming questions as quick and bloat-free as possible

<div align="center">
<!-- TODO: better title -->
<h2> Searching is Complicated </h2>

</div>

When you want to look up something (as you often will), going on the Web to look it up requires a massive amount of extra noise.

You open your browser, type in your question, and load up a whole react-based web page with *design* and *animation* and *ads* and *cookies* and all this extra fluff when what you really want is just the text that tells you what to do. And you'll often be looking things up multiple times a minute, and the process begins again.

*some* will even ask a bot, and set in motion billions of tensor operations in a noisy datacenter to generate each individual word of *how to center a div for the ten thousandth time because you can't bother to remember*.

**Terse gets rid of all this bloat**. You ask a question, you get an answer, right in your terminal. Simple as that. Everything's plain-text, small, and direct, written by developers like you.

<div align="center">

<h2> Why use Terse? </h2>

</div>

### Terse is fast

Written in the Rust language, with no rendering of HTML, no background processes running, nothing but plain-text answers to your plain-text questions

### No distractions

There's no ads, no banners asking you to donate, <!--You do that on the github-->no tabs everywhere trying to get you to click, no extra fluff obscuring the actual answer, no getting distracted and hopping on social media, either. Just direct flow between you and the answers you need.

### Terse is lightweight

No worrying about "bandwidth" or "tokens"; Terse is the bare minimum, and all you need.

<div align="center">

<h2> Usage </h2>
<h3> WARNING: All this is theoretical, but a good roadmap for the functionality </h3>

</div>

Once you have Terse installed, type in ```ts --init``` and enter in the following information (only necessary if you want to publish Answers):
<!-- What about rate limiting? What about so-and so?... ah well for now, it's OK. -->

```shell

~$ ts --init

Enter your nickname and email
Email: 
 bob@example.com
Nickname (max 16 chars):
 Johnny McCool
```

Now to search for something, type in ```ts``` followed by your question / keywords:

```shell

~$ ts format string python

String Formatting Cheatsheet - All Languages (1)
by coolguy3D (18k) at 2/14/2026

Formatting Strings in Python - In-Depth (2)
by pythonwranglers (354) at 1/12/2025

What Are Format Strings? - Explanation (3)
by kevinwright (2k) at 3/14/2026
...
```

<!-- TODO: find other shortcuts -->
You can scroll up or down using the arrow keys, and enter a number to 
open the answer corresponding to it.

<!-- https://www.c-sharpcorner.com/uploadfile/mahesh/format-string-in-C-Sharp/ -->
```shell
String Formatting Cheatsheet - All Languages

Python:
    f"number a: {a}; number b: {b}"

C#:
    $"number a: {a}; number b: {b}" or
    String.Format("number a: {a}; number b: {b}", a, b)

JS: 
    `number a: ${a}; number b: ${b}`
...
```

To get info about a specific user, type in ts --who [username]:

```shell

~$ ts --who pythonwranglers

pythonwranglers (354)
pythonsquad@hotmail.com

Hey there! We are the Python wranglers, posting helpful, 
reliable Python tips and cheatsheets whenever we have time!

How to (Cheat and) Do Overloading in Python (1)
by pythonwranglers (354) at 3/11/2026

The History Of Python (2)
by pythonwranglers (354) at 2/24/2026

Formatting Strings in Python - In-Depth (3)
by pythonwranglers (354) at 1/12/2025
...
```

<!-- https://www.linkedin.com/pulse/git-identity-paradox-why-your-commit-email-isnt-always-amit-kumar-eeh5f ??! We must do better? How do we verify email at all?! -->

To publish your own answers, use ```ts --pub "Answer Name Goes Here" path/to/answer.txt```
You can only publish .txt files, with links embeddable.
<!-- TODO: ...via [text](link) but how displayed? how linkable? and of course you should be able to escape those characters -->

<!-- TODO: No idea how reasonable this is; maybe do byte size? -->
The max number of characters is 32768, forcing answers to be brief, succinct, and direct, 
with maybe a little room for humor, too.

```shell
ts --pub "Traits in Rust - Deep Dive" /answers/traits-in-rust.txt

Publishing...
Successfully published:

Traits in Rust - Deep Dive 
by Johnny McCool on 4/10/2026
```


<!-- TODO: likes, followers? some sort of (transparent) algorithm to rank 'em, a --help sorta thing (which of course leads to an Answer in Ts), more detailed instructions than the README

Also ts --edit "title" path-to-file
Also comments! comments are good! so you could ask questions as opposed to answers...

Also you could go; for adding ratings to articles;

1: How to (Cheat and) Do Overloading in Python (134k)
   by pythonwranglers (354) at 3/11/2026

2: The History Of Python (21)
   by pythonwranglers (354) at 2/24/2026

3: Formatting Strings in Python - In-Depth (235)
   by pythonwranglers (354) at 1/12/2025

Also a way to follow links... Like the links in Ts should just be an ID, not like an actual link; things should be easy to link to in, say, markdown files / a ts go command? ts go 32203948, for example, for a quick thing you gotta remember?... and in text, you prefix it with https://Terse.com/answers?id={id}
-->