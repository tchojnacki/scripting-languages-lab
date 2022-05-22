from graphlibpy import Graph, Node

g = Graph()
n = Node("a")
g.add_node(n)
print(n.label)
n.label = 3
