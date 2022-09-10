# A-MAZE

## Development
### Dependencies
- VS Code with "Remote - Containers" extension
- Docker
### Setup
Open the project in VS Code and run the "Reopen in Container" command. This may take a while the first time as the container is built, but will give you a working, consistent environment. After the container is built, you will be dropped in to the container, but the `bootstrap.sh` script will still be running in a terminal. This will install a variety of dependencies that can't be included in the `Dockerfile` because they need to be created by the `vscode` user.

### Testing
Run the game from a terminal with `just watch`. VS Code should pop up with a link to open the game in your browser. Open it in two browsers next to each other to test multiplayer functionality.