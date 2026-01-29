# Shamash: Monitor your Internet connection and keep your ISP honest

Shamash is a small and lightweight tool that monitors your Internet connection for any drops in connectivity, to keep your ISP honest about the quality of service they are providing.

No data leaves your machine.

## Features:

- Logging: Shamash logs connection events, determining your uptime.
- No special privileges needed: Shamash simply reads the network activity and logs any drops.
    - Only write privileges to the current directory required.
- All data stays on your machine: Shamash logs all data to a local file. That's it.
- Detect local outages (router is down)
- Detect remote outages (ISP is down)
- See if there are any local or remote outages in real time

### Real-time:

Shamash creates a file with no contents if an outage is detected. It can be found inside the `shamash-logs` directory.

- `isp_outage` - When the ISP is down.
- `local_outage` - When the local network is down.
- `diagnosing` - While the program is diagnosing the issue. This is only visible for a few seconds at most.

This file is deleted as soon as the outage ends.

## Why use Shamash?

- Keep your ISP honest: By tracking your Internet uptime, packet loss, and latency, you can identify and document any performance issues that may be caused by your ISP. This information can be used to hold your ISP accountable and to negotiate better service.

## What data does Shamash log?

- Start and end time & date of a connection drop.
- Type of connection drop (local or remote).

## Roadmap

- [ ] Removal of all `panics`, `unwraps` or `expects`
    - Instead log the errors in an error file (`base_dir/error.log`)

## Installation:

Shamash should run on any UNIX system, it's only tested on Debian.

1. Clone the repository.
2. Change the IP found in `src/config.rs` under `ROUTER_IP` to the IP of your router.
3. Build the project.

```sh
    $ git clone https://github.com/Xqhare/shamash
    $ cd shamash
    $ cargo build -r
```

4. Move the binary to the desired location.
5. Add the binary to your autostart script.
6. Restart your system or start the program manually.

Now Shamash is up and running, monitoring your Internet connection.
Whenever the system boots up, it will automatically start Shamash in the background, as long as you have added it to your autostart script.

### Docker

You can also run Shamash in a Docker container.
The Dockerfile for Shamash is located in the root directory of the repository.

It supports the following environment variables:
- `ROUTER_IP`: The IP address of the router.
- `LOG_DIR_PATH`: The path to the directory where the logs will be saved.

## Usage:

After setting up the program, you can simply let it run in the background.
Shamash will create a directory `shamash-logs` in the same directory you place it in and store all of its data there.

The log files are saved as text inside the `shamash-logs` directory.

## How Shamash works:

Shamash is a simple program that pings a remote server every second. If the ping fails, it starts an outage and begins logging.
From now, Shamash will ping every third of a second until the connection is back up.
It then saves the log and starts pinging every second again.

It rotates through DNS targets and pings them in a round-robin manner.
Namely: `1.1.1.1`, `1.0.0.1`, `8.8.4.4`, `8.8.8.8`, `9.9.9.9`, `94.140.14.14`, `94.140.15.15`, `149.112.112.112`, `208.67.222.222`, `208.67.220.220`

## Naming:

In ancient Mesopotamian mythology, Shamash was the god of justice, law, and truth. He was tasked with upholding righteousness and ensuring that all beings were treated fairly.

## Acknowledgments
Thanks to the open-source community for providing invaluable tools and libraries.
Used in this project:
- [signal-hook](https://crates.io/crates/signal-hook)
- [horae](https://github.com/Xqhare/horae)
