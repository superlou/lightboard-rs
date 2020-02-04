# Lightboard-rs

An open-source DMX lighting controller. See the [demo](https://youtu.be/TAGh97YsDVg).

![Screenshot](https://raw.githubusercontent.com/superlou/lightboard-rs/master/screenshot.png)

## Running
Currently this doesn't support anything well, however it works on my Linux Mint system with a [low-cost RS-485/DMX USB dongle](https://amzn.to/36YWtbC). In order to allow the logged-in user to access the serial device you need to add the user to the `dialout` group, then log out and back in:

`sudo adduser <username> dialout`

## Configuration
Who needs a GUI when there's text-files. This is partially intentional: it would be great to be able to diff setups when someone has "helpfully" tweaked a setting and your show no longer works. It's also a lack of GUI programming. There's probably a happy medium where the configuration files are still human-readable but also settable from the GUI for repetitve tasks.

The configuration files are:

* installation.toml - Defines the fixed aspects of your venue with lighting fixutures
* scenes.toml - Defines various scenes you can mix together
* patterns/xyz.lua - Lua scripts that control animated patterns like strobes and chases
* fixtures/xyz.toml - Defines each hardware fixture made up of elements with information on DMX channels
