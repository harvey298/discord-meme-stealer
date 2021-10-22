# Discord Meme Stealer
this is just for fun!

## why?
I thought it would be a funny idea to make a index of memes, so I did but I made the "stealer" in python, over time the index grew to GBs and then soon it started to break down in ways I didn't expect and I had enough of fixing the poor Python implementation so I'm porting it to Rust with some improvements like instead of have the files download to the index itself it will instead put it in a tempoary folder! 

## Disclaimer
This isn't meant to be used for self-botting!
I am not responsible for anything bad that happens while using this!

USE AT YOUR OWN RISK

## What can this do?
currently it only downloads discord links
anything that isn't a "supported" file type will be saved as .unknown for safety

## Build
`cargo build --release`
