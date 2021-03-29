# .SILENT:
.DEFAULT_GOAL = help

.PHONY: help
help:
	@echo
	@echo "Recipes:"
	@echo "- help:     this help message"
	@echo "- clean:    cleans all compiled output"
	@echo
	@echo "To run programs see the main readme and each of those in the numbered directories"
	@echo

.PHONY: clean
clean:
	@echo Removing compiled output at top level
	rm -rf target

	@echo Removing compiled output from challenge directories
	@for i in 0{1..9} {10..25}; do \
		if [ -f $$i/Cargo.lock ]; then \
			rm -rf $$i/target; \
		fi; \
		for file in "*.out" solution solution2 solution_test solution2_test; do \
			rm -fv $$i/$$file; \
		done; \
	done;
