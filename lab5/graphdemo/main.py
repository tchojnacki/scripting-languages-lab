from graphlibpy import Graph, Node

g = Graph()
g.create_node("a")
n = g.add_node(Node("b"))
none = g.create_node("a")
print(none)
print(n.label)
g.create_edge("a", "b", 1.5)

for n in g.node_list:
    print(n.label)

for e in g.edge_list:
    print(e.weight)

print(g.order)
print(g.size)
print(g.is_empty)
