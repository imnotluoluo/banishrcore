import base
from ch6b import EXPECTED_6b, NOT_EXPECTED_4b

EXPECTED_8b = EXPECTED_6b + [
    # ch7b_pipetest
    "pipetest passed!",

    # ch8b_mpsc_sem
    "mpsc_sem passed!",

    # ch8b_phil_din_mutex
    "philosopher dining problem with mutex test passed!",

    # ch8b_race_adder_mutex_spin
    "race adder using spin mutex test passed!",

    # ch8b_sync_sem
    "sync_sem passed!",

    # ch8b_test_condvar
    "test_condvar passed!",

    # ch8b_threads_arg
    "threads with arg test passed!",

    # ch8b_threads
    "threads test passed!",
]

EXPECTED_8b = list(set(EXPECTED_8b) - set(["Test sbrk almost OK!"]))

if __name__ == "__main__":
    base.test(EXPECTED_8b, NOT_EXPECTED_4b)
