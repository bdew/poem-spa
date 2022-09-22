# poem-spa
This crate provides an Endpoint for [poem web framework](https://github.com/poem-web/poem) that serves an SPA from static files.

Unlike **StaticFilesEndpoint** this endpoint will serve an index file for all paths that don't match an actual file.

Assets folders can be defined that will return 404 response for any requests that don't match files inside them.

## Example
```rust
Route::new()
	.nest("/",
		SPAEndpoint::new("./site", "index.html")
		.with_assets("static")
	)
```

## License 
This code is licensed under the [MIT License](https://opensource.org/licenses/MIT).
