## 2048

## How it works
The project is split into two parts, frontend and backend. The frontend handles the display of the board, and the backend does the updating of the board. 

To run use 
```bash	
	 cargo +nightly build && cargo +nightly run
```


To test using curl 
```bash
	curl -X POST -H "Content-Type: application/json" -d '{"Key[Right, Left, etc]": null}' http://localhost:8000/keystroke
```

## Links
- [git merge vs git rebase](https://stackoverflow.com/questions/804115/when-do-you-use-git-rebase-instead-of-git-merge)
- [rocket-rs](https://rocket.rs/guide/v0.5/)
