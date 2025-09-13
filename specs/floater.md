## floater


Floater is a desktop application that creates a small floating window that contains any content you want.

## UX

The floater window can be dragged around the screen.
It cannot be resized - it is a fixed 200x120px window.
It cannot be themed - it is a static green on white ux.


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

### Packaging

The application must have a dmg installer that can be launch from the applications folder.
The application must bundle a CLI as a bundled bun-built binary.
The application should have a non-default icon - you can use a python script to generate an svg or something and then convert it to a png.
