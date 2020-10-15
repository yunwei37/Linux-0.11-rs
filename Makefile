run:
	@make -C lab run

clean:
	@make -C lab clean

fmt:
	@cd lab && cargo fmt