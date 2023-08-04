# My little game engine/ graphics programming project :3
Heyo dear reader. This is my little dive into game development without a game engine using only rust and the opengl library allowing programs to run on the graphics card.

## Installation
If you want to install the current version of the project and play around with it for yourself a litle here are the steps I recommend you take to have the smoothest expierience overall:

### 1.) Install git and clone this repository
I recommend installing git and cloning this repo into a directory on your machine to quickly be able to update the project whenever I implement some new features to try out.

If you are running windows the steps to that are the following:

1. Get your head checked what the hell are you thinking using windows? :P 
    
2. Go to the following link https://git-scm.com/download/win
    
3. Download the standalone installer for 32-bit or 64-bit systems depending on what CPU-architecture you are running, execute the installer and follow the instructions of the installer. Whenever possible stick to the default options unless you know what you are doing

4. Clone this git repository. You can do this by opening a terminal and navigating into a folder that you would like to clone this project into. After you have navigated into your desired folder execute the following command:
```
    git clone https://github.com/newblorbodropped/rusty-engine.git
```

### 2.) Install the rust compiler and some other neat rust related features
Of course to install and run this project you need to install the rust compiler. There are several methods to do this, but I recommend the following:

1. Go to the following link https://rustup.rs/

2. Download the executable 'rustup-init.exe' and execute it. Follow the onscreen instructions. You may also need to install the Visual studio prerequisites, but the installer should initiate this process automatically.

### 3.) Finally install the project and run it
Congrats if you got this far :D Give yourself a pat on the back. Especially if you're new to this. Now we will be building and runnig the project.

To build the project navigate into the folder where you cloned the project to. This should be the folder where the file 'Cargo.toml' is located. Then execute the command: 

```
    cargo build --release
```

Don't worry if this takes a few moments. To run the project execute the command:

```
    cargo run -- -s scene.sce 
```

This is the most basic argument configuration right now. You can however append additional argumens to the command to give the program some additional behaviour. Execute

```
    cargo run -- -s scene.sce -f
```

to make the program run in full screen mode. Execute 

```
    cargo run -- -s scene.sce -p 1
```
or
```
    cargo run -- -s scene.sce -p 2
```
to select one of the two postprocessing shaders that I implemented thus far.
You can also combine the additional arguments in any order you like.

To move the camera you can use the WASD-keys in the common fashion and use SPACE and CTRL to move up and down
respectively. To rotate the camera you can move the mouse or alternatively use the arrow keys if your mouse is broken. To quit the program use the ESC-key.

