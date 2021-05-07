# cowin-notifier

A cross platform tool which instantly notifies about COVID vaccine availability.

## Usage

### Windows
* [Downlaod the zip file](https://github.com/aryan9600/cowin-notifier/releases/download/v0.1/cowin-notifier-windows.zip) and unzip it.
* Open the folder containing the file, do a `Shift + Right Click` and select "Open Powershell here."
* Run `./cowin-notifier "state_name", "district_name", "age_group"`
![Alt text](https://raw.githubusercontent.com/aryan9600/cowin-notifier/main/assets/WhatsApp%20Image%202021-05-06%20at%2021.23.49.jpeg)
* Minimize the Powershell window.
* If a slot opens up for your age group and district, you'll recieve a notification. __Make sure that you don't close the Powershell window.__

### macOS
* [Download the zip file](https://github.com/aryan9600/cowin-notifier/releases/download/v0.1/cowin-notifier-macos.zip) and unzip it.
* Open your terminal and `cd` into your default Downloads directory. macOS users can fire up Spotlight(cmd+Space), search for Terminal and open it.

![Alt text](https://raw.githubusercontent.com/aryan9600/cowin-notifier/main/assets/Screen%20Shot%202021-05-05%20at%2015.59.44.png)

* Run `cd ~/Downloads`

* Run `./cowin-notifier "state_name", "district_name", "age_group"`
![Alt text](https://raw.githubusercontent.com/aryan9600/cowin-notifier/main/assets/Screen%20Shot%202021-05-05%20at%2016.06.28.png)

* Minimize your terminal.
* If a slot opens up for your age group and district, you'll recieve a notification. __Make sure that you don't close the Terminal.__

![Alt text](https://raw.githubusercontent.com/aryan9600/cowin-notifier/main/assets/Screen%20Shot%202021-05-05%20at%2017.46.10.png)

### Linux
* We use the [reqwest](https://github.com/seanmonstar/reqwest) crate, which has some specific [requirements for Linux](https://github.com/seanmonstar/reqwest#requirements). Please follow them first.
* Download the [zip file](https://github.com/aryan9600/cowin-notifier/releases/download/v0.1/cowin-notifier-linux.zip) and unzip it.
* The rest of the usage is similar to macOS.

> Note: The notification functionality hasn't been tested yet on Linux. Please feel free to play around with the code and test it.

## Contributing

Please feel free to contribute. The tool is written purely in the Rust Programming Langauge. The code definitely needs some cleanup and refactoring, so please open a PR if you feel up for the task. If anyone can figure out how to daemonize this (at least on POSIX) without having to write a systemd file, please go ahead and implement it.
