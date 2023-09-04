from typing import TypeVar, List, Optional
from weighted_graph import WeightedGraph
from weighted_edge import WeightedEdge
from priority_queue import PriorityQueue

V=TypeVar('V') # type of the vertices in the graph
WeightedPath=List[WeightedEdge] # type alias for paths

def total_weight(wp: WeightedPath)->float:
    return sum([e.weight for e in wp])

def mst(wg: WeightedGraph[V], start:int=0)-> Optinal[WeightedPath]:
    if start> (wg.vertex_count-1) or start <0:
        return None
    result: WeightedPath=[]
    pq:PriorityQueue[WeightedEdge]=PriorityQueue()
    visisted: [bool]=[False] * wg.vertex_count 
    def visit (index: int):
        visisted[index]=True
        for edge in wg.edges_for_index(index):
            if not visited[edge.v]:
                pq.push(edge)
    
    visit(start)
    while not pq.empty:
        edge=pq.pop()
        if visisted[edge.v]:
            continue
        result.append(edge)
        visit(edge.v)
    return result


