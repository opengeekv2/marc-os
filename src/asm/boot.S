.option norvc
.section .text.init
.global _start
_start:
    csrw satp, zero
.option push
.option norelax
    la		gp, _global_pointer
.option pop
    csrr	t0, mhartid
	bnez	t0, idle
    la 		a0, _bss_start
	la		a1, _bss_end
	bgeu	a0, a1, set_stack_pointer
zero_bss:
	sd		zero, (a0)
	addi	a0, a0, 8
	bltu	a0, a1, zero_bss
set_stack_pointer:
	# The stack grows from bottom to top, so we put the stack pointer
	# to the very end of the stack range.
	la		sp, _stack_end
    li		t0, (0b11 << 11) | (1 << 7) | (1 << 3)
    csrw    mstatus, t0
    li		t3, (1 << 3) | (1 << 7) | (1 << 11)
    csrw	mie, t3
    la		t1, kmain
    csrw	mepc, t1
    la		ra, idle
    mret
idle:
    wfi
    j idle
    