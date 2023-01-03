import seis


def my_first_function():
    test = seis.seis.sum_as_string(2, 2)
    return test


def create_gather():
    new_gather = seis.seis.Gather(1, 2, 3)
    return new_gather
