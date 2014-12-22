# Makefile; works for Mac OS X Yosemite with default c++ (clang++)
# -std=c++14 not available for Mac yet.
CXXFLAGS = -Wall -std=c++1y
RUSTC = rustc

all:	steve_seg_fault unique_ptr_simulation seg_fault_in_vector rc_out_of_vector unsafe_seg_fault

# illegal_move_out_of_vector will fail to type-check.
fail:	steve_illegal_move illegal_move_out_of_vector

clean:
	rm -f unique_ptr_simulation seg_fault_in_vector

# Simple Rust compilation without Cargo.
%:	%.rs
	$(RUSTC) $<

.PHONY:	all fail clean
