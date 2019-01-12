

The de facto standard [walkdir](https://github.com/BurntSushi/walkdir) crate offers a very configurable way to walk over the files of a directory.

But this power comes with a cost.

After having improved the performances of computing directory size in [broot](https://github.com/Canop/broot) by 15% by removing walkdir, I wanted to have a clearer idea of the cost, which is why I wrote this simple bench comparing walkdir with a naive and obvious implementation for some very simple operations.

Here's the output on my computer:

	dys@dys-desktop:~/dev/walk_pace> target/release/walk_pace
	warming up
	count_files_walking. Number of files: 1685126
	count_files_running. Number of files: 1685113
	count_visible_files_walking. Number of files: 1225536
	count_visible_files_running. Number of files: 1225523
	measuring
	counting files, walking, took 2.027986542s
	counting files, running, took 1.77173462s
	counting visible files, walking, took 1.481694735s
	counting visible files, running, took 1.27163876s

The files listed by walkdir and not by my iteration are the ones in `/proc`, which looks like a bug in walkdir.

In all my tests I get about the same result of 15% overhead of WalkDir over a simple iteration.

And here are my personal conclusions:

* WalkDir does allow for a much shorter and clearer code
* WalkDir is a poor choice when performances matter and the operation involves dealing with a lot of files (at least in the cases I tested)

