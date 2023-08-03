def look_for_key(box):
    for item in box:
        if item_is_a_box(item):
            look_for_key(item)
        elif item_is_a_key(item):
            print("found the key!")
