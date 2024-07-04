## 2048

## How it works ##
The project is split into two parts, frontend and backend. The frontend handles the display of the board, and the backend does the updating of the board. The game state is managed by the server and it handles determining score, positions of numbers after key presses, and if the game is over. 

The JS in the frontend uses ```js await fetch(url, [options])``` to send and receive requests from the server. Then the browser uses the CSS and HTML format the stuff nicely.

To run the server and start the game use...
```bash	
git clone git@github.com:pgao1116/2048.git
cd 2048
cargo +nightly build && cargo +nightly run

```

## How it looks ##
A minute GIF of the project. 
![2048 ](assets/game.gif)


## Other stuff ## 
To test the server using curl...
```bash
	curl -X POST -H "Content-Type: application/json" -d '{"Key[Right, Left, etc]": null}' http://localhost:8000/keystroke
```
There is a shell script that does test the server, but it needs to be changed to use jq to print the JSON, but the above is good enough for now.

## Links
- [git merge vs git rebase](https://stackoverflow.com/questions/804115/when-do-you-use-git-rebase-instead-of-git-merge)
- [rocket-rs](https://rocket.rs/guide/v0.5/)
- [fetch in js](https://javascript.info/fetch)
