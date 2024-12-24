import base
from ch3b import NOT_EXPECTED_2b, EXPECTED_3b


EXPECTED_4b = EXPECTED_3b + [
    # ch4b_sbrk
    "Test sbrk almost OK!",
]

NOT_EXPECTED_4b = NOT_EXPECTED_2b+ [
    # ch4b_sbrk
    "Test sbrk failed!",
]

if __name__ == "__main__":
    base.test(EXPECTED_4b, NOT_EXPECTED_2b)
