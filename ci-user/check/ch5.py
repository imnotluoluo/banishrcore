import base
from ch5b import EXPECTED_5b, NOT_EXPECTED_4b
from ch4 import EXPECTED_4

EXPECTED_5 = EXPECTED_4 + [
    # ch5_spawn0
    "Test spawn0 OK!",

    # ch5_spawn1
    "Test wait OK!",
    "Test waitpid OK!",

    # ch5_setprio
    "Test set_priority OK!",
]

EXPECTED_5 = list(set(EXPECTED_5) - set([
    "string from task info test",
    "Test task info OK!",
]))

if __name__ == "__main__":
    base.test(EXPECTED_5, NOT_EXPECTED_4b)