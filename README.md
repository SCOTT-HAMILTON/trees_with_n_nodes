# trees_with_n_nodes

Algorithm implementation that computes all the trees with n nodes

## Building
This project is configured for Cargo
```shell_session
$ cargo build
$ cargo run
```

## Usage
```
trees_with_n_nodes [-p|--print] <N>
	N, the number of nodes per tree.
	--print, whether every possible tree should be pretty-printed as ASCII.
```


## Requirements
 - [slab_tree](https://github.com/iwburns/slab-tree) (just to pretty print trees as ASCII)
 - [trees](https://github.com/oooutlk/trees)
 - [itertools](https://docs.rs/itertools/)

## Help
This is just a little project, but feel free to fork, change, extend or correct the code.

## License
This is delivered as it is under the well known MIT License.

**References that helped**
 - [slab-tree documentation] : <https://docs.rs/slab_tree/>
 - [trees documentation] : <https://oooutlk.github.io/trees/>

[//]: # (These are reference links used in the body of this note and get stripped out when the markdown processor does its job. There is no need to format nicely because it shouldn't be seen. Thanks SO - http://stackoverflow.com/questions/4823468/store-comments-in-markdown-syntax)

   [slab-tree documentation]: <https://docs.rs/slab_tree/>
   [trees documentation]: <https://oooutlk.github.io/trees/>
