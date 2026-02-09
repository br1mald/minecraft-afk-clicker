# Minecraft Mob Farm Autoclicker

A simple autoclicker built in Rust for afk mob farming in Minecraft. Only works on macOS.

## Important disclaimers

- This is intended for single player/private server use.
- Do **NOT** use this in a public server, unless you are certain it isn't against the server's Terms of Service.
- I am not responsible for any bans or issues from missuse.
- This is a learning project for Rust programming.

## Functions

After executing the file, left clicks will be simulated with a 650ms (configurable by modifying ATTACK_DELAY_MS) interval between each of them, with eating every ~20 minutes, which should be enough to keep you alive, though you can change this via the ATTACKS_BEFORE_EATING constant (you will need to figure out how many attacks correspond to the duration you want). 
Eating is achieved by simulating a number key press to switch to your food slot (you can customize this by modifying the corresponding variables), holding right click for 1.7 seconds then simulating another key press to switch back to your weapon slot. After that the left clicks will continue.

## Usage 

You will need:
- A way to run the binary
- A way to stop the program (make sure to have this before testing it, it could get out of control otherwise)

The way I do it is through a shell script, similar to *start_autoclicker.sh* and a keyboard shortcut. The shell script is run through Raycast and simply executes the binary, while the keyboard shortcut will stop the program on the next iteration of the loop. It is set to Shift + Option + S but you can always change that by modifying the callback closure.

You can do the same or find another way to reliably stop the program once it's running.

**Example**

I position myself in front of my XP farm (make sure to be close to where the XP drops can get to me), then use the Raycast command corresponding to start_autoclicker.sh, after which I can go AFK.
Whenever I want to stop it I simply hit Shift + Option + S.

## Compilation

Simply use `cargo build --release` while in the directory that contains the Cargo.toml file.
