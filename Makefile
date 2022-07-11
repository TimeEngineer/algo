run-i:
	cargo build
	python storage/interactive_runner.py python storage/testing_tool_2022_1b.py 0 -- ./target/debug/interact

run:
	cargo build
	./target/debug/jam

simplex:
	gcc storage/simplex.c -lglpk -O3 -o test && ./test

record:
	perf record --call-graph=dwarf test

report:
	perf report --hierarchy -M intel