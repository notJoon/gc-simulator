from random import random, choice
from typing import List, TypeVar

from controller import PIController
from mark_and_sweep import MarkAndSweepGC
from mem import Memory
from object import Object

T = TypeVar('T')

if __name__ == "__main__":
    threshold: int = 10

    mem: Memory = Memory()
    gc: T = MarkAndSweepGC(mem)
    pi: PIController = PIController(gc, threshold)

    roots: List[Object] = []

    root = Object("root")
    roots.append(root)

    for i in range(threshold * 2):
        new_obj: Object = Object(f"obj{i}")
        mem.add_object(new_obj)

        # Add reference to root randomly
        if choice([True, False]):
            root.add_reference(new_obj)
        else:
            root.remove_reference(new_obj)

        print("****")
        print(f"Objects: {mem.objects}")
        gced = pi.check_gc(roots)
        if gced[1]:
            print("------")
            print(gced[0])
            print(f"result: {mem.objects}")
        else:
            print(gced[0])

    print("Done")
