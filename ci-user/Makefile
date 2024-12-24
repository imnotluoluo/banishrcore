RAND := $(shell awk 'BEGIN{srand();printf("%d", 65536*rand())}')
CHAPTER ?=
OFFLINE :=
# This is only for TAs to run base test, students do not know the token
TOKEN_SHA1 := $(shell echo -n '$(passwd)' | sha1sum | xargs | awk -F' ' '{ print $$1 }')
ifeq ($(TOKEN_SHA1), 3d95c23bbc8fa7540096eb11aa094621c28f9358)
	BASE := 1
	BASE_CHAR := b
else
	CHECK_REPORTS := 1
	BASE := 2
endif

# For students, report will be counted in to pass the CI
ifeq ($(CHAPTER), 3)
	LAB := 1
else ifeq ($(CHAPTER), 4)
	LAB := 2
else ifeq ($(CHAPTER), 5)
	INITPROC := 1
	LAB := 3
else ifeq ($(CHAPTER), 6)
	INITPROC := 1
	LAB := 4
else ifeq ($(CHAPTER), 7)
# For ch7 we have no ch7_usertest, so just run ch7b_usertest to pass the CI
	INITPROC := 1
	BASE_CHAR := b
	LAB := 4
else ifeq ($(CHAPTER), 8)
	INITPROC := 1
	LAB := 5
else ifeq ($(CHAPTER), 9)
	INITPROC := 1
	LAB := 6
endif

env:
ifeq ($(OFFLINE),)
	(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add riscv64gc-unknown-none-elf
	cargo install cargo-binutils
	rustup component add rust-src
	rustup component add llvm-tools-preview
endif

backup:
# Save the original files
	@echo "Backing up original files to temp-* dirs"
	@mkdir -p ../temp-os
	@mkdir -p temp-user
	@mkdir -p temp-checker
	@make -C ../os clean
	@make -C user/ clean
	@cp -r ../os/. ../temp-os
	@cp -r user/* temp-user
	@cp -r user/.cargo temp-user
	@cp -r check/. temp-checker
	@if [ -d "../easy-fs-fuse" ]; then \
		mkdir -p ../temp-easy-fs-fuse ; \
		cd ../easy-fs-fuse && cargo clean && cd ../ci-user; \
		cp -r ../easy-fs-fuse/. ../temp-easy-fs-fuse ; \
	fi

restore:
# Restore the original files
	@echo "Restoring original files from temp-* dirs"
	@if [ -d "../temp-os" ]; then \
		rm -rf ../os/*; \
		cp -r ../temp-os/. ../os; \
		rm -rf ../temp-os; \
	fi
	
	@if [ -d "../temp-easy-fs-fuse" ]; then \
		rm -rf ../easy-fs-fuse; \
		cp -r ../temp-easy-fs-fuse/. ../easy-fs-fuse; \
		rm -rf ../temp-easy-fs-fuse; \
	fi

	@if [ -d "temp-user" ]; then \
		rm -rf user/*; \
		cp -r temp-user/. user; \
		rm -rf temp-user; \
	fi

	@if [ -d "temp-checker" ]; then \
		rm -rf check/*; \
		cp -r temp-checker/. check; \
		rm -rf temp-checker; \
	fi

randomize:
	find user/src/bin -name "*.rs" | xargs -I {} sh -c 'sed -i.bak 's/OK/OK$(RAND)/g' {} && rm -rf {}.bak'
	find user/src/bin -name "*.rs" | xargs -I {} sh -c 'sed -i.bak 's/passed/passed$(RAND)/g' {} && rm -rf {}.bak'
	find check -name "*.py" | xargs -I {} sh -c 'sed -i.bak 's/OK/OK$(RAND)/g' {} && rm -rf {}.bak'
	find check -name "*.py" | xargs -I {} sh -c 'sed -i.bak 's/passed/passed$(RAND)/g' {} && rm -rf {}.bak'

test: backup env randomize
ifneq ($(OFFLINE),)
	cp -r vendor/os-vendor ../os/vendor
	cp -r vendor/user-vendor user/vendor
	cat overwrite/os-config.toml >> ../os/.cargo/config.toml
	cat overwrite/user-config.toml >> user/.cargo/config.toml
	cp overwrite/os-Cargo.lock ../os/Cargo.lock
	cp overwrite/user-Cargo.lock user/Cargo.lock
	cp overwrite/rust-toolchain.toml ../os/
	cp overwrite/rust-toolchain.toml user/
	if [ -d "../easy-fs-fuse" ]; then \
		cp -r vendor/fuse-vendor ../easy-fs-fuse/vendor ; \
		mkdir ../easy-fs-fuse/.cargo ; \
		cp overwrite/user-config.toml ../easy-fs-fuse/.cargo/config.toml ; \
		cp overwrite/fuse-Cargo.lock ../easy-fs-fuse/Cargo.lock ; \
		cp overwrite/rust-toolchain.toml ../easy-fs-fuse/ ; \
	fi
endif
	python3 overwrite.py $(CHAPTER)
	make -C user build BASE=$(BASE) TEST=$(CHAPTER) CHAPTER=$(CHAPTER)
ifdef INITPROC
	cp -f user/build/elf/ch$(CHAPTER)$(BASE_CHAR)_usertest.elf user/build/elf/ch$(CHAPTER)b_initproc.elf
endif
	make -C ../os run OFFLINE=$(OFFLINE) | tee stdout-ch$(CHAPTER)
ifdef LAB
	python3 check/ch$(CHAPTER)$(BASE_CHAR).py < stdout-ch$(CHAPTER) || (\
		$(MAKE) restore ; \
		exit 1 ; \
	)
endif

	$(MAKE) restore

ifneq ($(and $(LAB),$(CHECK_REPORTS)),)
	@for i in $(shell seq $(LAB)); do \
		if ! [ -f ../reports/lab$$i.pdf -o -f ../reports/lab$$i.md ]; then \
			echo "Report for lab$$i needed. Add your report to reports/lab$$i.pdf or reports/lab$$i.md" ; \
			exit 1 ; \
		else \
			echo "Report for lab$$i found." ; \
		fi; \
	done
endif

.PHONY: test
