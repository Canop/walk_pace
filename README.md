

The standard walkdir crate offers a very configurable way to walk over the files of a directory.

But this power comes with a cost.

After having improved the performances of [broot](https://github.com/Canop/broot) by removing walkdir, I wanted to have a clearer idea of the cost, which is why I wrote this simple bench comparing walkdir with a naive and obvious implementation for some very simple operations.

Here's the output on my computer:

	dys@dys-desktop:~/dev/walk_pace> target/release/walk_pace
	warming up
	count_files_walking. Result=1685126
	count_files_running. Result=1685113
	count_visible_files_walking. Result=1225536
	count_visible_files_running. Result=1225523
	measuring
	counting files, walking, took 2.027986542s
	counting files, running, took 1.77173462s
	counting visible files, walking, took 1.481694735s
	counting visible files, running, took 1.27163876s


And here are my personal conclusions:

* WalkDir does allow for a much shorter and clearer code
* WalkDir can't really be used when performances matter and the operation involves dealing with a lot of files
