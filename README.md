# Shamash: Monitor your Internet connection and keep your ISP honest

Shamash is a small and lightweight tool that monitors your Internet connection for any drops in connectivity, to keep your ISP honest about the quality of service they are providing.
No data leaves your machine, and only the date, time and duration of a connectivity drop are logged.

## Features:

- Logging: Shamash logs connection events, determining your uptime.
- No special privileges needed: Shamash simply reads the network activity and logs any drops.
- All data stays on your machine: Shamash logs all data to a local file.

## Why use Shamash?

- Keep your ISP honest: By tracking your Internet uptime, packet loss, and latency, you can identify and document any performance issues that may be caused by your ISP. This information can be used to hold your ISP accountable and to negotiate better service.

## What data does Shamash log?

- Start and end date of a connection drop.
- Start and end time of a connection drop.
- Duration of a connection drop.

## Installation:

WIP

## Usage:

WIP

## Naming:

The name "Shamash" is particularly fitting for this program given its role in monitoring and evaluating Internet connection performance. In ancient Mesopotamian mythology, Shamash was the god of justice, law, and truth. He was tasked with upholding righteousness and ensuring that all beings were treated fairly. This association with fairness and accountability aligns well with the purpose of Shamash, which is to provide users with comprehensive insights into their Internet connection and hold ISPs accountable for their service quality. Just as Shamash served as a divine arbiter, Shamash the program acts as an impartial observer of network performance, shedding light on any inconsistencies or potential issues. By providing users with accurate and detailed data, Shamash empowers them to make informed decisions about their Internet service and hold their ISPs to a higher standard of performance.

## Acknowledgments
Thanks to the open-source community for providing invaluable tools and libraries.
Used in this project:
- [signal-hook](https://crates.io/crates/signal-hook)
- [sysinfo](https://crates.io/crates/sysinfo)
- [nabu](https://github.com/Xqhare/nabu)
