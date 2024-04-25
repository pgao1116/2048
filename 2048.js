// https://www.topcoder.com/thrive/articles/fetch-api-javascript-how-to-make-get-and-post-requests

endpoint = "localhost:8080/test"
fetch(endpoint)
	.then(response => {
		console.log(response); 
	})
