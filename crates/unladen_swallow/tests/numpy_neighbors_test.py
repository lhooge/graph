from unladen_swallow import Graph, Ungraph


def test_out_neighbors(g: Graph):
    for n in range(g.node_count()):
        nb = g.out_neighbors(n)

        assert len(nb) == g.out_degree(n)
        assert nb.base is not None
        assert nb.tolist() == g.copy_out_neighbors(n)


def test_in_neighbors(g: Graph):
    for n in range(g.node_count()):
        nb = g.in_neighbors(n)

        assert len(nb) == g.in_degree(n)
        assert nb.base is not None
        assert nb.tolist() == g.copy_in_neighbors(n)


def test_neighbors(ug: Ungraph):
    for n in range(ug.node_count()):
        nb = ug.neighbors(n)

        assert len(nb) == ug.degree(n)
        assert nb.base is not None
        assert nb.tolist() == ug.copy_neighbors(n)


def test_neighbors_keep_alive(g: Graph):
    import numpy

    degree = g.in_degree(82)
    nb = g.in_neighbors(82)

    del g

    assert len(nb) == degree
    assert numpy.all([nb >= 0, nb < 1 << 8])
