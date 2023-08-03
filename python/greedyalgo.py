def greedy_select(states_needed, stations):
    
    solution_found=False
    selected_stations=[]
    while solution_found==False:
    
        best_station=None
        states_covered = set()
        for station, states in stations.items():
            covered = states_needed & states
            if len(covered) > len(states_covered):
                best_station = station
                states_covered = covered
        if best_station==None:
            solution_found=True
            return "Selected Stations: "+str(selected_stations)+" Remaining States Requiring Station: "+str(states_needed)
        stations.pop(best_station)
        selected_stations.append(best_station)
        states_needed=states_needed-set(states_covered)
        if states_needed==set([]):
            solution_found=True
            break
    
    return selected_stations

    
    
states_needed = set(["mt", "wa", "or", "id", "nv", "ut", "ca", "az"])

#How you would actually do this:
state_need_ls=["mt", "wa", "or", "id", "nv", "ut", "ca", "az"]
state_need_set=set(state_need_ls)

stations = {}
stations["kone"] = set(["id", "nv", "ut"])
stations["ktwo"] = set(["wa", "id", "mt"])
stations["kthree"] = set(["or", "nv", "ca"])
stations["kfour"] = set(["nv", "ut"])
stations["kfive"] = set(["ca", "az"])

final_stations = set()

selected_stations=greedy_select(states_needed, stations)
print(str(selected_stations))

#best_station = None


