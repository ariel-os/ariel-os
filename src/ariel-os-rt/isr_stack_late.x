/* using `_tmp` helpers so this overrides other linker script variables */
_stack_lowest = _stack_lowest_tmp;
_stack_highest = _stack_highest_tmp;

/* used by xtensa. grep for `xtensa_lx::set_stack_pointer` in esp-hal repo. */
_stack_end_cpu0 = _stack_lowest_tmp;
_stack_start_cpu0 = _stack_highest_tmp;

_stack_end = _stack_lowest_tmp;
_stack_start = _stack_highest_tmp;

__stack_chk_guard = _stack_lowest + 4096;

ASSERT(_stack_start != _stack_lowest, "ERROR(ariel-os-rt): isr stack too small");
ASSERT(_stack_start == _stack_highest_tmp, "ERROR(ariel-os-rt): _stack_start not used!");
