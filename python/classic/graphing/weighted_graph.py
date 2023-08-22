from typing import TypeVar, Generic, List, Tuple
from graph import Graph
from weighted_edge import WeightedEdge

V=TypeVar('V')

class WeightedGraph(Generic[V], Graph[V]):
    def __init__(self, vertices:List[V]=[])->None:
        self._vertices:List[V]=vertices
        self._edges:List[List[WeightedEdge]]=[[] for _ in vertices]

    def add_edge_by_indices(self, u: int, v:int, weight:float)-> None:
        edge: WeightedEdge=WeightedEdge(u,v,weight)
        self.add_edge(edge)
    def add_edge_by_vertives(self, first:V, second:V, weight:float)-> None:
        u:int=self._vertices.index(first)
        v:int=self._vertices.index(second)
        self.add_edge_by_indices(u,v,weight)

    def neighbors_for_index_with_weights(self, index:int)-> List[Tuple[V, float]]:
        distance_tuples: List[Tuple[V,float]]=[]
        for edge in self.edges_for_index(index):
            distance_tuples.append((self.vertex_at(edge.v), edge.weight))
        return distance_tuples
    def __str__(self)-> str:
        desc: str=""
        for i in range(self.vertex_count):
            desc+=f"{self.vertex_at(i)} -> {self.neighbors_for_index_with_weights(i)}\n"
        return desc

if __name__=="__main__":
    city_graph: WeightedGraph[str]=WeightedGraph(["Seattle","San Francisco","Los Angeles","Riverside","Phoenix","Chicago","Boston","New York","Atlanta","Miami","Dallas","Houston","Detroit","Philadelphia","Washington"])
    city_graph.add_edge_by_vertives("Seattle","Chicago",1737)
    city_graph.add_edge_by_vertives("Seattle","San Francisco",678)
    city_graph.add_edge_by_vertives("San Francisco","Riverside",386)
    city_graph.add_edge_by_vertives("San Francisco","Los Angeles",348)
    city_graph.add_edge_by_vertives("Los Angeles","Riverside",50)
    city_graph.add_edge_by_vertives("Los Angeles","Phoenix",357)
    city_graph.add_edge_by_vertives("Riverside","Phoenix",307)
    city_graph.add_edge_by_vertives("Riverside","Chicago",1704)
    city_graph.add_edge_by_vertives("Phoenix","Dallas",887)
    city_graph.add_edge_by_vertives("Phoenix","Houston",1015)
    city_graph.add_edge_by_vertives("Dallas","Chicago",805)
    city_graph.add_edge_by_vertives("Dallas","Atlanta",721)
    city_graph.add_edge_by_vertives("Dallas","Houston",225)
    city_graph.add_edge_by_vertives("Houston","Atlanta",702)
    city_graph.add_edge_by_vertives("Houston","Miami",968)
    city_graph.add_edge_by_vertives("Atlanta","Chicago",588)
    city_graph.add_edge_by_vertives("Atlanta","Washington",543)
    city_graph.add_edge_by_vertives("Atlanta","Miami",604)
    city_graph.add_edge_by_vertives("Miami","Washington",923)
    city_graph.add_edge_by_vertives("Chicago","Detroit",238)
    city_graph.add_edge_by_vertives("Detroit","Boston",613)
    city_graph.add_edge_by_vertives("Detroit","Washington",396)
    city_graph.add_edge_by_vertives("Detroit","New York",482)
    city_graph.add_edge_by_vertives("Boston","New York",190)
    city_graph.add_edge_by_vertives("New York","Philadelphia",81)
    city_graph.add_edge_by_vertives("Philadelphia","Washington",123)

    print(city_graph)

