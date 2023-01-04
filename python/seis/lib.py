from seis import seis


def create_gather():
    new_gather = seis.Gather(1, 2, 3, 0.01, seis.Precision.Double)
    return new_gather
