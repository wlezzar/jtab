# Jtab

A tiny command line tool written in rust to print json data as a formatted table.

```bash
➜ echo '[{"id": "1", "name": "Rust"}, {"id": "2", "name": "Jtab"}]' | jtab

┌────┬──────┐
│ id │ name │
├────┼──────┤
│ 1  │ Rust │
├────┼──────┤
│ 2  │ Jtab │
└────┴──────┘
```

Nested data is handled correctly.

```
➜ echo '[{"id": 1, "nested": {"value1": 1, "value2": 2}}, {"id": 2, "nested": {"array": [1, 2], "deeply_nested": {"value": 1}}}]' | jtab

┌────┬────────────────┐
│ id │ nested         │
├────┼────────────────┤
│ 1  │ value1: 1      │
│    │ value2: 2      │
├────┼────────────────┤
│ 2  │ array:         │
│    │   - 1          │
│    │   - 2          │
│    │ deeply_nested: │
│    │   value: 1     │
└────┴────────────────┘
```

## Install

### Using Homebrew (MacOs & Linux)

```bash
brew tap wlezzar/jtab https://github.com/wlezzar/jtab
brew install wlezzar/jtab/jtab

# use jtab
jtab --help
```

### Using prebuilt binaries

Linux and Macos binaries are available as tarballs in
the [release section](https://github.com/wlezzar/jtab/releases/latest).

### Using cargo (all platforms)

For other platforms, you can use `cargo`:

```bash
cargo install --git https://github.com/wlezzar/jtab

jtab --help
```

## Sample commands

#### Example usage

Pipe some weather data into jtab:

```bash
➜ curl -s 'https://www.metaweather.com/api/location/search/?query=san' \
    | jtab

┌───────────────────────┬───────────────┬───────────────┬─────────┐
│ latt_long             │ location_type │ title         │ woeid   │
├───────────────────────┼───────────────┼───────────────┼─────────┤
│ 37.777119, -122.41964 │ City          │ San Francisco │ 2487956 │
├───────────────────────┼───────────────┼───────────────┼─────────┤
│ 32.715691,-117.161720 │ City          │ San Diego     │ 2487889 │
├───────────────────────┼───────────────┼───────────────┼─────────┤
│ 37.338581,-121.885567 │ City          │ San Jose      │ 2488042 │
├───────────────────────┼───────────────┼───────────────┼─────────┤
│ 29.424580,-98.494614  │ City          │ San Antonio   │ 2487796 │
├───────────────────────┼───────────────┼───────────────┼─────────┤
│ 36.974018,-122.030952 │ City          │ Santa Cruz    │ 2488853 │
└───────────────────────┴───────────────┴───────────────┴─────────┘
```

Take only 2 rows and a subset of the columns:

```bash
➜ curl -s 'https://www.metaweather.com/api/location/search/?query=san' \
    | jtab --take 2 -f title -f woeid

┌───────────────┬─────────┐
│ title         │ woeid   │
├───────────────┼─────────┤
│ San Francisco │ 2487956 │
├───────────────┼─────────┤
│ San Diego     │ 2487889 │
└───────────────┴─────────┘
```

#### Ingesting a stream of json (one json per row)

By default, `jtab` parses the full payload before printing it and expects it to be a valid JSON. To support use cases
where data is piped in streaming mode (one valid json per line), `jtab` has a `--streaming` flag for that purpose.

The example below shows how you can use the `--streaming` flag to ingest the wikipedia change stream in `jtab`:

```
➜ curl -s  https://stream.wikimedia.org/v2/stream/recentchange \
    | grep data \
    | sed 's/^data: //g' \
    | jtab --streaming --take 5 -f id -f title -f user -f type

┌────────────┬───────────────────────────────────────────────────────┬───────────────────┬────────────┐
│ id         │ title                                                 │ user              │ type       │
├────────────┼───────────────────────────────────────────────────────┼───────────────────┼────────────┤
│ 1248099852 │ Q695926                                               │ Ecchbz            │ edit       │
├────────────┼───────────────────────────────────────────────────────┼───────────────────┼────────────┤
│ 98274450   │ Bản mẫu:Số ca nhiễm COVID-19 theo tinh thành Việt Nam │ Thái Nhi          │ edit       │
├────────────┼───────────────────────────────────────────────────────┼───────────────────┼────────────┤
│ 1272589900 │ Category:Noindexed pages                              │ Materialscientist │ categorize │
├────────────┼───────────────────────────────────────────────────────┼───────────────────┼────────────┤
│ 1272589899 │ Tong Tau Po                                           │ Underwaterbuffalo │ new        │
├────────────┼───────────────────────────────────────────────────────┼───────────────────┼────────────┤
│ 1272589901 │ Category:Wikipedia sockpuppets                        │ Materialscientist │ categorize │
└────────────┴───────────────────────────────────────────────────────┴───────────────────┴────────────┘
```

#### Table formats

`jtab` support multiple table formats using the `--format` command line option:

- `default`: prints the table in a pretty formatted (to be used in the command line).
- `markdown`: outputs the table in markdown
- `html`: outputs the table in Html format styled with bootstrap
- `html-raw`: outputs the table in Html format without any styling.

For example, to print the table in HTML format:

```
➜ echo '[{"id": 1, "nested": {"value1": 1, "value2": 2}}, {"id": 2, "nested": {"array": [1, 2], "deeply_nested": {"value": 1}}}]' \
    | jtab --format html > /tmp/table.html
  
# Open the file with your favorite browser  
➜ xdg-open /tmp/table.html
```

#### Conditionally colorize fields (only supported with `--format default`)

You can conditionally colorize some fields based on their value:

```
➜  ~ curl -s  https://stream.wikimedia.org/v2/stream/recentchange \
         | grep data \
         | sed 's/^data: //g' \
         | jtab --streaming --take 5 -f id -f title -f user -f type \
                --colorize 'type:categorize:bFg' \
                --colorize 'type:edit:bFr'
```

![res](docs/img/colorize-result.png)

The `colorize` option takes a string in the format `column_name:value:style_spec`. The `style_spec` corresponds
to [the style specifiers of the prettytable-rs](https://github.com/phsym/prettytable-rs#list-of-style-specifiers)
library that jtab is based on.
