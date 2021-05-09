# 💉 CoWIN Notifier 😷
[![Open Source Love](https://badges.frapsoft.com/os/v1/open-source.svg?v=103)](https://github.com/aryan9600/cowin-notifier)
[![GitHub License](https://img.shields.io/github/license/aryan9600/cowin-notifier)](https://github.com/aryan9600/cowin-notifier/blob/master/LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/aryan9600/cowin-notifier/issues/new/choose)
![release](https://img.shields.io/github/v/release/aryan9600/cowin-notifier)
![rust-version](https://img.shields.io/badge/rust-v1.5.2-red)

A cross-platform tool written in rust, which instantly notifies users about COVID-19 vaccine availability at their regions. Currently, this tool is only compatible in India.

## Usage

### 💻 Windows
* [Download the zip file](https://github.com/aryan9600/cowin-notifier/releases/download/v0.1/cowin-notifier-windows.zip) and unzip it.
* Open the folder containing the file, do a `Shift + Right Click` and select "Open Powershell here."
* Run `./cowin-notifier "state_name" "district_name" "age_group"`
![Windows Screenshot](https://raw.githubusercontent.com/aryan9600/cowin-notifier/main/assets/WhatsApp%20Image%202021-05-06%20at%2021.23.49.jpeg)
* Minimize the Powershell window.
* If a slot opens up for your age group and district, you'll recieve a notification. __Make sure that you don't close the Powershell window.__

### 🍎 macOS
* [Download the zip file](https://github.com/aryan9600/cowin-notifier/releases/download/v0.1/cowin-notifier-macos.zip) and unzip it.
* Open your terminal and `cd` into your default Downloads directory where you have downloaded the file. macOS users can fire up Spotlight using `cmd + Space`, search for Terminal and open it.
![macOS Screenshot-1](https://github.com/mintbomb27/cowin-notifier/raw/main/assets/macOS%20spotlight.png)
* Run the following commands in the terminal.
```
 cd ~/Downloads 
chmod +x cowin-notifier
./cowin-notifier "state_name" "district_name" "age_group"
```
![macOS Screenshot-2](https://github.com/mintbomb27/cowin-notifier/raw/main/assets/macOS%20notif.png)
* Minimize your terminal.
* If a slot opens up for your age group and district, you'll recieve a notification. __Make sure that you don't close the Terminal.__

![macOS Screenshot-3](https://raw.githubusercontent.com/aryan9600/cowin-notifier/main/assets/Screen%20Shot%202021-05-05%20at%2017.46.10.png)

### 🐧 Linux
* We use the [reqwest](https://github.com/seanmonstar/reqwest) crate, which has some specific [requirements for Linux](https://github.com/seanmonstar/reqwest#requirements). Please install the required dependencies as mentioned.
* [Download the zip file](https://github.com/aryan9600/cowin-notifier/releases/download/v0.1/cowin-notifier-linux.zip) and unzip it.
* The rest of the usage is similar to macOS.

> Note: The notification functionality hasn't been tested yet on Linux. Please feel free to play around with the code and test it.
> 
> If you're stuck somewhere in between, please check [these commands](https://maker.pro/linux/tutorial/basic-linux-commands-for-beginners) out, and of course try googling!

## Contributing

Please feel free to contribute. The tool is written purely in the Rust Programming Language. The code definitely needs some cleanup and refactoring, so please open a PR if you feel up for the task. If anyone can figure out how to daemonize this (at least on POSIX) without having to write a systemd file, please go ahead and implement it.
