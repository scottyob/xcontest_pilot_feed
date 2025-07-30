# RSS Viewer for my ❤️'d Pilots

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

I wanted a way of not missing the flights my friends have done!  I couldn't find a way of doing this in xcontest.org, so, this is a simple tool designed to:
- Periodically poll the flight page of my friends
- Produce an RSS feed of their recent flights.

# The settings file:
A demo of the *config.yml* file is below
```
key: 03ECF5952EB046AC-A53195E89B7996E4-D1B128E82C3E2A66
url: https://home.scottyob.com/xcontest

users:
  - TDonker
  - Scottyob
```

## Getting the key:
I found in the following line on https://www.xcontest.org/world/en/pilots/detail:Scottyob
```
    		<script type="text/javascript" src="https://www.xcontest.org/api/js/?key=03ECF5952EB046AC-A53195E89B7996E4-D1B128E82C3E2A66"></script>
```

No idea how often this changes!