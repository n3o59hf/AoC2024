# AoC 2024

## Setup

### Automation 

`cargo aoc credentials {token}` to setup auto download of inputs. To get token, login in Chrome, open developer tools,
then in "Application" tab "Cookies" copy the "Value" field of the session cookie.

### Daily

Run `cargo aoc input` to download current day's input.

Run `cargo aoc input -d {day} -y {year}` to download specific day's input.

Run `cargo aoc input -g` to download current day's input and generate boilerplate.

It's possible to also add inputs manually into `/input/{year}/day{n}.txt`.

## Running

Project can be run as a binary (`cargo run`).

`cargo aoc` to run latest implemented day.

`cargo aoc -d {day} -p {part}` to run specific day and specific part.

`cargo aoc bench` to measure solution speed.
