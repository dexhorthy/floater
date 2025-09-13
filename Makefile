.PHONY: dev build package
dev:
	cd floater && bun run tauri dev

build:
	cd floater && bun run build:release

package: build
