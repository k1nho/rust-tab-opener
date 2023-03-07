# Chrome Tab Opener CLI

A simple tab opener to keep track of multiple series and prepare for a binge.

## How it works
1. Add the series you are planning to watch to series.json file

```json
[
  {
    "name": "mywatch1",
    "ep": 5,
    "limit": 24
  },
  {
    "name": "mywatch2",
    "ep": 20,
    "limit": 50
  }
]
```

- name: The name of the series as displayed in the URL
- ep: The episode number
- limit: The total number of episodes (will not lauch a series if all episodes were watched)

2. Add a base url for the site in .env file

```
BASE_URL= yourbaseurl
```

3. To run the tab opener

```console
cargo run
```

### Flags
- x : execute (selected by default)
- w:  write (used to update series count **u** for counting up, and **d** for counting down). The use of w 
with no follow up arguments just prints the series


To update the series count up, the following command can be used.
```console
cargo run w u
```

To update the series count down, the following command can be used.
```console
cargo run w d
```

To print the series.
```console
cargo run w
```
