## floater


Floater is a desktop application that creates a small floating window that contains any content you want.

## UX

The floater window can be dragged around the screen.
It cannot be resized - it is a fixed 200x120px window.
It cannot be themed - it is a static green on white ux.

cmd+q and cmd+w should both close the window and exit the application.

The CLI `show` will launch the application if not already running.
The CLI `hide` will print an error if the application is not running.

The CLI `show` will update the content of the window if it is already visible running.

Opening the application from the spotlight/alfred/raycast will open it but not show the content until the `show` command is run.

The window will be present across ALL workspaces and always float on top of all other windows.


## Usage

```
floatercli show "hello, world!"
```

```
floatercli hide
```

```
# widgets
floatercli show "hello, world!" --timer
```

## platform notes

only macos is supported

## Implementation notes

Use tauri for the desktop application.

Use built-in tauri features for everything you can. ALWAYS search the tauri docs when implementing new features.

Search the homebrew docs for homebrew-related work.

### Packaging

The application must have a dmg installer that can be launch from the applications folder.
The application must bundle a CLI as a bundled bun-built binary.
The application should have a non-default icon - you can use a python script to generate an svg or something and then convert it to a png.

The applicaiton should include a brew formula at ../homebrew-floater/ that can be used to install the desktop application as a Cask that include the floater cli as a bundled binary.
